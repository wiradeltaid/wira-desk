# Second Opinion Review Packet — SPEC-26: Header Defaults Buttons, Conditional Visibility, and About Card Hierarchy Polish

## 1. Drafted Spec and Tickets

- Spec: `.scratch/spec-26-header-defaults-and-about-card-hierarchy/SPEC.md`
- Ticket 1: `.scratch/spec-26-header-defaults-and-about-card-hierarchy/issues/01-pane-header-defaults-button-and-conditional-visibility.md`
- Ticket 2: `.scratch/spec-26-header-defaults-and-about-card-hierarchy/issues/02-about-pane-card-hierarchy-and-troubleshooting-redesign.md`
- Registry entry: `.control/registry/specs.yaml` (entry `SPEC-26`)

## 2. Original Raw Notes from Owner (Verbatim)

```
1. Ubah tombol `Restore shortcuts`, menjadi [`icon restore` lalu teks `defaults`]
2. Tambahkan [`icon restore` lalu teks `defaults`] juga di General dan di Mouse, hanya muncul kalua settingan2 yg ada di page tidak sesuai dengan default
3. `An open source utility by Wira Digital Indonesi` buat aja card baru paling Bawah sekali
- Area Troubleshooting & Recovery buat menyatu dengan card Support development, taruh di bagian bawahnya, di section terpisah tapi masih 1 card
- Tombol restore all preferences to defaults, jelek sekali lihat gambar - tidak sesuai desain system
4. Yang lain belum saya cek, akan dicek setelah perbaikan nomor 1 s/d 3 di atas


Settings / Shortcuts Pane (SPEC-25-01)
1. Header & Pembersihan Tip
   [x] Buka tab Shortcuts: pastikan tip teks usang (💡 Tip: Windows reserves Win + 1..9...) sudah tidak ada lagi di bawah judul.
   [x] Pastikan tombol sekunder [↺ Restore shortcuts] tampil di sisi kanan atas header Shortcuts.
2. Restore Shortcuts ke Default Pabrik
   [ ] Ubah salah satu shortcut (misal: siklus/snap) atau persentase, lalu klik [↺ Restore shortcuts].
   [ ] Pastikan seluruh tombol shortcut, toggle aktif/nonaktif, 4 edge percentage (67%), dan lebar stack kembali ke default pada draft.
   [ ] Pastikan pengaturan non-shortcut (General: auto-start/visual switcher, Mouse, dan VM exceptions) tidak ikut ter-reset.
   [ ] Klik Revert di footer bawah dan pastikan nilai kembali ke konfigurasi tersimpan sebelumnya.
3. Inline Legacy Stack / Snap Bottom Conflict Banner
   [ ] Pada file konfigurasi atau melalui UI, atur shortcut Overlapping stack dan Snap to bottom edge sama-sama aktif dengan chord Ctrl + Alt + Shift + Down.
   [ ] Buka tab Shortcuts: pastikan muncul banner peringatan kuning Update a conflicting shortcut dengan teks penjelasan.
   [ ] Jika shortcut Ctrl + Alt + Shift + S belum terpakai, pastikan tombol [Update shortcut] muncul dan saat diklik langsung memindahkan Overlapping Stack ke Ctrl + Alt + Shift + S serta menghilangkan banner konflik.
   [ ] Jika chord Ctrl + Alt + Shift + S sengaja dipasangkan ke aksi lain yang aktif, pastikan tombol satu-klik tidak muncul dan banner menjelaskan bahwa pengguna harus memilih shortcut unik secara manual.

Settings / About Pane (SPEC-25-02)
1. Baris Tiga Tombol Aksi Tautan
   [ ] Buka tab About: pastikan teks link gabungan lama sudah diganti menjadi baris 3 tombol horizontal yang rapi tanpa horizontal scroll pada lebar default.
   [ ] Klik tombol [Support development]: pastikan membuka browser ke https://wiradigital.id/wira-desk.
   [ ] Klik tombol [Issue Tracker]: pastikan membuka browser ke https://github.com/wiradigitalid/wira-desk/issues.
   [ ] Klik tombol icon [GitHub repository]: pastikan membuka browser ke https://github.com/wiradigitalid/wira-desk.
2. Kartu Troubleshooting & Modal Dialog Reset Pengaturan
   [ ] Scroll ke kartu Troubleshooting & Recovery di bagian bawah About pane: pastikan terdapat judul Restore all preferences to defaults.
   [ ] Klik tombol merah [Reset all settings…]: pastikan modal dialog konfirmasi muncul di tengah jendela dengan latar belakang gelap.
   [ ] Tekan tombol Escape pada keyboard atau klik tombol [Cancel]: pastikan dialog tertutup dan tidak ada pengaturan yang berubah.
   [ ] Buka kembali dialog, coba klik atau tab tombol Save/Revert/navigasi di latar belakang: pastikan interaksi latar belakang terblokir sepenuhnya.
   [ ] Klik tombol merah [Restore all preferences]: pastikan dialog tertutup, semua tab preferensi (General, Shortcuts, Mouse, VM) pada draft berubah ke nilai default pabrik, tombol Save Changes dan Revert di footer menjadi aktif (dirty).
   [ ] Klik Revert: pastikan preferensi kembali ke kondisi sebelum reset dilakukan tanpa ada perubahan pada file disk config.toml.
```

## 3. Standing Review Mandate & Instructions

If this draft touches the architecture spine, an SRS, an SDD, or a SPEC, you're authorized to run `wdi-review` on it yourself and edit the document directly to apply its stamp — no need to ask first, that permission is already given for this dispatch.

Note for `kiro-cli`: you have no native Skill tool. When the `wdi-review` mandate applies, open and follow `.claude/skills/wdi-review/SKILL.md` as plain instructions rather than attempting to invoke a skill by name.
The touched component is `settings`, whose review intensity in `.control/registry/components.yaml` is `risk_accepted: medium`. The lens set for `settings` is `[structure, prose, edge-case-hunter]`.

Please review SPEC-26 and its tickets against the original notes and existing codebase contracts (`crates/settings/ui/panes/general_pane.slint`, `shortcuts_pane.slint`, `mouse_pane.slint`, `about_pane.slint`, `crates/settings/src/app.rs`, and `crates/settings/src/main.rs`).
Verify acceptance criteria, edge cases (draft lifecycle, conditional button visibility derivations, design system subtle button styling, card layout), fix any gaps or inconsistencies in place in the spec and ticket files, and stamp `spec_reviewed` on SPEC-26 in `specs.yaml`.
