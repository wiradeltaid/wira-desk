# Review Packet — Second Opinion for SPEC-16

## 1. Drafted Spec and Ticket Paths
- Spec: `.scratch/spec-16-visual-switcher-lifecycle-and-settings-polish/SPEC.md`
- Tickets:
  - `.scratch/spec-16-visual-switcher-lifecycle-and-settings-polish/issues/01-settings-stepper-centering-and-em-dash-removal.md`
  - `.scratch/spec-16-visual-switcher-lifecycle-and-settings-polish/issues/02-start-menu-suppression-on-visual-switcher-commit.md`
  - `.scratch/spec-16-visual-switcher-lifecycle-and-settings-polish/issues/03-modern-winui-popup-bridge-and-helper-surface-exclusion.md`
- Registry entry: `.control/registry/specs.yaml` (entry `SPEC-16`)

## 2. Original Raw Notes from Owner (Unedited)

```
Catatan:
1. Height dari fitur Hold Delay Threshold sudah cukup tinggi, tapi area bagian Bawah sepertinya agak ketinggian sedikit saja (sangat sedikit saja). Lalu jugfa untuk stepper itu maunya dia middle secara vertical atas height dari item (section) fitur ini
2. Hapus seluruh em-dash dari seluruh visual aplikasi
3. Ketika pakai WIN+` lama, dan setelah selesai memilih, si start menu otomatis terbuka (tidak benar). tapi kalua pakai cancel escape dia gak terbuka (sudah benar). Hal ini berlaku berbarengan juga dengan glitch tidak satabilnya perpindahan switcher. kadang bisa kadang tidak bisa - pas tidak bisa itulah dia muncul start menu. Kalau on click pada switcher dia berhasil, tapi kalau pakai keyboard kadang bisa kadang tidak bisa (ketika saya release window button). Kalau Alt+` aman2 saja.

Settings / General Pane
  1. Hold Delay Layout & Spacing (SPEC-15-01)
     [x] Buka pane General dan pastikan teks `(100–500 ms)` berada di baris baru tersendiri di bawah keterangan Hold Delay Threshold.
     [x] Pastikan baris Hold Delay memiliki ruang vertikal yang cukup (padding 16px ke bawah) dan tidak mepet dengan garis batas kartu.

Settings / About Pane
  2. GitHub Disclosure Text Formatting (SPEC-15-01)
     [x] Buka pane About dan periksa bagian paling bawah (Card 3).
     [x] Pastikan teks `update checks run entirely in-process against GitHub Releases, which you can switch off.` dimulai rapi pada baris baru (setelah tanda em-dash `—`).

Visual Switcher / Overlay Presentation
  3. Active Window Inclusion (SPEC-15-03)
     [x] Tekan dan tahan shortcut cycle (misal `Win + ~`) hingga overlay visual switcher muncul.
     [x] Pastikan jendela yang sedang aktif saat ini muncul sebagai kartu pertama (indeks 0), diikuti oleh jendela lainnya.
     [x] Pastikan highlight seleksi otomatis berada pada kartu kedua (indeks 1, jendela berikutnya) saat entry maju (forward).
     [x] Lepaskan tombol modifier dan pastikan jendela yang ter-highlight langsung aktif menjadi fokus.
     [x] Uji pembatalan: tekan dan tahan hotkey, lalu tekan tombol Escape — pastikan overlay tertutup dan jendela awal tetap aktif.

  4. Backward Entry Parity (SPEC-15-03)
     [x] Tekan dan tahan shortcut cycle mundur (`Shift + Win + ~`).
     [x] Pastikan highlight seleksi otomatis berada pada kartu terakhir (indeks `len - 1`), bukan pada kartu jendela aktif.

  5. Single-Window Desktop Triggering (SPEC-15-03)
     [x] Pindah ke desktop/workspace yang hanya memiliki 1 jendela dari aplikasi tersebut.
     [x] Tekan dan tahan shortcut cycle: pastikan overlay visual switcher tetap berhasil terbuka dan menampilkan 1 kartu tersebut (tidak lagi gagal/macet).

  6. Helper & Popup Surface Exclusion (SPEC-15-02)
     [x] Tahan shortcut cycle untuk memunculkan visual switcher.
     [ ] Pastikan helper surface seperti `PopupHost` atau `Xaml_WindowedPopupClass` tidak muncul sebagai kartu hantu di dalam overlay. -> masih muncul, terutama di notepad

Keyboard Lifecycle & Timing
  7. Decoupled Timing & Zero Premature Jump (SPEC-15-04)
     [x] Lakukan tap cepat pada shortcut cycle (tekan lalu lepas sebelum 150 ms): pastikan jendela langsung berpindah (blind cycling) tanpa ada lag.
     [x] Lakukan penahanan shortcut (hold past threshold): perhatikan jendela saat hotkey ditekan pertama kali — pastikan jendela TIDAK berpindah tempat sebelum overlay terbuka; overlay harus terbuka dengan jendela awal tetap berada di posisinya.

  8. Keyboard Auto-Repeat Suppression (SPEC-15-04)
     [x] Tahan shortcut cycle secara terus-menerus melampaui waktu munculnya overlay.
     [x] Pastikan auto-repeat keyboard tidak membuat seleksi kartu berputar kencang tak terkendali (~31 kali per detik); seleksi harus tetap diam di kartu awal sampai tombol utama ditekan ulang secara manual.
```

## 3. Standing Mandate
*"If this draft touches the architecture spine, an SRS, an SDD, or a SPEC, you're authorized to run `wdi-review` on it yourself and edit the document directly to apply its stamp — no need to ask first, that permission is already given for this dispatch."*

## 4. Context & Investigation Findings
- Modern Windows 11 `Notepad.exe` live WinProbe revealed helper surfaces have window class `Microsoft.UI.Content.PopupWindowSiteBridge` and window text `Pop-upHost`, owned by the main Notepad window (`GetWindow(hwnd, GW_OWNER) != 0`).
- Start Menu popping up on Win release during visual switcher hold is caused by `suppress_start_menu()` (which injects `VK_NONAME` while Win is held) not running because keydown cycle was decoupled in SPEC-15-04. Windows sees an uninterrupted Win-down and Win-up with Backtick swallowed by the hook, treating it as a lone Windows key tap.
- Hold delay stepper centering and padding reduction (`16px` -> `12px`) in `general_pane.slint`.
- Removal of em-dashes `—` across all visual UI in `crates/settings/ui/`.

Perform a critical review of SPEC-16 and tickets, verify against the codebase, identify any edge cases or gaps, and apply the `spec_reviewed` stamp directly in `.control/registry/specs.yaml`.
