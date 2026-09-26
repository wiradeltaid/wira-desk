# Wira Desk Privacy Policy

<!-- Copied from the Wira Delta Indonesia legal source (wira-desk/privacy.en.md) on 2026-09-24.
     Edit the source, then copy it here again. -->

This is an English translation of the Indonesian original. If the two differ in interpretation,
the Indonesian text prevails.

**In effect since:** 2026-09-24
**Applies to:** Wira Desk 0.3.0 and later

Wira Desk is published by **Wira Delta Indonesia** ("we"). This document explains what Wira Desk reads
on your computer, what it stores on disk, and what data reaches us.

Wira Desk contains no analytics, no crash reporting, no account, and no device ID. Two things in the
product use the network: the update check, which reaches our server, and the installer download,
which happens only when you ask for it. Both are described in full in §2 and §3 rather than
summarized. Everything else stays on your computer.

## 1. Terms Used in This Document

- **Daemon**: the Wira Desk process that runs in the tray and carries out shortcuts (`wiradesk.exe`).
- **Settings**: the Wira Desk settings window (`wiradesk-settings.exe`).
- **Window**: an application window on the Windows desktop.
- **Hook**: a Windows mechanism that lets a program receive every keyboard or mouse event on the
  desktop before the event reaches the application it is meant for.
- **Update check**: the request that asks whether a newer version of Wira Desk exists.
- **User-Agent**: the field in every HTTPS request that names the program making it.
- **Log**: a text file where Wira Desk records warnings.

## 2. Update Check

Wira Desk can check whether a newer version exists. Since version 0.3.0 this request goes to our server
at `wiradelta.com`, so that we can count how many copies of each version are still in use. That server
is the same one that runs the `wiradelta.com` site, and it answers this request itself, without passing
it on to GitHub or anyone else.

**When it is sent.** The daemon sends the first update check about two minutes after it starts, then
once every 24 hours while it runs. The "Check for updates" button in Settings sends the same request
once each time you press it. The automatic check is on by default. That is deliberate, in place of
asking once the first time Wira Desk runs.

**What is sent.** One HTTPS `GET` request to `https://wiradelta.com/api/v1/update/wira-desk/`. Its User-Agent header
contains four things: the product name, the Wira Desk version, the Windows version, and the processor
architecture (for example x64 or ARM64). Apart from those four things, nothing is attached: no computer
name, no user name, no configuration, no device ID, and no counter.

**What is revealed anyway, even though it is not sent.** Our server receives the IP address the request
came from and the time of the request, because every request on the internet carries both. An IP
address indicates an approximate location and, to whoever runs the network you are on, a particular
device. Traffic to `wiradelta.com` passes through Cloudflare, which sees the same request as it forwards
it, under Cloudflare's own privacy policy.

**What we keep, and for how long.** Our server records each update check with three things: the IP
address, the User-Agent (and so the Wira Desk version, the Windows version, and the architecture), and
the time. After 30 days, the raw records are deleted; what remains is only daily aggregate counts based
on the data the app sends (app version, Windows version, architecture) and an estimated number of
devices, with no IP address. We use this data only to learn which versions are still in use, on which
Windows versions and architectures, and on roughly how many devices.
We do not use it to identify you, we do not combine it with other data, and we do not give it to anyone
else. The basis for processing is legitimate interest, one of the processing bases in Law Number 27 of
2022 on Personal Data Protection (UU PDP). You can stop it at any time by turning the update check off.

**What is not sent, and cannot be.** The request carries no content, so nothing about how you use the
product travels with it: not the shortcuts you pressed, not the windows that were open, not how long
the daemon has run, and not whether you had checked before.

**Turning it off.** In Settings, on the About tab, turn off "Check for updates automatically". The
automatic check stops completely, with no need to restart the daemon. The "Check for updates" button
stays, so you can ask once without leaving anything running. Nothing is lost: the update check only
reports that a new version exists, and it does not lock any feature. With the automatic check off and
the button not pressed, Wira Desk sends nothing to the network.

**Versions before 0.3.0.** Wira Desk 0.2.4 and earlier send the update check to GitHub
(`github.com/wiradeltaid/wira-desk`), not to our server, with a User-Agent that holds only the product
name and version, for example `WiraDesk/0.2.4`. We do not
receive those requests. GitHub sees them under GitHub's own privacy policy.

## 3. Installer Download

**When it is sent.** Only when you press "Download and install" in Settings, on the About tab, after an
update check has found a new version. This download never runs automatically.

**What is sent.** One HTTPS `GET` request for that release's installer file on GitHub
(`github.com/wiradeltaid/wira-desk`), which GitHub then redirects to its file storage server. The
User-Agent is the same as for the update check. Nothing else is attached. The installer is still
downloaded from GitHub, not from our server.

**What is revealed anyway, even though it is not sent.** GitHub receives the IP address and the time of
the download, under GitHub's own privacy policy. We do not receive data about this download.

**What happens next.** Settings saves the installer in a new, randomly named folder in your temp
folder, compares its SHA-256 checksum with the checksum published for that release, and then runs it.
If they do not match, the file is deleted and nothing is run. Windows shows a UAC prompt because the
installer needs Administrator rights.

**If you do not want to use it.** Do not press the button. You can still download the installer
yourself from the releases page.

## 4. Links Opened in the Browser

Some buttons in Settings (release notes, website, source code, bug reports, and help) open a page in
your default browser. Wira Desk allows only addresses on `wiradelta.com` and
`github.com/wiradeltaid/wira-desk`. Those visits are made by your browser, not by Wira Desk, and are
governed by the privacy policy of the site being opened. For `wiradelta.com`, that is our site's Privacy
Policy.

## 5. Keystrokes

The daemon installs a global low-level keyboard hook (`WH_KEYBOARD_LL`) so that shortcuts work from any
window. Because of that, the daemon receives every key event on the desktop. Windows offers no narrower
way to implement a global shortcut.

From each event, the daemon reads the virtual-key code and the synthetic-input flag, matches them
against the shortcuts you have set, and **records none of it**: not to the log, not to the debug trace,
and not anywhere that outlasts the event. This can be checked: no logging call in the code takes a
virtual-key value as an argument.

The daemon also sends one synthetic key, `VK_NONAME` (a code nothing in Windows uses), so that the Start
menu does not open after a shortcut that uses the Windows key.

## 6. Mouse Events

The daemon also installs a global low-level mouse hook (`WH_MOUSE_LL`) for mouse navigation: the thumb
buttons and the tilt wheel. This hook is installed for as long as the daemon runs, and mouse navigation
is on by default. So the daemon receives every mouse event on the desktop, and what it does with them is
narrower than that sounds.

**Cursor movement is passed straight through.** Cursor movement events (`WM_MOUSEMOVE`) are returned on
the first line of the callback, before any lock, allocation, or other read. Most mouse events are
movement, and none of them is looked at.

**The hook does not read the cursor position.** Windows hands the callback a structure containing the
cursor coordinates and the button data. Only two things are taken from it: the event type, and the
button data (which thumb button, which tilt direction). The coordinate field is not read anywhere in the
code. Button data says only "the back thumb button was pressed". Coordinates would say what you were
pointing at.

**One exception, outside the hook.** While the visual switcher overlay (§7) is open and the cursor moves
or clicks over that overlay, the overlay reads the cursor position to tell which card is being pointed
at. That position is compared with the card positions and then discarded. It is not stored and not
sent.

**Nothing is recorded**, on the same terms as the keyboard hook: not to the log, not to the debug trace,
and not anywhere that outlasts the event.

**Turning it off.** In Settings, on the Mouse tab, turn off "Enable Mouse Navigation". The button
mappings stop, and every mouse event is passed on without further processing. The hook stays installed
for as long as the daemon runs. Nothing else about the daemon changes.

## 7. Window Information

To choose a target for switching or arranging windows, the daemon reads the window class name, the
visibility and cloak state, and the executable file name of the owning process. All of this is read when
a shortcut is pressed, kept only for the duration of that operation, and never sent. Window titles are
not used to choose a target.

**Visual switcher.** When you hold the window-switching shortcut (default `` Win+` ``), the visual
switcher overlay, which is on by default, shows one card for each window of the same application. To
draw the cards, the overlay reads each of those windows' title and icon, and asks Windows to show a live
image of the window. That live image is drawn by Windows itself. Wira Desk does not read or copy its
pixels. The titles, icons, and images exist only while the overlay is open, and are not stored and not
sent.

Window titles can contain document names, chat contacts, or web addresses. If you do not want window
titles to be read, turn off "Enable Visual Switcher Overlay" in Settings, on the General tab.

## 8. What Is Stored on Disk

| Path | Contents | Retention |
| --- | --- | --- |
| `%APPDATA%\WiraDesk\config.toml` | Shortcuts, exception lists, switcher, snap, layout, and mouse settings, and the auto-start and update check state | Until you delete it or use "Reset all settings…" |
| `%APPDATA%\WiraDesk\wiradesk.log` | Timestamped warning lines: configuration problems (which setting and which shortcut), and the path of the executable when its location can be overwritten by a user who is not an administrator. It contains no keystrokes and no window content | Rotated automatically at 1 MB, with one prior generation kept as `wiradesk.log.old` (about 2 MB in total). Both are deleted only when you delete them |
| `%TEMP%\WiraDesk-update-<random>\WiraDesk-setup.exe` | The installer downloaded through "Download and install" (§3) | Not deleted by Wira Desk after the installer runs. Removed when you or Windows clean the temp folder. If the checksum does not match, the file is deleted immediately |

The files in `%APPDATA%\WiraDesk\` and `%TEMP%` are in your user profile, with ordinary user
permissions. Treat them as readable by any program running as you.

Release builds write no other file. The `wiradesk-debug-trace.log` file is created only by debug builds
used in development, and its code is not compiled into release builds.

**On uninstall.** The uninstaller asks whether the `%APPDATA%\WiraDesk\` folder should be deleted too.
Its default answer is "No", so your settings remain if you plan to reinstall. A silent uninstall, for
example through a package manager, never deletes that folder.

## 9. Reset

To return to the default settings, use "Reset all settings…" in Settings, on the About tab, or delete
`config.toml`. Deleting the whole `%APPDATA%\WiraDesk\` folder also restores the default settings, and
deletes the log as well.

## 10. Data on Our Side

Wira Desk has no account, no login, and no profile. The only data about Wira Desk use that reaches us is
the update check records in §2. All other data the product knows is in the files in §8, on your own
disk, under your control.

Under UU PDP, you have the right to request access to the update check records about you and to request
that they be deleted. Because the raw records hold only an IP address, a User-Agent, and a time, we can find
them only if you tell us the IP address and the approximate time. After 30 days, no record containing an
IP address remains. These records are processed outside Indonesia: our server is in Singapore, and
Cloudflare operates on its global network.

## 11. Third-Party Components

All network requests in §2 and §3 are made by a single Wira Desk module that uses WinHTTP, the HTTP
component built into Windows. The third-party libraries Wira Desk uses, with their versions and
licenses, are listed in `NOTICE` in the repository and in `NOTICE.txt` in the installation folder.

## 12. Changes to This Document

This document is published with the same content in `PRIVACY.md` in the Wira Desk repository and at
`wiradelta.com/wira-desk/privacy/`. The Indonesian original is in `PRIVACY.id.md` and at
`wiradelta.com/id/wira-desk/privacy/`. When the Wira Desk behavior described here changes, this document
is changed and the date at the top is updated. The change in behavior is recorded in `CHANGELOG.md`.

## 13. Questions

Questions about this document or about your data: `support@wiradelta.com`. For anything that looks like
a security problem, use the channel in the Wira Desk Security Policy, because that channel stays
private until a fix exists.

## 14. Language

This is an English translation of the Indonesian original. If the two differ in interpretation,
the Indonesian text prevails.
