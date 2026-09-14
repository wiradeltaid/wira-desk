# Privacy

**Last updated:** 2026-09-14

Wira Desk sends **no telemetry, ever** — no analytics, no crash reporting, no account,
and no identifier of any kind. Exactly **one** thing in the product touches the network,
and the next section describes it in full rather than summarising it. Configuration and optional log files are stored locally under
`%APPDATA%\WiraDesk\`.

## Checking for updates

Wira Desk can check whether a newer version exists. This is the only network request the
product ever makes.

**What is sent.** An HTTPS `GET` for a small file published beside each release. Nothing is
attached to it: no version, no machine name, no user name, no configuration, no identifier,
no counter. It is the same request a browser makes for a public URL.

**What that reveals anyway, because a request cannot hide it.** The server sees the IP
address it came from, and that something asked for a Wira Desk release file. An IP address
is an approximate location and, to whoever runs the network you are on, a device. We do not
receive any of it: the file is hosted on GitHub, so GitHub's logs see it under GitHub's own
privacy policy, exactly as they would if you opened the releases page yourself.

**What is not sent, and could not be.** The request carries no payload, so nothing about how
you use the product travels with it — not which shortcuts you pressed, not which windows
were open, not how long the daemon has run, not whether you had checked before.

**Turning it off.** Settings has a toggle. Turning it off stops the periodic check
completely, and the manual button stays, so you can ask once without leaving anything
running. Nothing degrades either way: the check reports that a version exists, it does not
gate anything.

## Keystrokes

The daemon installs a global keyboard hook, so it observes every key event on the desktop.
It reads the virtual-key code and the injected-input flag in order to match the two
configured shortcuts, and **records none of it** — not to the log file, not to the debug
trace, not anywhere that outlives the event. No logging call in the codebase takes a
virtual-key value as an argument.

## Mouse events

The daemon installs a global low-level mouse hook as well, because the auxiliary mouse navigation
(thumb buttons and wheel tilt) is on by default. So it observes every mouse event on the desktop,
and what it does with them is narrower than that sentence sounds.

**Cursor movement is passed straight through.** `WM_MOUSEMOVE` returns on the first line of the
callback, before any lock, any allocation, and any other read. Movement is the overwhelming
majority of mouse events, and none of it is looked at.

**The cursor position is never read at all.** Windows hands the callback a structure holding the
pointer coordinates alongside the button data. Two values are taken out of it: the message type,
and the button identifier — which thumb button, which tilt direction. The coordinate field is not
read anywhere in the codebase, so where you click, and where your pointer was, do not exist inside
this program even for the duration of the event.

That distinction carries more than it looks. Button identity says "the back thumb button was
pressed" and nothing else. Coordinates would have said what you were pointing at.

**Nothing is recorded**, on the same terms as the keyboard hook: not to the log file, not to the
debug trace, not anywhere that outlives the event.

**Turning it off.** Auxiliary mouse navigation can be switched off in Settings. With it off the
mappings stop; nothing else about the daemon changes.

## Window metadata

To choose a switching or arrangement target the daemon reads window class names, visibility
and cloak state, and the executable basename of the owning process. That is read at the
moment a shortcut fires, kept only for the duration of that operation, and never uploaded.
Window titles are not used for switching decisions.

## What is stored on disk

| Path | Contents | Retention |
| --- | --- | --- |
| `%APPDATA%\WiraDesk\config.toml` | Your shortcuts, bypass lists, layout, auto-start flag | Until you delete it |
| `%APPDATA%\WiraDesk\wiradesk.log` | Timestamped warning lines | Until you delete it — **there is no log rotation in this version**, so the file grows unbounded |

Both live in your user profile at normal user permissions, so treat them as readable by
anything else running as you. To reset configuration, delete `config.toml` only — see
`CHANGELOG.md`, because deleting the whole folder is not a reset.