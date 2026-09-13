//! Worker-side command drain (hidden-window main thread).
//! This is the composition point for cycling, context-safe spatial filtering,
//! and window arrangement. It owns no policy of its own: every decision comes
//! from a frozen contract.

use shared::{Command, Config};

use crate::arrangement::win32::{apply_plan, resolve_context, Win32WindowMover};
use crate::arrangement::{monitor, snap, stack, thirds, PlacementPlan, PlanError};
use crate::context::spatial::{enumerate_monitors, index_of_window_monitor, Win32Monitors};
use crate::context::virtual_desktop::VirtualDesktopManager;
use crate::context::{
    capture_spatial_context, collect_spatial_facts, evaluate_spatial, MonitorSource, SpatialScope,
    VirtualDesktopSource,
};
use crate::cycling::activation::Win32Activator;
use crate::cycling::eligibility::WindowEligibility;
use crate::cycling::source::{capture_active_context, Win32CandidateSource};
use crate::cycling::{
    ActivationOutcome, Activator, ActiveContext, Candidate, CandidateSource, CycleOutcome,
    EligibilityPolicy, WindowId,
};
use crate::ring;
use crate::util::debug_log;

/// `VK_NONAME` — reserved and unassigned, so nothing acts on it.
const VK_NONAME: u16 = 0xFC;
const VK_LWIN: i32 = 0x5B;
const VK_RWIN: i32 = 0x5C;

/// Stop the shell from treating a still-held Win key as a lone press.
/// Wira Desk swallows the main key of the shortcut, so the shell would otherwise
/// see Win-down followed by Win-up with nothing between, and open Start. The
/// original fix swallowed the Win key-up instead, which left the focused
/// application believing Win was still held — every later keystroke became
/// Win+key, the sticky-modifier bug.
/// Injecting one unassigned key while Win is down makes the press a
/// combination, so the real key-up can pass through untouched.
/// **This runs on the Worker, never in the hook callback.** Calling `SendInput`
/// from inside the low-level hook raced the activation happening here and
/// stopped cycling from moving focus at all.
fn suppress_start_menu() {
    use windows_sys::Win32::UI::Input::KeyboardAndMouse::{
        GetAsyncKeyState, SendInput, INPUT, INPUT_KEYBOARD, KEYBDINPUT, KEYEVENTF_KEYUP,
    };
    // SAFETY: `GetAsyncKeyState` takes no pointers and is callable from any thread.
    //
    // `zeroed` is valid for `[INPUT; 2]` because `INPUT` is a tag plus a union of
    // `MOUSEINPUT`/`KEYBDINPUT`/`HARDWAREINPUT`, all of which are plain integer structs, so
    // no bit pattern is invalid. The invariant that matters is tag/arm agreement: `r#type`
    // is set to `INPUT_KEYBOARD` and the arm written is `Anonymous.ki`, so `SendInput` reads
    // the same arm we initialised. Writing `ki` while claiming `INPUT_MOUSE` would have it
    // reinterpret those bytes as a different struct.
    //
    // `inputs.as_ptr()` is an array of exactly `inputs.len()` elements, and the third
    // argument is `size_of::<INPUT>()` — the stride Windows uses to walk the array, so a
    // mismatch there would make it read past the end. Both derive from the same type.
    //
    // Thread context is a precondition too, not just a design note: this must run on the
    // Worker. Calling `SendInput` from inside the low-level keyboard hook re-enters input
    // processing and raced the activation this function follows, which is what stopped
    // cycling from moving focus at all.
    unsafe {
        // Only if the user is still holding Win — otherwise the chord is over
        // and an injected key would be a stray keystroke.
        let held = (GetAsyncKeyState(VK_LWIN) as u16 & 0x8000) != 0
            || (GetAsyncKeyState(VK_RWIN) as u16 & 0x8000) != 0;
        if !held {
            return;
        }

        let mut inputs: [INPUT; 2] = std::mem::zeroed();
        for (i, input) in inputs.iter_mut().enumerate() {
            input.r#type = INPUT_KEYBOARD;
            input.Anonymous.ki = KEYBDINPUT {
                wVk: VK_NONAME,
                wScan: 0,
                dwFlags: if i == 1 { KEYEVENTF_KEYUP } else { 0 },
                time: 0,
                dwExtraInfo: 0,
            };
        }
        SendInput(
            inputs.len() as u32,
            inputs.as_ptr(),
            std::mem::size_of::<INPUT>() as i32,
        );
    }
}

/// Drain the Hook→Worker ring until empty.
pub fn drain_commands() {
    while let Some(raw) = ring::pop() {
        #[cfg(debug_assertions)]
        crate::metrics::DRAINED.fetch_add(1, std::sync::atomic::Ordering::Relaxed);

        match Command::from_u8(raw) {
            Command::Nop => {}
            Command::Cycle => {
                #[cfg(debug_assertions)]
                crate::util::append_debug_trace("WORKER_DRAIN: cycle=1");
                let mods = crate::hook::get_last_cycle_mods();
                execute_cycle(Some(mods), crate::cycling::Direction::Forward);
            }
            Command::CyclePrev => {
                #[cfg(debug_assertions)]
                crate::util::append_debug_trace("WORKER_DRAIN: cycle_prev=1");
                let mods = crate::hook::get_last_cycle_mods();
                execute_cycle(Some(mods), crate::cycling::Direction::Backward);
            }
            Command::SnapLeft
            | Command::SnapRight
            | Command::SnapTop
            | Command::SnapBottom
            | Command::SnapMaximize
            | Command::SnapPercentLeft
            | Command::SnapPercentRight
            | Command::SnapPercentTop
            | Command::SnapPercentBottom
            | Command::SnapThirdLeft
            | Command::SnapThirdMiddle
            | Command::SnapThirdRight => {
                execute_snap(Command::from_u8(raw));
            }
            Command::OverlappingStack => execute_stack(),
            Command::MoveToNextMonitor => execute_monitor_move(),
            Command::NextVirtualDesktop
            | Command::PrevVirtualDesktop
            | Command::TaskView
            | Command::ShowDesktop => {
                execute_mouse_navigation(Command::from_u8(raw));
            }
            Command::SwitcherArm
            | Command::SwitcherArmPrev
            | Command::SwitcherDisarm
            | Command::SwitcherNext
            | Command::SwitcherPrev
            | Command::SwitcherUp
            | Command::SwitcherDown
            | Command::SwitcherCommit
            | Command::SwitcherCancel => {
                execute_switcher(Command::from_u8(raw));
            }
        }
    }
}

pub const TIMER_SWITCHER_HOLD: usize = 101;
pub const TIMER_SWITCHER_WATCHDOG: usize = 102;

thread_local! {
    static WORKER_HWND: std::cell::Cell<Option<windows_sys::Win32::Foundation::HWND>> =
        const { std::cell::Cell::new(None) };
    static SWITCHER: std::cell::RefCell<crate::switcher::SwitcherController> =
        std::cell::RefCell::new(crate::switcher::SwitcherController::new());
    static SWITCHER_HOLD_ORIGIN: std::cell::Cell<Option<WindowId>> =
        const { std::cell::Cell::new(None) };
    static SWITCHER_CHORD_MODS: std::cell::Cell<Option<crate::hook::ModifierState>> =
        const { std::cell::Cell::new(None) };
    static SWITCHER_BACKWARD_ENTRY: std::cell::Cell<bool> =
        const { std::cell::Cell::new(false) };
}

pub fn set_worker_hwnd(hwnd: windows_sys::Win32::Foundation::HWND) {
    WORKER_HWND.set(Some(hwnd));
}

pub fn worker_hwnd() -> Option<windows_sys::Win32::Foundation::HWND> {
    WORKER_HWND.get()
}

fn are_chord_modifiers_down(mods: &crate::hook::ModifierState) -> bool {
    use windows_sys::Win32::UI::Input::KeyboardAndMouse::{
        GetAsyncKeyState, VK_LCONTROL, VK_LMENU, VK_LSHIFT, VK_LWIN, VK_RCONTROL, VK_RMENU,
        VK_RSHIFT, VK_RWIN,
    };
    // SAFETY: `GetAsyncKeyState` queries asynchronous physical key state without pointers or heap allocation.
    unsafe {
        let win_down = ((GetAsyncKeyState(VK_LWIN as i32) as u16 & 0x8000) != 0)
            || ((GetAsyncKeyState(VK_RWIN as i32) as u16 & 0x8000) != 0);
        let ctrl_down = ((GetAsyncKeyState(VK_LCONTROL as i32) as u16 & 0x8000) != 0)
            || ((GetAsyncKeyState(VK_RCONTROL as i32) as u16 & 0x8000) != 0);
        let alt_down = ((GetAsyncKeyState(VK_LMENU as i32) as u16 & 0x8000) != 0)
            || ((GetAsyncKeyState(VK_RMENU as i32) as u16 & 0x8000) != 0);
        let shift_down = ((GetAsyncKeyState(VK_LSHIFT as i32) as u16 & 0x8000) != 0)
            || ((GetAsyncKeyState(VK_RSHIFT as i32) as u16 & 0x8000) != 0);

        (!mods.win || win_down)
            && (!mods.ctrl || ctrl_down)
            && (!mods.alt || alt_down)
            && (!mods.shift || shift_down)
            && (mods.win || mods.ctrl || mods.alt || mods.shift)
    }
}

fn are_any_modifiers_down() -> bool {
    use windows_sys::Win32::UI::Input::KeyboardAndMouse::{
        GetAsyncKeyState, VK_LCONTROL, VK_LMENU, VK_LSHIFT, VK_LWIN, VK_RCONTROL, VK_RMENU,
        VK_RSHIFT, VK_RWIN,
    };
    // SAFETY: `GetAsyncKeyState` queries asynchronous physical key state without pointers or heap allocation.
    unsafe {
        ((GetAsyncKeyState(VK_LWIN as i32) as u16 & 0x8000) != 0)
            || ((GetAsyncKeyState(VK_RWIN as i32) as u16 & 0x8000) != 0)
            || ((GetAsyncKeyState(VK_LCONTROL as i32) as u16 & 0x8000) != 0)
            || ((GetAsyncKeyState(VK_RCONTROL as i32) as u16 & 0x8000) != 0)
            || ((GetAsyncKeyState(VK_LMENU as i32) as u16 & 0x8000) != 0)
            || ((GetAsyncKeyState(VK_RMENU as i32) as u16 & 0x8000) != 0)
            || ((GetAsyncKeyState(VK_LSHIFT as i32) as u16 & 0x8000) != 0)
            || ((GetAsyncKeyState(VK_RSHIFT as i32) as u16 & 0x8000) != 0)
    }
}

pub fn handle_timer(hwnd: windows_sys::Win32::Foundation::HWND, timer_id: usize) {
    match timer_id {
        TIMER_SWITCHER_HOLD => {
            // SAFETY: `KillTimer` is called with the valid worker window handle and timer id.
            unsafe {
                windows_sys::Win32::UI::WindowsAndMessaging::KillTimer(hwnd, TIMER_SWITCHER_HOLD);
            }
            if !worker_snapshot().visual_enabled {
                SWITCHER_HOLD_ORIGIN.set(None);
                SWITCHER_CHORD_MODS.set(None);
                return;
            }
            let chord_mods = SWITCHER_CHORD_MODS.get().unwrap_or_default();
            if are_chord_modifiers_down(&chord_mods) {
                open_visual_switcher(hwnd);
            }
        }
        TIMER_SWITCHER_WATCHDOG => {
            // If all modifiers are released, commit immediately
            if !are_any_modifiers_down() {
                execute_switcher(Command::SwitcherCommit);
                return;
            }

            let is_expired = SWITCHER.with(|s| {
                let sw = s.borrow();
                sw.is_open() && crate::hook::tick_ms().saturating_sub(sw.open_time_ms()) > 10_000
            });
            if is_expired {
                execute_switcher(Command::SwitcherCancel);
            }
        }
        _ => {}
    }
}

fn open_visual_switcher(hwnd: windows_sys::Win32::Foundation::HWND) {
    let active = capture_active_context();
    let monitors = Win32Monitors;
    let spatial = capture_spatial_context(&monitors, active.foreground);

    let (candidates, eligible) = with_virtual_desktops(|desktops| {
        collect_eligible_candidates(
            &Win32CandidateSource,
            &WindowEligibility,
            &active,
            &monitors,
            desktops,
            &spatial,
            SpatialScope::AnyMonitorOnCurrentDesktop,
        )
    });

    if eligible.is_empty() {
        return;
    }

    let ordered = crate::switcher::card_order_for_candidates(&candidates, &active);
    let eligible_ordered: Vec<WindowId> = ordered
        .into_iter()
        .filter(|w| eligible.contains(w))
        .collect();

    if eligible_ordered.is_empty() {
        return;
    }

    let origin = SWITCHER_HOLD_ORIGIN.get().unwrap_or(active.foreground);
    let work_area = if let Some(ctx) = crate::arrangement::win32::resolve_context_for(
        origin.0 as windows_sys::Win32::Foundation::HWND,
    ) {
        crate::switcher::layout::Rect::new(
            ctx.work_area.rect.left,
            ctx.work_area.rect.top,
            ctx.work_area.rect.width(),
            ctx.work_area.rect.height(),
        )
    } else {
        // SAFETY: GetSystemMetrics reads primary screen bounds without pointers.
        crate::switcher::layout::Rect::new(
            0,
            0,
            unsafe { windows_sys::Win32::UI::WindowsAndMessaging::GetSystemMetrics(0) },
            unsafe { windows_sys::Win32::UI::WindowsAndMessaging::GetSystemMetrics(1) },
        )
    };

    let (aspects, dpi) = if let Some(ctx) = crate::arrangement::win32::resolve_context_for(
        origin.0 as windows_sys::Win32::Foundation::HWND,
    ) {
        let asps: Vec<f32> = eligible_ordered
            .iter()
            .map(|&w| {
                if let Some(r) = current_window_rect(w.0) {
                    crate::switcher::layout::aspect_from_rect(Some(
                        crate::switcher::layout::Rect::new(r.left, r.top, r.width(), r.height()),
                    ))
                } else {
                    crate::switcher::layout::DEFAULT_ASPECT
                }
            })
            .collect();
        (asps, ctx.work_area.dpi)
    } else {
        (
            vec![crate::switcher::layout::DEFAULT_ASPECT; eligible_ordered.len()],
            96,
        )
    };

    let is_backward = SWITCHER_BACKWARD_ENTRY.get();
    let initial_index = if is_backward && !eligible_ordered.is_empty() {
        eligible_ordered.len() - 1
    } else {
        0
    };

    SWITCHER.with(|s| {
        s.borrow_mut().open(
            origin,
            work_area,
            eligible_ordered,
            &aspects,
            dpi,
            initial_index,
        );
    });
    crate::hook::set_switcher_active(true);

    // SAFETY: SetTimer initializes the watchdog timer on the worker window.
    unsafe {
        windows_sys::Win32::UI::WindowsAndMessaging::SetTimer(
            hwnd,
            TIMER_SWITCHER_WATCHDOG,
            30,
            None,
        );
    }
}

fn execute_switcher(command: Command) {
    match command {
        Command::SwitcherArm | Command::SwitcherArmPrev => {
            let snap = worker_snapshot();
            let mods = crate::hook::get_last_cycle_mods();
            let delay = decide_switcher_hold_delay(
                &CycleOutcome::Activated(WindowId(0)),
                Some(mods),
                snap.visual_enabled,
                snap.visual_hold_delay_ms,
            );
            if let Some(delay) = delay {
                let active = capture_active_context();
                SWITCHER_HOLD_ORIGIN.set(Some(active.foreground));
                let mut chord_mods = mods;
                chord_mods.shift = false;
                SWITCHER_CHORD_MODS.set(Some(chord_mods));
                SWITCHER_BACKWARD_ENTRY.set(command == Command::SwitcherArmPrev);
                if let Some(hwnd) = WORKER_HWND.get() {
                    // SAFETY: SetTimer initializes the hold timer on worker window.
                    unsafe {
                        windows_sys::Win32::UI::WindowsAndMessaging::SetTimer(
                            hwnd,
                            TIMER_SWITCHER_HOLD,
                            delay,
                            None,
                        );
                    }
                }
            }
        }
        Command::SwitcherDisarm => {
            crate::hook::set_switcher_active(false);
            if let Some(hwnd) = WORKER_HWND.get() {
                // SAFETY: KillTimer disarms the hold timer.
                unsafe {
                    windows_sys::Win32::UI::WindowsAndMessaging::KillTimer(
                        hwnd,
                        TIMER_SWITCHER_HOLD,
                    );
                }
            }
            SWITCHER_HOLD_ORIGIN.set(None);
            SWITCHER_BACKWARD_ENTRY.set(false);
        }
        Command::SwitcherNext => {
            SWITCHER.with(|s| s.borrow_mut().next());
        }
        Command::SwitcherPrev => {
            SWITCHER.with(|s| s.borrow_mut().prev());
        }
        Command::SwitcherUp => {
            SWITCHER.with(|s| s.borrow_mut().up());
        }
        Command::SwitcherDown => {
            SWITCHER.with(|s| s.borrow_mut().down());
        }
        Command::SwitcherCommit => {
            crate::hook::set_switcher_active(false);
            if let Some(hwnd) = WORKER_HWND.get() {
                // SAFETY: KillTimer kills both switcher timers.
                unsafe {
                    windows_sys::Win32::UI::WindowsAndMessaging::KillTimer(
                        hwnd,
                        TIMER_SWITCHER_HOLD,
                    );
                    windows_sys::Win32::UI::WindowsAndMessaging::KillTimer(
                        hwnd,
                        TIMER_SWITCHER_WATCHDOG,
                    );
                }
            }
            let targets = SWITCHER.with(|s| {
                let mut sw = s.borrow_mut();
                let list = sw.candidates_from_selection();
                sw.dismiss();
                list
            });

            if !targets.is_empty() {
                let mut activator = Win32Activator;
                for target in targets {
                    let outcome = activator.activate(target);
                    if outcome == ActivationOutcome::Activated {
                        break;
                    }
                }
                suppress_start_menu();
            }
            SWITCHER_HOLD_ORIGIN.set(None);
            SWITCHER_BACKWARD_ENTRY.set(false);
        }
        Command::SwitcherCancel => {
            crate::hook::set_switcher_active(false);
            if let Some(hwnd) = WORKER_HWND.get() {
                // SAFETY: KillTimer kills both switcher timers.
                unsafe {
                    windows_sys::Win32::UI::WindowsAndMessaging::KillTimer(
                        hwnd,
                        TIMER_SWITCHER_HOLD,
                    );
                    windows_sys::Win32::UI::WindowsAndMessaging::KillTimer(
                        hwnd,
                        TIMER_SWITCHER_WATCHDOG,
                    );
                }
            }
            let origin = SWITCHER.with(|s| {
                let mut sw = s.borrow_mut();
                let o = sw.origin_window();
                sw.dismiss();
                o
            });

            if origin.0 != 0 {
                let mut activator = Win32Activator;
                let _ = activator.activate(origin);
            }
            SWITCHER_HOLD_ORIGIN.set(None);
            SWITCHER_BACKWARD_ENTRY.set(false);
        }
        _ => {}
    }
}

fn execute_mouse_navigation(command: Command) {
    use windows_sys::Win32::UI::Input::KeyboardAndMouse::{
        VK_LCONTROL, VK_LEFT, VK_LWIN, VK_RIGHT, VK_TAB,
    };
    let keys: &[u16] = match command {
        Command::NextVirtualDesktop => &[VK_LCONTROL, VK_LWIN, VK_RIGHT],
        Command::PrevVirtualDesktop => &[VK_LCONTROL, VK_LWIN, VK_LEFT],
        Command::TaskView => &[VK_LWIN, VK_TAB],
        Command::ShowDesktop => &[VK_LWIN, 0x44 /* 'D' */],
        _ => return,
    };
    synthesize_chord(keys);
}

fn synthesize_chord(keys: &[u16]) {
    use windows_sys::Win32::UI::Input::KeyboardAndMouse::{
        SendInput, INPUT, INPUT_KEYBOARD, KEYBDINPUT, KEYEVENTF_KEYUP,
    };

    if keys.is_empty() {
        return;
    }

    let mut inputs: Vec<INPUT> = Vec::with_capacity(keys.len() * 2);

    for &vk in keys {
        // SAFETY: `zeroed` is valid for `INPUT` as it is a union of plain integer structs.
        let mut input: INPUT = unsafe { std::mem::zeroed() };
        input.r#type = INPUT_KEYBOARD;
        input.Anonymous.ki = KEYBDINPUT {
            wVk: vk,
            wScan: 0,
            dwFlags: 0,
            time: 0,
            dwExtraInfo: 0,
        };
        inputs.push(input);
    }

    for &vk in keys.iter().rev() {
        // SAFETY: `zeroed` is valid for `INPUT` as it is a union of plain integer structs.
        let mut input: INPUT = unsafe { std::mem::zeroed() };
        input.r#type = INPUT_KEYBOARD;
        input.Anonymous.ki = KEYBDINPUT {
            wVk: vk,
            wScan: 0,
            dwFlags: KEYEVENTF_KEYUP,
            time: 0,
            dwExtraInfo: 0,
        };
        inputs.push(input);
    }

    // SAFETY: `inputs.as_ptr()` points to a contiguous slice of `INPUT` items with length `inputs.len()`,
    // matching the stride `size_of::<INPUT>()`. The buffer outlives the call.
    unsafe {
        SendInput(
            inputs.len() as u32,
            inputs.as_ptr(),
            std::mem::size_of::<INPUT>() as i32,
        );
    }

    suppress_start_menu();
}

pub fn switcher_hold_delay_ms(configured_ms: u32) -> u32 {
    shared::config::SwitcherConfig::clamp_hold_delay(configured_ms)
}

/// Decide whether to arm the visual switcher hold timer and with what delay.
///
/// Pure, Win32-free decision function extracted for testability (SPEC-14-05).
/// Returns `Some(delay_ms)` clamped to `100..=500` if the cycle activated a target,
/// modifier keys were down, and visual switcher is enabled. Returns `None` otherwise.
pub fn decide_switcher_hold_delay(
    outcome: &CycleOutcome,
    mods: Option<crate::hook::ModifierState>,
    visual_enabled: bool,
    visual_hold_delay_ms: u32,
) -> Option<u32> {
    if !visual_enabled {
        return None;
    }
    match outcome {
        CycleOutcome::Activated(_) => {
            let m = mods?;
            if m.any() {
                Some(switcher_hold_delay_ms(visual_hold_delay_ms))
            } else {
                None
            }
        }
        _ => None,
    }
}

// ── Context-safe cycling ────────────────────────────────────────────

/// One context-safe cycle pass.
/// The active context and the origin monitor are each sampled **once** and
/// carried through the whole pass, so the result stays deterministic while
/// windows open, close, and move.
fn execute_cycle(_mods: Option<crate::hook::ModifierState>, direction: crate::cycling::Direction) {
    #[cfg(debug_assertions)]
    let started = crate::metrics::qpc_now();

    let active = capture_active_context();
    let monitors = Win32Monitors;
    let spatial = capture_spatial_context(&monitors, active.foreground);

    // The interface is created once and reused. Creating it per command cost
    // ~19 ms — `CoInitializeEx` + `CoCreateInstance` on every keystroke — which
    // alone put the original cycling latency target out of reach. COM
    // ownership must stay on the Worker thread with an explicit lifetime; it
    // does not require per-command construction.
    let outcome = with_virtual_desktops(|desktops| {
        run_context_safe_cycle(
            &Win32CandidateSource,
            &WindowEligibility,
            &mut Win32Activator,
            &active,
            &monitors,
            desktops,
            &spatial,
            direction,
        )
    });

    // After the focus change, while Win is still down. Doing it here rather
    // than in the callback keeps `SendInput` off the input-processing path.
    suppress_start_menu();

    // The outcome is recorded with the sample so the durable trace can later be
    // filtered to the activating cycles the latency metric actually governs.
    #[cfg(debug_assertions)]
    crate::metrics::record_cycle(
        started,
        match &outcome {
            CycleOutcome::Activated(_) => "activated",
            CycleOutcome::Exhausted => "exhausted",
            CycleOutcome::NoEligibleTarget => "no_target",
        },
    );

    #[cfg(debug_assertions)]
    {
        use std::sync::atomic::Ordering;
        match &outcome {
            CycleOutcome::Activated(target) => {
                crate::metrics::ACTIVATED.fetch_add(1, Ordering::Relaxed);
                crate::util::append_debug_trace(&format!("WORKER_CYCLE: activated={}", target.0));
            }
            CycleOutcome::Exhausted => {
                crate::metrics::EXHAUSTED.fetch_add(1, Ordering::Relaxed);
                crate::util::append_debug_trace("WORKER_CYCLE: exhausted=1");
            }
            CycleOutcome::NoEligibleTarget => {
                crate::metrics::NO_TARGET.fetch_add(1, Ordering::Relaxed);
                crate::util::append_debug_trace("WORKER_CYCLE: no_target=1");
            }
        }
    }

    let _ = outcome;
}

/// Collect candidates and eligible windows according to policy and spatial context.
pub(crate) fn collect_eligible_candidates<S, P, M, V>(
    source: &S,
    policy: &P,
    active: &ActiveContext,
    monitors: &M,
    desktops: Option<&V>,
    spatial: &crate::context::SpatialContext,
    scope: SpatialScope,
) -> (Vec<crate::cycling::Candidate>, Vec<WindowId>)
where
    S: CandidateSource + ?Sized,
    P: EligibilityPolicy + ?Sized,
    M: MonitorSource + ?Sized,
    V: VirtualDesktopSource + ?Sized,
{
    let candidates = source.snapshot();

    let eligible: Vec<WindowId> = candidates
        .iter()
        .filter(|c| policy.evaluate(active, c).is_eligible())
        .filter(|c| context_allows(monitors, desktops, spatial, scope, c))
        .map(|c| c.facts.window)
        .collect();

    (candidates, eligible)
}

/// Cycle driver with the spatial gate layered on top of eligibility.
/// Both filters must pass. Cycling order and eligibility rules are unchanged;
/// the spatial adapter only removes candidates, never reorders them.
#[allow(clippy::too_many_arguments)]
fn run_context_safe_cycle<S, P, A, M, V>(
    source: &S,
    policy: &P,
    activator: &mut A,
    active: &ActiveContext,
    monitors: &M,
    desktops: Option<&V>,
    spatial: &crate::context::SpatialContext,
    direction: crate::cycling::Direction,
) -> CycleOutcome
where
    S: CandidateSource + ?Sized,
    P: EligibilityPolicy + ?Sized,
    A: Activator + ?Sized,
    M: MonitorSource + ?Sized,
    V: VirtualDesktopSource + ?Sized,
{
    let (candidates, eligible) = collect_eligible_candidates(
        source,
        policy,
        active,
        monitors,
        desktops,
        spatial,
        SpatialScope::SameMonitor,
    );

    #[cfg(debug_assertions)]
    crate::util::append_debug_trace(&format!(
        "CYCLE_ELIGIBLE: active={} candidates={} eligible={:?}",
        active.foreground.0,
        candidates.len(),
        eligible.iter().map(|w| w.0).collect::<Vec<_>>()
    ));

    if eligible.is_empty() {
        return CycleOutcome::NoEligibleTarget;
    }

    for target in crate::cycling::cycle_order_directed(&candidates, active, direction) {
        if !eligible.contains(&target) {
            continue;
        }
        if activator.activate(target) == ActivationOutcome::Activated {
            return CycleOutcome::Activated(target);
        }
    }

    CycleOutcome::Exhausted
}

thread_local! {
 /// Worker-thread-owned virtual-desktop interface.
 /// `thread_local!` is what keeps COM ownership on the Worker thread: the
 /// value can only ever be touched from the thread that created it, and it
 /// is dropped — releasing the interface and its apartment — when that
 /// thread exits. `RefCell<Option<..>>` distinguishes "not tried yet" from
 /// "tried and unavailable", so a machine without COM is not re-probed on
 /// every keystroke.
    static VIRTUAL_DESKTOPS: std::cell::RefCell<Option<Option<VirtualDesktopManager>>> =
        const { std::cell::RefCell::new(None) };
}

/// Run `f` with the shared virtual-desktop adapter, creating it on first use.
fn with_virtual_desktops<R>(f: impl FnOnce(Option<&VirtualDesktopManager>) -> R) -> R {
    VIRTUAL_DESKTOPS.with(|cell| {
        let mut slot = cell.borrow_mut();
        let created = slot.get_or_insert_with(VirtualDesktopManager::create);
        f(created.as_ref())
    })
}

/// Spatial gate for one candidate. Fails closed when COM is unavailable.
fn context_allows<M, V>(
    monitors: &M,
    desktops: Option<&V>,
    spatial: &crate::context::SpatialContext,
    scope: SpatialScope,
    candidate: &Candidate,
) -> bool
where
    M: MonitorSource + ?Sized,
    V: VirtualDesktopSource + ?Sized,
{
    let Some(desktops) = desktops else {
        return false;
    };
    let facts = collect_spatial_facts(monitors, desktops, candidate.facts.window);
    evaluate_spatial(scope, spatial, &facts).is_eligible()
}

// ── Arrangement ───────────────────────────────────────────────────────

fn execute_snap(command: Command) {
    let Some(ctx) = resolve_context() else {
        report_arrangement_failure("no platform context");
        return;
    };

    // Maximize asks Windows to maximize, rather than resizing the window to the work area.
    // The two look alike and are not: sizing leaves the window in the *normal* state, so the
    // title bar still offers Maximize rather than Restore, a double-click maximizes it again
    // to a slightly different size, and the application never receives `WM_GETMINMAXINFO` and
    // so never gets the maximized bounds it asked for.
    //
    // The geometric plan stays as the fallback for a window whose own style forbids
    // maximizing -- a fixed-size dialog, a tool palette -- where sizing it to the work area
    // is still the best available answer and is what this did before.
    if command == Command::SnapMaximize && crate::arrangement::win32::try_real_maximize(ctx.target)
    {
        #[cfg(debug_assertions)]
        crate::util::append_debug_trace("WORKER_ARRANGE: SnapMaximize real=1");
        return;
    }

    let plan = match command {
        Command::SnapLeft => snap::plan_snap_left(&ctx.work_area, ctx.target),
        Command::SnapRight => snap::plan_snap_right(&ctx.work_area, ctx.target),
        Command::SnapTop => snap::plan_snap_top(&ctx.work_area, ctx.target),
        Command::SnapBottom => snap::plan_snap_bottom(&ctx.work_area, ctx.target),
        Command::SnapMaximize => snap::plan_snap_maximize(&ctx.work_area, ctx.target),
        Command::SnapThirdLeft => thirds::plan_snap_third_left(&ctx.work_area, ctx.target),
        Command::SnapThirdMiddle => thirds::plan_snap_third_middle(&ctx.work_area, ctx.target),
        Command::SnapThirdRight => thirds::plan_snap_third_right(&ctx.work_area, ctx.target),
        Command::SnapPercentLeft
        | Command::SnapPercentRight
        | Command::SnapPercentTop
        | Command::SnapPercentBottom => {
            let snapping = snapping_config();
            let (edge, pct) = match command {
                Command::SnapPercentLeft => (snap::SnapEdge::Left, snapping.percent_left),
                Command::SnapPercentRight => (snap::SnapEdge::Right, snapping.percent_right),
                Command::SnapPercentTop => (snap::SnapEdge::Top, snapping.percent_top),
                Command::SnapPercentBottom => (snap::SnapEdge::Bottom, snapping.percent_bottom),
                _ => unreachable!(),
            };
            snap::plan_snap_percent(&ctx.work_area, ctx.target, edge, pct)
        }
        _ => return,
    };

    apply_or_report(plan, command);
}

/// Move the active window to the next monitor, keeping its share of the work area.
///
/// The display set is enumerated **here**, on the Worker, and never on the Hook thread: the
/// hook callback is budgeted under 10 ms and must not allocate, and the monitor list is only
/// needed once a command has already been accepted. The Hook answers *whose chord is this*;
/// the Worker answers *what is a legal target and where does it go* — the same split
/// `DEC-006` drew for target eligibility.
fn execute_monitor_move() {
    // `resolve_context` first, because it is the one gate that refuses a Wira Desk window of
    // our own (`LBR-WM-6`, `DEC-006`). Enumerating before that check would do work for a
    // target that is about to be refused.
    let Some(ctx) = resolve_context() else {
        report_arrangement_failure("no platform context");
        return;
    };
    let hwnd = ctx.target.0;

    // Fresh every invocation, cached nowhere: an `HMONITOR` is a handle rather than an
    // identity, so a list kept between keypresses outlives the configuration it described.
    let monitors = enumerate_monitors();
    let Some(from) = index_of_window_monitor(hwnd, &monitors) else {
        report_arrangement_failure("active window resolved to no enumerated monitor");
        return;
    };
    let Some(to) = monitor::next_monitor_index(monitors.len(), from) else {
        // A single attached monitor. This is a **successful no-op**, not a failure: nothing
        // moves, nothing is shown, and nothing is logged beyond the debug trace. Warning here
        // would train the user to ignore a log that is meant to carry real problems.
        #[cfg(debug_assertions)]
        crate::util::append_debug_trace(
            "WORKER_ARRANGE: MoveToNextMonitor noop=1 reason=one_monitor",
        );
        return;
    };

    let Some(window_rect) = current_window_rect(hwnd) else {
        report_arrangement_failure("could not read the active window rect");
        return;
    };

    // No restore here any more: `apply_plan` does it for every placement of every command,
    // which is what this comment used to claim about the snap path without it being true.
    apply_or_report(
        monitor::plan_move_to_monitor(
            &monitors[from].work,
            &monitors[to].work,
            ctx.target,
            window_rect,
        ),
        Command::MoveToNextMonitor,
    );
}

/// The window's current outer rect, as the planner's proportional source.
fn current_window_rect(hwnd: isize) -> Option<crate::arrangement::Rect> {
    use windows_sys::Win32::Foundation::{FALSE, RECT};
    let mut r: RECT = RECT {
        left: 0,
        top: 0,
        right: 0,
        bottom: 0,
    };
    // SAFETY: `hwnd` came from `resolve_context`, which validated it with `IsWindow`, and
    // `&mut r` is a unique pointer to a live local of exactly the type the API writes. A
    // failed call leaves `r` as initialised above, which is why the result is checked rather
    // than trusted — a zeroed rect would otherwise read as a degenerate window.
    if unsafe { windows_sys::Win32::UI::WindowsAndMessaging::GetWindowRect(hwnd, &mut r) } == FALSE
    {
        return None;
    }
    crate::arrangement::win32::rect_from_win32(r).ok()
}

// The Worker's own configuration, replaced only by an accepted reload.
// `thread_local!` rather than a `static`: the Worker is the main thread and the
// `WM_APP_RELOAD_CONFIG` handler runs on it, so installing a snapshot is a
// plain assignment on the owning thread. Single-threaded by construction means
// no lock on the arrangement path, and it makes the Hook/Worker isolation the
// AC demands true in both directions rather than by convention.
thread_local! {
    static WORKER_CONFIG: std::cell::RefCell<Option<crate::config::WorkerSnapshot>> =
        const { std::cell::RefCell::new(None) };
}

/// Install the snapshot handed over by an accepted reload.
pub fn install_config_snapshot(snapshot: crate::config::WorkerSnapshot) {
    if !snapshot.visual_enabled {
        if let Some(hwnd) = WORKER_HWND.get() {
            // SAFETY: KillTimer cancels any in-flight hold timer if reload arrives between arming and firing (SPEC-14-05).
            unsafe {
                windows_sys::Win32::UI::WindowsAndMessaging::KillTimer(hwnd, TIMER_SWITCHER_HOLD);
            }
        }
        SWITCHER_HOLD_ORIGIN.set(None);
        SWITCHER_CHORD_MODS.set(None);
    }
    WORKER_CONFIG.with(|slot| *slot.borrow_mut() = Some(snapshot));
}

fn worker_snapshot() -> crate::config::WorkerSnapshot {
    WORKER_CONFIG.with(|slot| {
        let mut slot = slot.borrow_mut();
        if slot.is_none() {
            let cfg = Config::load_or_default(&shared::config_path());
            *slot = Some(crate::config::WorkerSnapshot {
                layout: cfg.layout,
                snapping: cfg.snapping,
                visual_enabled: cfg.switcher.visual_enabled,
                visual_hold_delay_ms: cfg.switcher.visual_hold_delay_ms,
            });
        }
        slot.as_ref().expect("populated immediately above").clone()
    })
}

/// The layout configuration currently in force.
fn layout_config() -> shared::config::LayoutConfig {
    worker_snapshot().layout
}

/// The snapping configuration currently in force.
fn snapping_config() -> shared::config::SnappingConfig {
    worker_snapshot().snapping
}

/// `OverlappingStack` reuses the candidate contract for live
/// same-application windows, then keeps only those on the active target
/// monitor — without requiring the full spatial contract.
fn execute_stack() {
    let layout = layout_config();

    let Some(ctx) = resolve_context() else {
        report_arrangement_failure("no platform context");
        return;
    };

    let active = capture_active_context();
    let monitors = Win32Monitors;
    let origin = monitors.monitor_of(active.foreground);

    let candidates: Vec<WindowId> = Win32CandidateSource
        .snapshot()
        .into_iter()
        .filter(|c| WindowEligibility.evaluate(&active, c).is_eligible())
        // Same monitor only. This is a plain monitor comparison, deliberately
        // not the spatial contract: stacking must work before spatial filtering
        // converges.
        .filter(|c| origin.is_some() && monitors.monitor_of(c.facts.window) == origin)
        .map(|c| c.facts.window)
        .collect();

    apply_or_report(
        stack::plan_stack(&layout, &ctx.work_area, &candidates),
        Command::OverlappingStack,
    );
}

fn apply_or_report(plan: Result<PlacementPlan, PlanError>, command: Command) {
    match plan {
        Ok(plan) => {
            if plan.is_noop() {
                #[cfg(debug_assertions)]
                crate::util::append_debug_trace(&format!("WORKER_ARRANGE: {command:?} noop=1"));
                return;
            }
            let (applied, skipped) = apply_plan(&mut Win32WindowMover, &plan.placements);
            #[cfg(debug_assertions)]
            crate::util::append_debug_trace(&format!(
                "WORKER_ARRANGE: {command:?} applied={applied} skipped={skipped}"
            ));
            let _ = (applied, skipped);
        }
        Err(err) => report_arrangement_failure(&format!("{command:?} {err:?}")),
    }
}

/// Arrangement failures follow the Tier-2 path: a diagnostic, never a popup,
/// and never a downgrade of an existing Critical tray state.
fn report_arrangement_failure(detail: &str) {
    debug_log(&format!("Wira Desk: arrangement failed — {detail}"));
    #[cfg(debug_assertions)]
    crate::util::append_debug_trace(&format!("WORKER_ARRANGE: failed detail={detail}"));
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::context::fixtures::{FakeDesktops, FakeMonitors, MONITOR_A, MONITOR_B};
    use crate::context::SpatialContext;
    use crate::cycling::fixtures::{normal, ordered, ScriptedActivator, StaticSource};
    use crate::cycling::ActivationOutcome;

    fn active_ctx(w: isize) -> ActiveContext {
        crate::cycling::fixtures::active(w)
    }

    #[test]
    fn candidate_on_another_monitor_is_never_activated() {
        let candidates = ordered(vec![normal(1), normal(2), normal(3)]);
        let monitors = FakeMonitors(vec![
            (WindowId(1), Some(MONITOR_A)),
            (WindowId(2), Some(MONITOR_B)),
            (WindowId(3), Some(MONITOR_A)),
        ]);
        let desktops = FakeDesktops(vec![
            (WindowId(1), Some(true)),
            (WindowId(2), Some(true)),
            (WindowId(3), Some(true)),
        ]);
        let spatial = SpatialContext {
            origin_monitor: Some(MONITOR_A),
        };
        let mut activator = ScriptedActivator::always(ActivationOutcome::Activated);
        let outcome = run_context_safe_cycle(
            &StaticSource(candidates),
            &WindowEligibility,
            &mut activator,
            &active_ctx(1),
            &monitors,
            Some(&desktops),
            &spatial,
            crate::cycling::Direction::Forward,
        );
        assert_eq!(outcome, CycleOutcome::Activated(WindowId(3)));
        assert!(!activator.attempts.contains(&WindowId(2)));
    }

    #[test]
    fn candidate_on_another_virtual_desktop_is_never_activated() {
        let candidates = ordered(vec![normal(1), normal(2), normal(3)]);
        let monitors = FakeMonitors(vec![
            (WindowId(1), Some(MONITOR_A)),
            (WindowId(2), Some(MONITOR_A)),
            (WindowId(3), Some(MONITOR_A)),
        ]);
        let desktops = FakeDesktops(vec![
            (WindowId(1), Some(true)),
            (WindowId(2), Some(false)),
            (WindowId(3), Some(true)),
        ]);
        let spatial = SpatialContext {
            origin_monitor: Some(MONITOR_A),
        };
        let mut activator = ScriptedActivator::always(ActivationOutcome::Activated);
        let outcome = run_context_safe_cycle(
            &StaticSource(candidates),
            &WindowEligibility,
            &mut activator,
            &active_ctx(1),
            &monitors,
            Some(&desktops),
            &spatial,
            crate::cycling::Direction::Forward,
        );
        assert_eq!(outcome, CycleOutcome::Activated(WindowId(3)));
        assert!(!activator.attempts.contains(&WindowId(2)));
    }

    #[test]
    fn missing_com_leaves_focus_unchanged() {
        // No virtual-desktop adapter means nothing can be proven eligible.
        let candidates = ordered(vec![normal(1), normal(2)]);
        let monitors = FakeMonitors(vec![
            (WindowId(1), Some(MONITOR_A)),
            (WindowId(2), Some(MONITOR_A)),
        ]);
        let spatial = SpatialContext {
            origin_monitor: Some(MONITOR_A),
        };
        let mut activator = ScriptedActivator::always(ActivationOutcome::Activated);
        let outcome = run_context_safe_cycle(
            &StaticSource(candidates),
            &WindowEligibility,
            &mut activator,
            &active_ctx(1),
            &monitors,
            None::<&FakeDesktops>,
            &spatial,
            crate::cycling::Direction::Forward,
        );
        assert_eq!(outcome, CycleOutcome::NoEligibleTarget);
        assert!(activator.attempts.is_empty());
    }

    #[test]
    fn unknown_origin_monitor_leaves_focus_unchanged() {
        let candidates = ordered(vec![normal(1), normal(2)]);
        let monitors = FakeMonitors(vec![
            (WindowId(1), Some(MONITOR_A)),
            (WindowId(2), Some(MONITOR_A)),
        ]);
        let desktops = FakeDesktops(vec![(WindowId(1), Some(true)), (WindowId(2), Some(true))]);
        let spatial = SpatialContext {
            origin_monitor: None,
        };
        let mut activator = ScriptedActivator::always(ActivationOutcome::Activated);
        let outcome = run_context_safe_cycle(
            &StaticSource(candidates),
            &WindowEligibility,
            &mut activator,
            &active_ctx(1),
            &monitors,
            Some(&desktops),
            &spatial,
            crate::cycling::Direction::Forward,
        );
        assert_eq!(outcome, CycleOutcome::NoEligibleTarget);
    }

    #[test]
    fn epic_two_ordering_survives_the_spatial_gate() {
        // Spatial filtering removes candidates; it must never reorder them.
        let candidates = ordered(vec![normal(1), normal(2), normal(3), normal(4)]);
        let monitors = FakeMonitors(vec![
            (WindowId(1), Some(MONITOR_A)),
            (WindowId(2), Some(MONITOR_A)),
            (WindowId(3), Some(MONITOR_B)),
            (WindowId(4), Some(MONITOR_A)),
        ]);
        let desktops = FakeDesktops(vec![
            (WindowId(1), Some(true)),
            (WindowId(2), Some(true)),
            (WindowId(3), Some(true)),
            (WindowId(4), Some(true)),
        ]);
        let spatial = SpatialContext {
            origin_monitor: Some(MONITOR_A),
        };
        let mut activator = ScriptedActivator::always(ActivationOutcome::InvalidTarget);
        let outcome = run_context_safe_cycle(
            &StaticSource(candidates),
            &WindowEligibility,
            &mut activator,
            &active_ctx(1),
            &monitors,
            Some(&desktops),
            &spatial,
            crate::cycling::Direction::Forward,
        );
        assert_eq!(outcome, CycleOutcome::Exhausted);
        // 4 then 2 - least-recently-used first - with 3 removed by the
        // spatial gate and 1 (active) never retried.
        assert_eq!(activator.attempts, vec![WindowId(4), WindowId(2)]);
    }

    #[test]
    fn visual_switcher_collects_candidates_across_all_physical_monitors() {
        let candidates = ordered(vec![normal(1), normal(2), normal(3)]);
        let monitors = FakeMonitors(vec![
            (WindowId(1), Some(MONITOR_A)),
            (WindowId(2), Some(MONITOR_B)),
            (WindowId(3), Some(MONITOR_A)),
        ]);
        let desktops = FakeDesktops(vec![
            (WindowId(1), Some(true)),
            (WindowId(2), Some(true)),
            (WindowId(3), Some(true)),
        ]);
        let spatial = SpatialContext {
            origin_monitor: Some(MONITOR_A),
        };
        let (_all, eligible) = collect_eligible_candidates(
            &StaticSource(candidates),
            &WindowEligibility,
            &active_ctx(1),
            &monitors,
            Some(&desktops),
            &spatial,
            SpatialScope::AnyMonitorOnCurrentDesktop,
        );
        assert!(
            eligible.contains(&WindowId(2)),
            "Window on secondary monitor must be included in visual switcher (DEC-026)"
        );
        assert!(
            eligible.contains(&WindowId(3)),
            "Window on primary monitor must be included in visual switcher"
        );
    }

    #[test]
    fn visual_switcher_still_excludes_other_virtual_desktops() {
        let candidates = ordered(vec![normal(1), normal(2), normal(3)]);
        let monitors = FakeMonitors(vec![
            (WindowId(1), Some(MONITOR_A)),
            (WindowId(2), Some(MONITOR_B)),
            (WindowId(3), Some(MONITOR_A)),
        ]);
        let desktops = FakeDesktops(vec![
            (WindowId(1), Some(true)),
            (WindowId(2), Some(false)), // Other virtual desktop
            (WindowId(3), Some(true)),
        ]);
        let spatial = SpatialContext {
            origin_monitor: Some(MONITOR_A),
        };
        let (_all, eligible) = collect_eligible_candidates(
            &StaticSource(candidates),
            &WindowEligibility,
            &active_ctx(1),
            &monitors,
            Some(&desktops),
            &spatial,
            SpatialScope::AnyMonitorOnCurrentDesktop,
        );
        assert!(
            !eligible.contains(&WindowId(2)),
            "Candidate on another virtual desktop must be excluded even in visual switcher"
        );
        assert!(
            eligible.contains(&WindowId(3)),
            "Candidate on current virtual desktop must be included"
        );
    }

    #[test]
    fn blind_cycle_stays_locked_to_the_active_monitor() {
        let candidates = ordered(vec![normal(1), normal(2), normal(3)]);
        let monitors = FakeMonitors(vec![
            (WindowId(1), Some(MONITOR_A)),
            (WindowId(2), Some(MONITOR_B)),
            (WindowId(3), Some(MONITOR_A)),
        ]);
        let desktops = FakeDesktops(vec![
            (WindowId(1), Some(true)),
            (WindowId(2), Some(true)),
            (WindowId(3), Some(true)),
        ]);
        let spatial = SpatialContext {
            origin_monitor: Some(MONITOR_A),
        };
        let mut activator = ScriptedActivator::always(ActivationOutcome::Activated);
        let outcome = run_context_safe_cycle(
            &StaticSource(candidates),
            &WindowEligibility,
            &mut activator,
            &active_ctx(1),
            &monitors,
            Some(&desktops),
            &spatial,
            crate::cycling::Direction::Forward,
        );
        assert_eq!(outcome, CycleOutcome::Activated(WindowId(3)));
        assert!(
            !activator.attempts.contains(&WindowId(2)),
            "Blind cycle must never attempt secondary monitor window"
        );
    }

    #[test]
    fn a_cross_monitor_commit_activates_in_place_and_moves_no_window() {
        // Committing a card from a secondary monitor activates it in place (DEC-026 clause 1)
        let mut activator = ScriptedActivator::always(ActivationOutcome::Activated);
        let target = WindowId(2); // on Monitor B
        let outcome = activator.activate(target);
        assert_eq!(outcome, ActivationOutcome::Activated);
        assert_eq!(activator.attempts, vec![WindowId(2)]);
    }

    #[test]
    fn the_blind_cycle_after_a_cross_monitor_commit_locks_to_the_new_monitor() {
        // DEC-026 §B-8: after cross-monitor activation, target on MONITOR_B is foreground
        let candidates = ordered(vec![normal(1), normal(2), normal(4)]);
        let monitors = FakeMonitors(vec![
            (WindowId(1), Some(MONITOR_A)),
            (WindowId(2), Some(MONITOR_B)),
            (WindowId(4), Some(MONITOR_B)),
        ]);
        let desktops = FakeDesktops(vec![
            (WindowId(1), Some(true)),
            (WindowId(2), Some(true)),
            (WindowId(4), Some(true)),
        ]);
        // Now foreground is WindowId(2) on MONITOR_B
        let spatial_new = SpatialContext {
            origin_monitor: Some(MONITOR_B),
        };
        let mut activator = ScriptedActivator::always(ActivationOutcome::Activated);
        let outcome = run_context_safe_cycle(
            &StaticSource(candidates),
            &WindowEligibility,
            &mut activator,
            &active_ctx(2),
            &monitors,
            Some(&desktops),
            &spatial_new,
            crate::cycling::Direction::Forward,
        );
        assert_eq!(outcome, CycleOutcome::Activated(WindowId(4)));
        assert!(
            !activator.attempts.contains(&WindowId(1)),
            "Next blind cycle locks to MONITOR_B, excluding MONITOR_A"
        );
    }

    #[test]
    fn mouse_virtual_desktop_commands_dispatch_safely() {
        for cmd in [
            Command::NextVirtualDesktop,
            Command::PrevVirtualDesktop,
            Command::TaskView,
            Command::ShowDesktop,
        ] {
            // Must dispatch without panicking in test environment
            execute_mouse_navigation(cmd);
        }
    }

    #[test]
    fn snap_commands_from_mouse_dispatch_to_planning() {
        for cmd in [
            Command::SnapLeft,
            Command::SnapRight,
            Command::SnapTop,
            Command::SnapBottom,
            Command::SnapMaximize,
            Command::SnapThirdLeft,
            Command::SnapThirdMiddle,
            Command::SnapThirdRight,
            Command::SnapPercentLeft,
            Command::SnapPercentRight,
            Command::SnapPercentTop,
            Command::SnapPercentBottom,
            Command::OverlappingStack,
            Command::MoveToNextMonitor,
        ] {
            match cmd {
                Command::OverlappingStack => execute_stack(),
                Command::MoveToNextMonitor => execute_monitor_move(),
                _ => execute_snap(cmd),
            }
        }
    }

    #[test]
    fn switcher_commit_falls_through_on_invalid_target() {
        let mut controller = crate::switcher::SwitcherController::new();
        let candidates = vec![WindowId(101), WindowId(102), WindowId(103)];
        controller.open(
            WindowId(100),
            crate::switcher::layout::Rect::new(0, 0, 1920, 1080),
            candidates,
            &[16.0 / 9.0; 3],
            96,
            0,
        );

        let targets = controller.candidates_from_selection();
        assert_eq!(targets, vec![WindowId(101), WindowId(102), WindowId(103)]);

        // Scripted activator returns InvalidTarget for 101, then Activated for 102
        let mut activator = ScriptedActivator::scripted(vec![
            (WindowId(101), ActivationOutcome::InvalidTarget),
            (WindowId(102), ActivationOutcome::Activated),
        ]);

        let mut activated = None;
        for target in targets {
            if activator.activate(target) == ActivationOutcome::Activated {
                activated = Some(target);
                break;
            }
        }

        assert_eq!(activated, Some(WindowId(102)));
        assert_eq!(activator.attempts, vec![WindowId(101), WindowId(102)]);
    }

    #[test]
    fn a_cold_start_snapshot_carries_the_on_disk_switcher_settings() {
        WORKER_CONFIG.with(|slot| *slot.borrow_mut() = None);
        let snap = worker_snapshot();
        let cfg = Config::load_or_default(&shared::config_path());
        assert_eq!(snap.visual_enabled, cfg.switcher.visual_enabled);
        assert_eq!(snap.visual_hold_delay_ms, cfg.switcher.visual_hold_delay_ms);
    }

    #[test]
    fn disabled_visual_switcher_never_arms_the_hold_timer() {
        let outcome = CycleOutcome::Activated(WindowId(10));
        let mods = crate::hook::ModifierState {
            win: true,
            ctrl: false,
            alt: false,
            shift: false,
        };
        let delay = decide_switcher_hold_delay(&outcome, Some(mods), false, 150);
        assert_eq!(
            delay, None,
            "Disabled visual switcher must never arm hold timer"
        );
    }

    #[test]
    fn enabled_visual_switcher_arms_with_the_configured_hold_delay() {
        let outcome = CycleOutcome::Activated(WindowId(10));
        let mods = crate::hook::ModifierState {
            win: true,
            ctrl: false,
            alt: false,
            shift: false,
        };
        let delay = decide_switcher_hold_delay(&outcome, Some(mods), true, 250);
        assert_eq!(
            delay,
            Some(250),
            "Enabled visual switcher arms with configured hold delay"
        );
    }

    #[test]
    fn an_out_of_range_hold_delay_is_clamped_before_it_reaches_settimer() {
        let outcome = CycleOutcome::Activated(WindowId(10));
        let mods = crate::hook::ModifierState {
            win: true,
            ctrl: false,
            alt: false,
            shift: false,
        };
        let delay_under = decide_switcher_hold_delay(&outcome, Some(mods), true, 50);
        assert_eq!(
            delay_under,
            Some(100),
            "Hold delay below 100ms must be clamped to 100ms"
        );

        let delay_over = decide_switcher_hold_delay(&outcome, Some(mods), true, 1000);
        assert_eq!(
            delay_over,
            Some(500),
            "Hold delay above 500ms must be clamped to 500ms"
        );
    }

    #[test]
    fn a_reload_between_arming_and_firing_does_not_open_the_overlay() {
        // Set up in-flight arming state
        SWITCHER_HOLD_ORIGIN.set(Some(WindowId(100)));
        SWITCHER_CHORD_MODS.set(Some(crate::hook::ModifierState {
            win: true,
            ctrl: false,
            alt: false,
            shift: false,
        }));

        // Now install a snapshot where visual_enabled is false (reload arrived between arming and firing)
        let disabled_snap = crate::config::WorkerSnapshot {
            layout: shared::config::LayoutConfig::default(),
            snapping: shared::config::SnappingConfig::default(),
            visual_enabled: false,
            visual_hold_delay_ms: 150,
        };
        install_config_snapshot(disabled_snap);

        // State must be cleared immediately by install_config_snapshot
        assert_eq!(SWITCHER_HOLD_ORIGIN.get(), None);
        assert_eq!(SWITCHER_CHORD_MODS.get(), None);

        // Reset thread-local worker config
        WORKER_CONFIG.with(|slot| *slot.borrow_mut() = None);
    }

    #[test]
    fn changing_hold_delay_while_timer_armed_keeps_in_flight_delay() {
        // Defined resolution for SPEC-14-05 item 7: changing visual_hold_delay_ms while
        // a timer is already armed preserves the in-flight timer and origin; new delay
        // takes effect on subsequent cycle chords.
        SWITCHER_HOLD_ORIGIN.set(Some(WindowId(100)));
        SWITCHER_CHORD_MODS.set(Some(crate::hook::ModifierState {
            win: true,
            ctrl: false,
            alt: false,
            shift: false,
        }));

        let new_delay_snap = crate::config::WorkerSnapshot {
            layout: shared::config::LayoutConfig::default(),
            snapping: shared::config::SnappingConfig::default(),
            visual_enabled: true,
            visual_hold_delay_ms: 350,
        };
        install_config_snapshot(new_delay_snap);

        // In-flight hold origin remains preserved
        assert_eq!(SWITCHER_HOLD_ORIGIN.get(), Some(WindowId(100)));

        // Clean up
        SWITCHER_HOLD_ORIGIN.set(None);
        SWITCHER_CHORD_MODS.set(None);
        WORKER_CONFIG.with(|slot| *slot.borrow_mut() = None);
    }

    #[test]
    fn releasing_shift_during_the_hold_window_still_opens_the_overlay() {
        let mods_with_shift = crate::hook::ModifierState {
            win: true,
            ctrl: false,
            alt: false,
            shift: true,
        };
        let outcome = CycleOutcome::Activated(WindowId(1));
        let delay = decide_switcher_hold_delay(&outcome, Some(mods_with_shift), true, 150);
        assert_eq!(delay, Some(150));

        // When armed in execute_cycle, Shift is stripped from SWITCHER_CHORD_MODS (DEC-026 §B-5)
        let chord_mods = Some(mods_with_shift).map(|mut m| {
            m.shift = false;
            m
        });
        SWITCHER_CHORD_MODS.set(chord_mods);

        let recorded_mods = SWITCHER_CHORD_MODS
            .get()
            .expect("hold timer armed chord mods");
        assert!(recorded_mods.win);
        assert!(
            !recorded_mods.shift,
            "Shift must be stripped from SWITCHER_CHORD_MODS"
        );

        // Clean up
        SWITCHER_HOLD_ORIGIN.set(None);
        SWITCHER_CHORD_MODS.set(None);
        SWITCHER_BACKWARD_ENTRY.set(false);
    }

    #[test]
    fn a_backward_entry_opens_on_the_last_card() {
        let mut controller = crate::switcher::SwitcherController::new();
        let candidates = vec![WindowId(101), WindowId(102), WindowId(103)];

        // Under Direction::Backward, backward entry flag is armed
        let direction = crate::cycling::Direction::Backward;
        SWITCHER_BACKWARD_ENTRY.set(direction == crate::cycling::Direction::Backward);
        assert!(
            SWITCHER_BACKWARD_ENTRY.get(),
            "Backward entry flag must be set by CyclePrev"
        );

        let backward = SWITCHER_BACKWARD_ENTRY.get();
        let initial_index = if backward && !candidates.is_empty() {
            candidates.len() - 1
        } else {
            0
        };
        assert_eq!(initial_index, 2);

        controller.open(
            WindowId(100),
            crate::switcher::layout::Rect::new(0, 0, 1920, 1080),
            candidates,
            &[16.0 / 9.0; 3],
            96,
            initial_index,
        );

        assert_eq!(controller.selected_window(), Some(WindowId(103)));
        SWITCHER_BACKWARD_ENTRY.set(false);
    }

    #[test]
    fn backward_blind_cycle_activates_in_reverse_order_and_stays_monitor_locked() {
        // Normal 1 (active), 2 on Monitor B, 3 on Monitor A, 4 on Monitor A
        let candidates = ordered(vec![normal(1), normal(2), normal(3), normal(4)]);
        let monitors = FakeMonitors(vec![
            (WindowId(1), Some(MONITOR_A)),
            (WindowId(2), Some(MONITOR_B)),
            (WindowId(3), Some(MONITOR_A)),
            (WindowId(4), Some(MONITOR_A)),
        ]);
        let desktops = FakeDesktops(vec![
            (WindowId(1), Some(true)),
            (WindowId(2), Some(true)),
            (WindowId(3), Some(true)),
            (WindowId(4), Some(true)),
        ]);
        let spatial = SpatialContext {
            origin_monitor: Some(MONITOR_A),
        };
        let mut activator = ScriptedActivator::always(ActivationOutcome::Activated);
        // Forward order among eligible [3, 4] from active 1 is [4, 3]
        // Backward order is the forward rotation unreversed: activates [3] first!
        let outcome = run_context_safe_cycle(
            &StaticSource(candidates),
            &WindowEligibility,
            &mut activator,
            &active_ctx(1),
            &monitors,
            Some(&desktops),
            &spatial,
            crate::cycling::Direction::Backward,
        );
        assert_eq!(outcome, CycleOutcome::Activated(WindowId(3)));
        assert!(
            !activator.attempts.contains(&WindowId(2)),
            "Secondary monitor window is excluded"
        );
    }

    #[test]
    fn queued_cycle_and_cycle_prev_each_keep_their_own_direction() {
        let _guard = crate::ring::tests::TEST_LOCK.lock().unwrap();
        // Clear ring and push Command::Cycle followed by Command::CyclePrev
        while ring::pop().is_some() {}
        assert!(ring::push(Command::Cycle.as_u8()));
        assert!(ring::push(Command::CyclePrev.as_u8()));

        let first = ring::pop().map(Command::from_u8);
        let second = ring::pop().map(Command::from_u8);

        assert_eq!(first, Some(Command::Cycle));
        assert_eq!(second, Some(Command::CyclePrev));
    }
}
