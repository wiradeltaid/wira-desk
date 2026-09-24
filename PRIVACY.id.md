# Kebijakan Privasi Wira Desk

<!-- Copied from the Wira Delta Indonesia legal source (wira-desk/privacy.id.md) on 2026-09-24.
     Edit the source, then copy it here again. -->

**Berlaku sejak:** 2026-09-24
**Berlaku untuk:** Wira Desk 0.3.0 dan sesudahnya

Wira Desk diterbitkan oleh **Wira Delta Indonesia** ("kami"). Naskah ini menjelaskan apa yang dibaca
Wira Desk di komputer Anda, apa yang disimpannya di disk, dan data apa yang sampai ke kami.

Wira Desk tidak berisi analitik, pelaporan crash, akun, atau ID perangkat. Dua hal di dalam produk
memakai jaringan: cek update, yang sampai ke server kami, dan unduhan installer, yang hanya terjadi
saat Anda memintanya. Keduanya diuraikan lengkap di §2 dan §3, bukan diringkas. Selebihnya tetap di
komputer Anda.

## 1. Istilah dalam Naskah Ini

- **Daemon**: proses Wira Desk yang berjalan di tray dan menjalankan shortcut (`wiradesk.exe`).
- **Settings**: jendela pengaturan Wira Desk (`wiradesk-settings.exe`).
- **Window**: jendela aplikasi di desktop Windows.
- **Hook**: mekanisme Windows yang membuat sebuah program menerima setiap event keyboard atau mouse
  di desktop sebelum event itu sampai ke aplikasi tujuannya.
- **Cek update**: permintaan yang menanyakan apakah ada versi Wira Desk yang lebih baru.
- **User-Agent**: kolom di setiap permintaan HTTPS yang menyebut program apa yang meminta.
- **Log**: file teks tempat Wira Desk mencatat peringatan.

## 2. Cek Update

Wira Desk bisa memeriksa apakah ada versi yang lebih baru. Sejak versi 0.3.0 permintaan ini dikirim
ke server kami di `wiradelta.id`, supaya kami bisa menghitung berapa salinan tiap versi yang masih
dipakai. Server itu sama dengan server situs `wiradelta.id`, dan menjawab permintaan ini sendiri tanpa
meneruskannya ke GitHub atau pihak lain.

**Kapan dikirim.** Daemon mengirim cek update pertama sekitar dua menit sesudah mulai berjalan, lalu
sekali setiap 24 jam selama ia berjalan. Tombol "Check for updates" di Settings mengirim permintaan
yang sama satu kali setiap Anda menekannya. Cek otomatis aktif secara default. Pilihan ini disengaja,
sebagai ganti bertanya sekali saat pertama kali Wira Desk dijalankan.

**Apa yang dikirim.** Satu permintaan HTTPS `GET` ke `https://wiradelta.id/api/v1/update/wira-desk/`. Header
User-Agent-nya berisi empat hal: nama produk, versi Wira Desk, versi Windows, dan arsitektur prosesor
(misalnya x64 atau ARM64). Selain keempat hal itu tidak ada yang dilampirkan: tidak ada nama komputer,
nama pengguna, konfigurasi, ID perangkat, atau penghitung.

**Apa yang tetap terungkap walaupun tidak dikirim.** Server kami menerima alamat IP asal permintaan dan
waktu permintaan, karena setiap permintaan di internet membawa keduanya. Alamat IP menunjukkan
perkiraan lokasi, dan bagi pengelola jaringan yang Anda pakai, perangkat tertentu. Lalu lintas ke
`wiradelta.id` melewati Cloudflare, yang melihat permintaan yang sama saat meneruskannya, menurut
kebijakan privasi Cloudflare sendiri.

**Yang kami simpan, dan berapa lama.** Server kami mencatat setiap cek update dengan tiga hal: alamat
IP, User-Agent (jadi juga versi Wira Desk, versi Windows, dan arsitektur), dan waktu. Sesudah 30 hari,
catatan mentah dihapus; yang tersisa hanya hitungan harian agregat berdasarkan data yang dikirim
aplikasi (versi aplikasi, versi Windows, arsitektur) dan perkiraan jumlah perangkat, tanpa alamat IP.
Kami memakai data ini hanya untuk mengetahui versi mana yang masih dipakai, di versi Windows dan
arsitektur apa, dan kira-kira di berapa perangkat. Kami tidak
memakainya untuk mengenali siapa Anda, tidak menggabungkannya dengan data lain, dan tidak memberikannya
kepada pihak lain. Dasar pemrosesannya adalah kepentingan yang sah, salah satu dasar pemrosesan dalam
Undang-Undang Nomor 27 Tahun 2022 tentang Pelindungan Data Pribadi (UU PDP). Anda bisa menghentikannya
kapan saja dengan mematikan cek update.

**Apa yang tidak dikirim, dan tidak mungkin dikirim.** Permintaan ini tidak membawa isi, jadi tidak
ada informasi tentang cara Anda memakai produk yang ikut terkirim: tidak ada shortcut yang Anda tekan,
tidak ada window yang terbuka, tidak ada lama daemon berjalan, dan tidak ada keterangan apakah Anda
pernah memeriksa sebelumnya.

**Cara mematikannya.** Di Settings, tab About, matikan "Check for updates automatically". Cek otomatis
berhenti sepenuhnya tanpa perlu menjalankan ulang daemon. Tombol "Check for updates" tetap ada, jadi
Anda bisa bertanya sekali tanpa membiarkan apa pun berjalan. Tidak ada yang berkurang: cek update
hanya memberi tahu bahwa ada versi baru, dan tidak mengunci fitur apa pun. Dengan cek otomatis mati
dan tombol tidak ditekan, Wira Desk tidak mengirim apa pun ke jaringan.

**Versi sebelum 0.3.0.** Wira Desk 0.2.4 dan sebelumnya mengirim cek update ke GitHub
(`github.com/wiradeltaid/wira-desk`), bukan ke server kami, dengan User-Agent yang hanya berisi nama
produk dan versi, misalnya `WiraDesk/0.2.4`.
Kami tidak menerima permintaan itu. GitHub melihatnya menurut kebijakan privasi GitHub sendiri.

## 3. Unduhan Installer

**Kapan dikirim.** Hanya saat Anda menekan "Download and install" di Settings, tab About, sesudah cek
update menemukan versi baru. Unduhan ini tidak pernah berjalan otomatis.

**Apa yang dikirim.** Satu permintaan HTTPS `GET` untuk file installer rilis itu di GitHub
(`github.com/wiradeltaid/wira-desk`), yang lalu dialihkan GitHub ke server penyimpanan filenya.
User-Agent-nya sama dengan cek update. Tidak ada lampiran lain. Installer tetap diunduh dari GitHub,
bukan dari server kami.

**Apa yang tetap terungkap walaupun tidak dikirim.** GitHub menerima alamat IP dan waktu unduhan,
menurut kebijakan privasi GitHub sendiri. Kami tidak menerima data unduhan ini.

**Apa yang terjadi sesudahnya.** Settings menyimpan installer di folder baru bernama acak di folder temp
Anda, mencocokkan checksum SHA-256-nya dengan checksum yang diterbitkan untuk rilis itu, lalu
menjalankannya. Bila tidak cocok, file dihapus dan tidak ada yang dijalankan. Windows menampilkan prompt
UAC karena installer membutuhkan hak Administrator.

**Bila Anda tidak ingin memakainya.** Jangan tekan tombolnya. Anda tetap bisa mengunduh installer
sendiri dari halaman rilis.

## 4. Tautan yang Dibuka di Browser

Beberapa tombol di Settings (catatan rilis, situs, kode sumber, laporan bug, dan bantuan) membuka
halaman di browser default Anda. Wira Desk hanya mengizinkan alamat di `wiradelta.id` dan
`github.com/wiradeltaid/wira-desk`. Kunjungan itu dilakukan browser Anda, bukan Wira Desk, dan diatur
kebijakan privasi situs yang dibuka. Untuk `wiradelta.id`, itu Kebijakan Privasi situs kami.

## 5. Ketikan Keyboard

Daemon memasang hook keyboard global tingkat rendah (`WH_KEYBOARD_LL`) supaya shortcut bekerja dari
window mana pun. Karena itu daemon menerima setiap event tombol di desktop. Windows tidak menyediakan
cara yang lebih sempit untuk shortcut global.

Dari setiap event, daemon membaca kode tombol virtual (virtual-key code) dan penanda input sintetis,
mencocokkannya dengan shortcut yang Anda atur, dan **tidak mencatat apa pun**: tidak ke log, tidak ke
debug trace, dan tidak ke tempat mana pun yang bertahan sesudah event selesai. Hal ini bisa diperiksa:
tidak ada pemanggilan log di kode yang menerima nilai tombol virtual sebagai argumen.

Daemon juga mengirim satu tombol sintetis, `VK_NONAME` (kode yang tidak dipakai apa pun di Windows),
supaya menu Start tidak terbuka sesudah shortcut yang memakai tombol Windows.

## 6. Event Mouse

Daemon juga memasang hook mouse global tingkat rendah (`WH_MOUSE_LL`) untuk navigasi mouse: tombol
jempol (thumb button) dan tilt wheel. Hook ini terpasang selama daemon berjalan, dan navigasi mouse
aktif secara default. Jadi daemon menerima setiap event mouse di desktop, dan yang dilakukannya lebih
sempit daripada kedengarannya.

**Gerakan kursor langsung diteruskan.** Event gerak kursor (`WM_MOUSEMOVE`) dikembalikan di baris
pertama callback, sebelum kunci, alokasi, atau pembacaan lain. Sebagian besar event mouse adalah
gerakan, dan tidak satu pun dilihat.

**Hook tidak membaca posisi kursor.** Windows memberi callback sebuah struktur berisi koordinat kursor
dan data tombol. Yang diambil hanya dua: jenis event, dan data tombol (tombol jempol mana, arah tilt
mana). Kolom koordinat tidak dibaca di mana pun di kode. Data tombol hanya menyatakan "tombol jempol
belakang ditekan". Koordinat akan menyatakan apa yang sedang Anda tunjuk.

**Satu pengecualian, di luar hook.** Saat overlay visual switcher (§7) terbuka dan kursor bergerak atau
diklik di atas overlay itu, overlay membaca posisi kursor untuk mengetahui kartu mana yang ditunjuk.
Posisi itu dibandingkan dengan letak kartu lalu dibuang. Posisi itu tidak disimpan dan tidak dikirim.

**Tidak ada yang dicatat**, dengan ketentuan yang sama seperti hook keyboard: tidak ke log, tidak ke
debug trace, dan tidak ke tempat mana pun yang bertahan sesudah event selesai.

**Cara mematikannya.** Di Settings, tab Mouse, matikan "Enable Mouse Navigation". Pemetaan tombol
berhenti, dan setiap event mouse diteruskan tanpa diproses lebih lanjut. Hook tetap terpasang selama
daemon berjalan. Selain itu tidak ada yang berubah pada daemon.

## 7. Informasi Window

Untuk memilih target berpindah atau menata window, daemon membaca class name window, status terlihat
dan status cloak, serta nama file executable proses pemiliknya. Semua itu dibaca saat shortcut
ditekan, disimpan hanya selama operasi itu, dan tidak pernah dikirim. Judul window tidak dipakai untuk
menentukan target.

**Visual switcher.** Saat Anda menahan shortcut berpindah window (default `` Win+` ``), overlay visual
switcher, yang aktif secara default, menampilkan satu kartu untuk setiap window dari aplikasi yang
sama. Untuk menggambar kartu, overlay membaca judul dan ikon setiap window itu, dan meminta Windows
menampilkan gambar live window tersebut. Gambar live itu digambar langsung oleh Windows. Wira Desk
tidak membaca atau menyalin pikselnya. Judul, ikon, dan gambar hanya ada selama overlay terbuka, tidak
disimpan, dan tidak dikirim.

Judul window bisa memuat nama dokumen, kontak chat, atau alamat web. Bila Anda tidak ingin judul window
dibaca, matikan "Enable Visual Switcher Overlay" di Settings, tab General.

## 8. Yang Disimpan di Disk

| Path | Isi | Retensi |
| --- | --- | --- |
| `%APPDATA%\WiraDesk\config.toml` | Shortcut, daftar pengecualian, pengaturan switcher, snap, layout, dan mouse, serta status auto-start dan cek update | Sampai Anda menghapusnya atau memakai "Reset all settings…" |
| `%APPDATA%\WiraDesk\wiradesk.log` | Baris peringatan bertanggal: masalah konfigurasi (pengaturan dan shortcut yang bermasalah), dan path file executable bila lokasinya bisa ditimpa oleh pengguna yang bukan administrator. Tidak berisi ketikan atau isi window | Dirotasi otomatis pada 1 MB, dengan satu generasi sebelumnya disimpan sebagai `wiradesk.log.old` (total sekitar 2 MB). Keduanya baru terhapus bila Anda menghapusnya |
| `%TEMP%\WiraDesk-update-<acak>\WiraDesk-setup.exe` | Installer yang diunduh lewat "Download and install" (§3) | Tidak dihapus Wira Desk sesudah installer dijalankan. Hilang saat Anda atau Windows membersihkan folder temp. Bila checksum tidak cocok, file dihapus saat itu juga |

File di `%APPDATA%\WiraDesk\` dan `%TEMP%` ada di profil pengguna Anda dengan izin pengguna biasa.
Anggaplah file itu bisa dibaca oleh program apa pun yang berjalan sebagai Anda.

Build rilis tidak menulis file lain. File `wiradesk-debug-trace.log` hanya dibuat oleh build debug
untuk pengembangan, dan kodenya tidak ikut dikompilasi ke build rilis.

**Saat uninstall.** Uninstaller menanyakan apakah folder `%APPDATA%\WiraDesk\` ikut dihapus. Jawaban
default-nya "No", jadi pengaturan Anda tetap ada bila Anda berencana memasang ulang. Uninstall tanpa
tampilan (silent), misalnya lewat package manager, tidak pernah menghapus folder itu.

## 9. Reset

Untuk kembali ke pengaturan default, pakai "Reset all settings…" di Settings, tab About, atau hapus
`config.toml`. Menghapus seluruh folder `%APPDATA%\WiraDesk\` juga mengembalikan pengaturan default,
dan ikut menghapus log.

## 10. Data di Pihak Kami

Wira Desk tidak punya akun, login, atau profil. Satu-satunya data tentang pemakaian Wira Desk yang
sampai ke kami adalah catatan cek update di §2. Seluruh data lain yang diketahui produk ada di file
pada §8, di disk Anda sendiri, di bawah kendali Anda.

Sesuai UU PDP, Anda berhak meminta akses ke catatan cek update tentang Anda dan meminta catatan itu
dihapus. Karena catatan mentah hanya berisi alamat IP, User-Agent, dan waktu, kami hanya bisa menemukannya
bila Anda memberi tahu alamat IP dan perkiraan waktunya. Sesudah 30 hari tidak ada lagi catatan yang
memuat alamat IP. Catatan ini diproses di luar Indonesia: server kami ada di Singapura, dan Cloudflare
bekerja di jaringan globalnya.

## 11. Komponen Pihak Ketiga

Semua permintaan jaringan di §2 dan §3 dibuat oleh satu modul Wira Desk yang memakai WinHTTP, komponen
HTTP bawaan Windows. Pustaka pihak ketiga yang dipakai Wira Desk, beserta versi dan lisensinya,
tercantum di `NOTICE` di repo dan di `NOTICE.txt` di folder instalasi.

## 12. Perubahan Naskah

Naskah ini diterbitkan dengan isi yang sama di `PRIVACY.id.md` di repo Wira Desk dan di
`wiradelta.id/id/wira-desk/privacy/`. Terjemahan bahasa Inggrisnya ada di `PRIVACY.md` dan di
`wiradelta.id/wira-desk/privacy/`. Bila perilaku Wira Desk yang disebut di sini berubah, naskah ini
diubah dan tanggal di bagian atas diperbarui. Perubahan perilakunya dicatat di `CHANGELOG.md`.

## 13. Pertanyaan

Pertanyaan tentang naskah ini atau tentang data Anda: `support@wiradelta.id`. Untuk apa pun yang
tampak seperti masalah keamanan, pakai jalur di Kebijakan Keamanan Wira Desk, karena jalur itu
tertutup sampai ada perbaikan.

## 14. Bahasa

Bahasa. Naskah ini dibuat dalam bahasa Indonesia dan diterjemahkan ke bahasa Inggris. Bila terdapat
perbedaan tafsir antara keduanya, naskah bahasa Indonesia yang berlaku.
