# Wira Desk Security Policy

<!-- Copied from the Wira Delta Indonesia legal source (wira-desk/security.en.md) on 2026-09-24.
     Edit the source, then copy it here again. -->

This is an English translation of the Indonesian original. If the two differ in interpretation,
the Indonesian text prevails.

**In effect since:** 2026-09-24
**Applies to:** Wira Desk 0.3.0 and later

Wira Desk runs with Administrator rights, installs global low-level keyboard and mouse hooks
(`WH_KEYBOARD_LL`, `WH_MOUSE_LL`), can register a logon task that starts it with Administrator rights
without a prompt, reads the list of top-level windows (including their titles, for the visual
switcher), and can download and run a new version's installer when you ask it to. Those properties
deserve scrutiny. That is why `docs/threat-model.md` in the repository sets out the trust boundaries,
the reason for each privilege, and the risks that remain after mitigation. Read it first if you are
deciding whether this software can be trusted.

## 1. Terms Used in This Document

- **Daemon**: the Wira Desk process that runs in the tray and carries out shortcuts (`wiradesk.exe`).
- **Settings**: the Wira Desk settings window (`wiradesk-settings.exe`), which runs without
  Administrator rights.
- **Window**: an application window on the Windows desktop.
- **Hook**: a Windows mechanism that lets a program receive every keyboard or mouse event on the
  desktop before the event reaches the application it is meant for.
- **Auto-start**: a Windows Task Scheduler task that starts the daemon when you log on.
- **Checksum**: the SHA-256 fingerprint of a file. A file that changes by even one byte has a different
  checksum.
- **UAC**: the Windows prompt that asks for permission before a program receives Administrator rights.

## 2. Three Facts Most People Want Up Front

- **No keystroke content is recorded.** The keyboard hook reads virtual-key codes to match the
  shortcuts you have set, and writes none of them to disk, to the log, or to the debug trace. No logging
  call in the code takes a key value as an argument.
- **The mouse hook does not read the cursor position, and no mouse activity is recorded.** The mouse
  hook exists for the thumb-button and tilt-wheel mappings, which are on by default. Cursor movement is
  returned on the callback's first line, before any lock or allocation. Of the event structure Windows
  supplies, only the event type and the button data are taken. **The coordinate field is not read
  anywhere in the code.** One exception exists outside the hook: the visual switcher overlay reads the
  cursor position only while the cursor is over that overlay, to tell which card is being pointed at,
  and then discards it.
- **No analytics, no account, and no separate updater service.** The update check runs inside the daemon
  process. Wira Desk makes two kinds of outbound HTTPS request:
  1. The update check to `https://wiradelta.com/api/v1/update/wira-desk/`, automatically once a day or when you ask for
     it. Its User-Agent header names the product, the Wira Desk version, the Windows version, and the
     processor architecture, with no other computer, user, or configuration data. This endpoint runs on
     the same server as the `wiradelta.com` site, behind Cloudflare, and answers itself without
     redirecting to GitHub. Our server records the IP address, the User-Agent, and the time. After 30
     days, the raw records are deleted; what remains is only daily aggregate counts (app version,
     Windows version, architecture) and an estimated number of devices, with no IP address. The automatic
     check is on by default and can be switched off completely in Settings.
  2. The installer download from GitHub (`github.com/wiradeltaid/wira-desk`), which happens only when
     you press "Download and install" in Settings. The installer is checked against the published
     SHA-256 checksum before it runs.

  Configuration and logs stay in `%APPDATA%\WiraDesk\`. The Wira Desk Privacy Policy describes both
  requests line by line.

## 3. Reporting a Vulnerability

Use **GitHub Security Advisories** on the Wira Desk repository ("Report a vulnerability" on the Security
tab), so that the report stays private until a fix exists. Do not open a public issue for a suspected
vulnerability.

Helpful in a report: the Windows build, the Wira Desk version, what you did, what happened, and, if you
have one, the simplest way to reproduce it. There is no bounty program and no guaranteed response time.
This is a small project, and honesty about that is more useful than a promise it cannot keep.

**In scope:**

- anything that lets a user who is not an administrator gain higher rights;
- anything that reads data the daemon should not expose;
- anything that makes the daemon, which runs with Administrator rights, act on input it should not
  trust.

**Out of scope, documented rather than fixed:**

- an attacker who is already an administrator on that computer;
- disabling a shortcut through resource squatting (for example, on the mutex). That only stops a
  convenience feature, and grants no rights;
- the residual risks in §9 and in `docs/threat-model.md`. Those are known trade-offs, not unreported
  bugs.

**Supported versions.** Only the latest release receives fixes. There is no long-term support branch.
This is a small project, not a commercial product with an SLA.

## 4. Release Integrity

**Release files are not code-signed.** There are three consequences, and you can see all of them:

- Windows SmartScreen warns on first run. Because the daemon needs Administrator rights, the UAC prompt
  shows an unverified publisher. That is expected for an unsigned build and is not evidence that the
  file was tampered with. But it also means the prompt cannot help you tell a genuine file from a fake
  one.
- On a computer with **Smart App Control** turned on, the daemon cannot run at all. Unlike SmartScreen,
  there is no way past it: Smart App Control judges the file itself, and an unsigned file that no
  reputation service has seen is refused, with only a CodeIntegrity entry in the event log to show for
  it. No setting in Wira Desk changes that.
- Every release publishes a `SHA256SUMS` file. Compare your download's checksum with `Get-FileHash`
  before running it. A checksum served from the same place as the download proves the file arrived
  intact, **not** who made it.

The strongest verification available today is **building from source yourself** from the repository,
which is why the full source is published rather than release files alone. Code signing is the real
fix, and it is not in place yet. Until it is, treat any "Wira Desk" file from anywhere other than this
repository's releases page as untrusted.

## 5. The Updater, and What Verifies It

- The release descriptor (a small file naming the latest version, the installer address, and its
  checksum) is fetched from `https://wiradelta.com/api/v1/update/wira-desk/` over HTTPS only. Our server serves that
  descriptor itself, with no redirect to GitHub. No code path can fetch it over plain HTTP, and a
  descriptor larger than 64 KB is refused.
- The installer address inside the descriptor must be HTTPS and must sit exactly on the host and
  repository `github.com/wiradeltaid/wira-desk`. The installer is still downloaded from GitHub
  Releases, not from our server. Any other address is refused, so a tampered descriptor can at worst
  make the update fail.
- The download is made by Settings, which runs without Administrator rights, with a 192 MB limit. The
  SHA-256 checksum is computed as the file is written and compared with the checksum in the descriptor.
  If they do not match, the file is deleted and nothing is run.
- The installer is started through the Windows Shell, so Windows always shows the UAC prompt. Wira Desk
  never raises its rights silently.
- The descriptor address can be overridden by an environment variable only in debug builds used for
  development. That code is not compiled into release builds, so no setting in the copy you install can
  redirect where an installer is fetched from.

Until release files are signed, the checksum and HTTPS are the whole of the verification. The descriptor
and the installer are published by the same release process, so a matching checksum proves the download
is the same as what was published, not who published it.

## 6. Why Wira Desk Needs Administrator Rights

Administrator rights exist for one purpose: activating and moving windows owned by processes at a higher
integrity level, which Windows would otherwise block (User Interface Privilege Isolation, UIPI). Without
those rights, the daemon could arrange your text editor but not the Administrator terminal beside it.

Those rights are **not** used to read the memory of other processes. The daemon opens processes with
`PROCESS_QUERY_LIMITED_INFORMATION`, never `PROCESS_VM_READ`. The manifest is not the only check: the
daemon re-checks its own token at startup and refuses to run without Administrator rights.

## 7. Hardening Guidance

These two points matter more than anything else on this page:

- **Install in a folder only administrators can write to**, such as `%ProgramFiles%`. The auto-start
  task runs the daemon with Administrator rights at every logon without a prompt, so anyone able to
  overwrite the executable at that path gains an Administrator foothold without a prompt. The task
  stores an absolute path and sets no working folder, so the path itself cannot be hijacked; the file
  permissions are what protect it.
- **Do not turn on auto-start from a build in `Downloads`, `Desktop`, or any other folder an ordinary
  user can write to**, for the same reason. If you use the portable version (zip), extract it first to
  a folder only administrators can write to.

Both points are now **checked, not only asked for**, and it is worth being clear about what that means:

- The daemon reads the permissions of its own executable and of the folder holding it. If a party that
  is not an administrator holds a right that would let it replace either one, and auto-start is
  registered, you get a warning: one line in `wiradesk.log` and a warning dot on the tray icon. **The
  daemon warns; it does not refuse.** Auto-start still turns on, because a check that blocked running
  from a build folder would be switched off rather than heeded, and the choice remains yours to make
  knowingly. The absence of a warning here does not mean safety on anything but this one question.
- The stored path no longer goes stale. Because the logon task stores an absolute path frozen when
  auto-start was turned on, moving the executable used to leave the task pointing at the old location.
  As a result, installing properly *after* first running from `Downloads` left the file in `Downloads`
  as the one Windows ran with Administrator rights. The daemon now re-points the task at itself every
  time it starts.

Neither is a substitute for installing in the right place. They tell you when you have not.

The installer is the third piece, and the one that makes the right place the default: it requires
Administrator rights, installs to `%ProgramFiles%\Wira Desk`, and **offers no per-user install
location**. An installer that offered `%LOCALAPPDATA%` would be offering the escalation route above as a
convenience, so that option does not exist. The installer also does not turn on auto-start. Registering
an Administrator logon task without a prompt is a decision that stays with you.

Uninstalling removes the scheduled task. An `ONLOGON` task with `/RL HIGHEST` that outlives the
executable it points to is the worst thing an uninstaller could leave behind.

## 8. Design Notes

- The hook callback is bounded by design: no heap allocation, no lock, no file I/O, and no logging on the
  callback path.
- `SetDllDirectoryW` runs as the first statement in `main` to remove the current folder from the DLL
  search order, so a DLL someone plants cannot load with the daemon's rights.
- Configuration reload uses explicit `WM_APP` messages and applies all or nothing: a file that is
  unreadable, malformed, or invalid leaves the previous configuration in force and produces one warning.
  **No configuration value ever becomes a path or a command line.**
- Every `unsafe` block carries a `SAFETY:` comment stating the precondition it relies on, and the
  compiler enforces it: `undocumented_unsafe_blocks` and `missing_safety_doc` are set to `deny` in the
  workspace lints, so an undocumented block fails the build.
- Dependencies are gated in CI by `cargo-deny` (advisories, licenses, bans, and sources), and the
  repository is scanned by `gitleaks` across the whole git history and the working tree.

## 9. Known Residual Risks

Listed rather than hidden:

- Auto-start from a folder an ordinary user can write to remains an escalation route without a prompt if
  you proceed past the warning in §7.
- A deliberate write to `config.toml` by a malicious party can influence the behavior of the daemon,
  which runs with Administrator rights, limited to typed fields with no path or command execution.
- Release files are unsigned (§4).
- Between the moment the installer's checksum is matched and the moment Windows starts it, another
  program running as the same user could replace the file in the temp folder. The window is small, and
  it closes properly only with a signature check once release files are signed. The risk is no worse
  than downloading the installer yourself and running it.
- The COM interface for virtual desktops is declared by hand and only minimally tested.
- Shortcuts can be disabled through mutex squatting or abuse of the exception lists, with no gain in
  rights.

The full analysis, including the trust boundaries and the reason for each privilege, is in
`docs/threat-model.md`.

## 10. Changes to This Document

This document is published with the same content in `SECURITY.md` in the Wira Desk repository and at
`wiradelta.com/wira-desk/security/`. The Indonesian original is in `SECURITY.id.md` and at
`wiradelta.com/id/wira-desk/security/`. When the behavior described here changes, this document is
changed and the date at the top is updated.

## 11. Language

This is an English translation of the Indonesian original. If the two differ in interpretation,
the Indonesian text prevails.
