# Wira Desk

> Schlanker, nativer Fensterwechsel für dieselbe App, Zonen-Snapping und treiberlose Mausnavigation für Windows 11 — geschrieben in Rust 🦀

[English](README.md) | [Bahasa Indonesia](README.id.md) | [简体中文](README.zh-CN.md) | [日本語](README.ja.md) | [한국어](README.ko.md) | [Español](README.es.md) | [Deutsch](README.de.md) | [Français](README.fr.md) | [Português (Brasil)](README.pt-BR.md) | [Русский](README.ru.md)  
[Website](https://wiradelta.id/wira-desk) | [Download](https://github.com/wiradeltaid/wira-desk/releases) | [Changelog](CHANGELOG.md) | [Contributing](CONTRIBUTING.md) | [License](LICENSE) | [Security](SECURITY.md) | [Privacy](PRIVACY.md)

---

> **Übersetzungshinweis:** Diese Datei ist eine Übersetzung von [README.md](README.md) und dient ausschließlich Informationszwecken. Bei Widersprüchen oder Auslegungsunterschieden ist die offizielle englische Originalfassung (`README.md`) maßgeblich. Alle tiefergehenden technischen Dokumentationen und rechtlichen Bedingungen werden auf Englisch geführt.

> **Wenn Sie PowerToys nur für FancyZones und Logi Options+ nur für die Daumentasten ausführen, ersetzt diese Anwendung beides — ein einzelner Tray-Prozess statt zweier speicherintensiver Hintergrunddienste.**
>
> Was nicht ersetzt wird: PowerRename, Awake, Farbauswahl, handgezeichnete FancyZones-Layouts; Logitech Flow, Profile pro Anwendung, Batterieüberwachung oder DPI-Umschaltung.

## Installation

### Über Scoop (Empfohlen)

```powershell
scoop bucket add wiradesk https://github.com/wiradeltaid/scoop-wiradesk
scoop install wiradesk
```

### Installationsprogramm (Setup Executable)

Laden Sie das Installationsprogramm (`WiraDesk-*-x64-setup.exe`) von der [Release-Seite](https://github.com/wiradeltaid/wira-desk/releases) (gespiegelt auf [SourceForge](https://sourceforge.net/projects/wira-desk/files/latest/download)) herunter und prüfen Sie den SHA-256-Hash:

```powershell
Get-FileHash .\WiraDesk-*-x64-setup.exe -Algorithm SHA256
```

Die Installation erfolgt mit Administratorrechten unter `%ProgramFiles%\Wira Desk`. Der Autostart ist optional und kann in den Einstellungen oder über das Tray-Icon konfiguriert werden.

### Standalone-Binärdateien (Portable)

Laden Sie die eigenständigen Binärdateien `wiradesk.exe` und `wiradesk-settings.exe` in ein Verzeichnis mit Administratorrechten herunter und führen Sie `wiradesk.exe` als Administrator aus.

---

## Hauptfunktionen

- **Fensterwechsel innerhalb derselben App (Same-App Window Cycling):** ``Win + ` `` wechselt ausschließlich zwischen den Fenstern der aktiven Anwendung auf dem aktuellen Monitor und virtuellen Desktop (Fallback: ``Alt + ` ``). Kurz antippen für sofortigen Wechsel, oder 300 ms gedrückt halten für das visuelle Vorschau-Overlay mit Live-Thumbnails.
- **Ein-Tasten-Zonen-Snapping:** Blitzschnelles Andocken von Fenstern an Bildschirmhälften (50%), Drittel (33%) oder benutzerdefinierte Kantenverhältnisse (Standard 67%, Oberkante Standard 33%) ohne Aufrufen eines Zonen-Editors.
- **Treiberlose Mausnavigation:** Weisen Sie die Daumentasten (`XBUTTON1`/`XBUTTON2`) und das Neigen des Mausrads ohne herstellereigene Bloatware dem Wechsel virtueller Desktops oder 20 Voreinstellungen zu.

### Standard-Tastaturkürzel

| Tastaturkürzel | Aktion |
|---|---|
| ``Win + ` `` | Fenster der aktiven App wechseln (300 ms halten für visuellen Switcher) |
| ``Alt + ` `` | Alternatives Tastaturkürzel für Fensterwechsel |
| `Strg+Alt+Links/Rechts/Oben/Unten` | Aktives Fenster an entsprechende Bildschirmhälfte (50%) andocken |
| `Strg+Alt+Umschalt+Links/Rechts/Oben/Unten` | Fenster mit benutzerdefiniertem Anteil an Kante andocken (67% Standard, Oben 33%) |
| `Strg+Alt+1/2/3` | Fenster an linkes, mittleres oder rechtes Drittel andocken |
| `Strg+Alt+Eingabe` | Fenster maximieren |
| `Strg+Alt+Umschalt+Eingabe` | Fenster auf den nächsten Monitor verschieben |
| `Strg+Alt+Umschalt+S` | 3 Fenster nebeneinander mit konfigurierbarer Breite anordnen |

### Maus-Voreinstellungen

Daumentasten wechseln standardmäßig zum vorherigen/nächsten virtuellen Desktop; das Neigen des Mausrads wechselt zu Desktop anzeigen / Task-Ansicht. Alle Aktionen können in den Einstellungen auf 20 Voreinstellungen angepasst werden. Cursor-Koordinaten werden niemals erfasst (siehe [`PRIVACY.md`](PRIVACY.md)).

---

## Warum Wira Desk

Windows bietet kein integriertes Wechseln zwischen den Fenstern derselben App. PowerToys, ein separater Download von Microsoft, hat in Version 0.101 Window Hopper ergänzt (standardmäßig deaktiviert), und Herstellertools übernehmen die Maustasten; zusammen laufen dafür mehrere Hintergrundprozesse mit 150–500 MB RAM. Wira Desk läuft als einzelner nativer Hintergrunddienst mit etwa 4.0 MB privatem Speicher (unter dem 5-MB-Budget) und ohne Telemetrie.

---

## Konfiguration & Entwicklung

- **Konfiguration:** Die Einstellungen liegen unter `%APPDATA%\WiraDesk\config.toml`. Eine vollständige Referenz finden Sie in [docs/CONFIGURATION.md](docs/CONFIGURATION.md).
- **Entwicklung:** Erstellt mit Rust und MSVC. Richtlinien zu Build, Tests und unsafe Code finden Sie in [DEVELOPMENT.md](DEVELOPMENT.md).
- **Mitwirkung:** Beiträge zum Open-Source-Projekt sind herzlich willkommen — siehe [CONTRIBUTING.md](CONTRIBUTING.md).

---

## Über das Projekt & Rechtliches

**Wira Delta Indonesia** ist das verantwortliche Software-Studio. Entwickelt und gepflegt von [@kodesh87](https://github.com/kodesh87).

- **Lizenz:** [GPL-3.0-only](LICENSE). Danksagungen und Lizenzen Dritter sind in [NOTICE](NOTICE) aufgeführt. Erstellt mit [Slint](https://slint.dev).
- **Datenschutz & Sicherheit:** Null Telemetrie, kein Benutzerkonto, kein stiller Update-Dienst. Siehe [PRIVACY.md](PRIVACY.md) und [SECURITY.md](SECURITY.md).
- **Name und Logo:** Die GPL gewährt Rechte am Code, nicht an Namen oder Markenzeichen. Die Namen **Wira Desk** und **Wira Delta Indonesia** sowie das Produkt-Icon verbleiben im Eigentum der PT Wira Delta Indonesia.
