# Kebijakan Keamanan Wira Desk

<!-- Copied from the Wira Delta Indonesia legal source (wira-desk/security.id.md) on 2026-09-24.
     Edit the source, then copy it here again. -->

**Berlaku sejak:** 2026-09-24
**Berlaku untuk:** Wira Desk 0.3.0 dan sesudahnya

Wira Desk berjalan dengan hak Administrator, memasang hook keyboard dan mouse global tingkat rendah
(`WH_KEYBOARD_LL`, `WH_MOUSE_LL`), bisa mendaftarkan tugas logon yang menjalankannya dengan hak
Administrator tanpa prompt, membaca daftar window tingkat atas (termasuk judulnya, untuk visual
switcher), dan bisa mengunduh lalu menjalankan installer versi baru saat Anda memintanya. Sifat-sifat
itu layak diperiksa. Karena itu `docs/threat-model.md` di repo menjelaskan batas kepercayaan, alasan
setiap hak akses, dan risiko yang tersisa sesudah mitigasi. Baca dokumen itu lebih dulu bila Anda
sedang menilai apakah software (perangkat lunak) ini layak dipercaya.

## 1. Istilah dalam Naskah Ini

- **Daemon**: proses Wira Desk yang berjalan di tray dan menjalankan shortcut (`wiradesk.exe`).
- **Settings**: jendela pengaturan Wira Desk (`wiradesk-settings.exe`), yang berjalan tanpa hak
  Administrator.
- **Window**: jendela aplikasi di desktop Windows.
- **Hook**: mekanisme Windows yang membuat sebuah program menerima setiap event keyboard atau mouse
  di desktop sebelum event itu sampai ke aplikasi tujuannya.
- **Auto-start**: tugas di Task Scheduler Windows yang menjalankan daemon saat Anda logon.
- **Checksum**: sidik jari SHA-256 sebuah file. File yang berubah satu byte pun punya checksum lain.
- **UAC**: prompt Windows yang meminta izin sebelum sebuah program mendapat hak Administrator.

## 2. Tiga Fakta yang Ingin Diketahui Lebih Dulu

- **Isi ketikan tidak dicatat.** Hook keyboard membaca kode tombol virtual untuk mencocokkan shortcut
  yang Anda atur, dan tidak menulisnya ke disk, ke log, atau ke debug trace. Tidak ada pemanggilan log
  di kode yang menerima nilai tombol sebagai argumen.
- **Hook mouse tidak membaca posisi kursor, dan aktivitas mouse tidak dicatat.** Hook mouse ada untuk
  pemetaan tombol jempol dan tilt wheel, yang aktif secara default. Gerakan kursor dikembalikan di baris
  pertama callback, sebelum kunci atau alokasi apa pun. Dari struktur event yang diberikan Windows, yang
  diambil hanya jenis event dan data tombol. **Kolom koordinat tidak dibaca di mana pun di kode.** Satu
  pengecualian ada di luar hook: overlay visual switcher membaca posisi kursor hanya saat kursor berada
  di atas overlay itu, untuk mengetahui kartu mana yang ditunjuk, lalu membuangnya.
- **Tidak ada analitik, akun, atau layanan updater terpisah.** Cek update berjalan di dalam proses
  daemon. Wira Desk membuat dua jenis permintaan HTTPS keluar:
  1. Cek update ke `https://wiradelta.com/api/v1/update/wira-desk/`, otomatis sekali sehari atau saat Anda memintanya.
     Header User-Agent-nya menyebut nama produk, versi Wira Desk, versi Windows, dan arsitektur
     prosesor, tanpa data lain tentang komputer, pengguna, atau konfigurasi. Endpoint ini berjalan di
     server yang sama dengan situs `wiradelta.com`, di belakang Cloudflare, dan menjawab sendiri tanpa
     mengalihkan ke GitHub. Server kami mencatat alamat IP, User-Agent, dan waktu. Sesudah 30 hari,
     catatan mentah dihapus; yang tersisa hanya hitungan harian agregat (versi aplikasi, versi
     Windows, arsitektur) dan perkiraan jumlah perangkat, tanpa alamat IP. Cek otomatis aktif
     secara default dan bisa dimatikan sepenuhnya di Settings.
  2. Unduhan installer dari GitHub (`github.com/wiradeltaid/wira-desk`), yang hanya terjadi saat Anda
     menekan "Download and install" di Settings. Installer dicocokkan dengan checksum SHA-256 yang
     diterbitkan sebelum dijalankan.

  Konfigurasi dan log tetap di `%APPDATA%\WiraDesk\`. Kebijakan Privasi Wira Desk menguraikan kedua
  permintaan itu baris per baris.

## 3. Melaporkan Kerentanan

Pakai **GitHub Security Advisories** di repo Wira Desk ("Report a vulnerability" di tab Security),
supaya laporan tetap tertutup sampai ada perbaikan. Jangan membuka issue publik untuk dugaan
kerentanan.

Yang membantu dalam laporan: build Windows, versi Wira Desk, apa yang Anda lakukan, apa yang terjadi,
dan, bila ada, cara mereproduksi yang paling sederhana. Tidak ada program bounty dan tidak ada jaminan
waktu tanggap. Ini proyek kecil, dan jujur soal itu lebih berguna daripada janji yang tidak bisa
ditepati.

**Dalam cakupan:**

- apa pun yang membuat pengguna yang bukan administrator mendapat hak lebih tinggi;
- apa pun yang membaca data yang seharusnya tidak dibuka daemon;
- apa pun yang membuat daemon, yang berjalan dengan hak Administrator, bertindak atas input yang
  seharusnya tidak dipercaya.

**Di luar cakupan, didokumentasikan alih-alih diperbaiki:**

- penyerang yang sudah menjadi administrator di komputer itu;
- mematikan shortcut lewat perebutan sumber daya (misalnya mutex). Itu hanya menghentikan fitur
  kemudahan, tanpa menambah hak;
- risiko sisa di §9 dan di `docs/threat-model.md`. Itu kompromi yang sudah diketahui, bukan bug yang
  belum dilaporkan.

**Versi yang didukung.** Hanya rilis terbaru yang mendapat perbaikan. Tidak ada cabang dukungan jangka
panjang. Ini proyek kecil, bukan produk komersial dengan SLA.

## 4. Integritas Rilis

**File rilis belum ditandatangani kode (code signing).** Akibatnya ada tiga, dan semuanya terlihat
oleh Anda:

- Windows SmartScreen memperingatkan saat pertama kali dijalankan. Karena daemon butuh hak
  Administrator, prompt UAC menampilkan penerbit yang tidak terverifikasi. Itu wajar untuk build yang
  tidak ditandatangani dan bukan bukti file diubah orang. Tetapi itu juga berarti prompt tersebut tidak
  bisa membantu Anda membedakan file asli dari file palsu.
- Di komputer yang menyalakan **Smart App Control**, daemon sama sekali tidak bisa berjalan. Berbeda
  dengan SmartScreen, tidak ada jalan untuk melewatinya: Smart App Control menilai file itu sendiri, dan
  file tanpa tanda tangan yang belum dikenal layanan reputasi mana pun ditolak, hanya dengan satu
  catatan CodeIntegrity di event log. Tidak ada pengaturan di Wira Desk yang mengubahnya.
- Setiap rilis menerbitkan file `SHA256SUMS`. Cocokkan checksum unduhan Anda dengan `Get-FileHash`
  sebelum menjalankannya. Checksum yang disajikan dari tempat yang sama dengan unduhannya membuktikan
  file sampai utuh, **bukan** siapa yang membuatnya.

Verifikasi terkuat yang tersedia hari ini adalah **membangun sendiri dari kode sumber** di repo, dan
itu alasan seluruh kode sumber diterbitkan, bukan hanya file jadinya. Penandatanganan kode adalah
perbaikan yang sebenarnya, dan belum ada. Sampai ada, anggap setiap file "Wira Desk" dari tempat selain
halaman rilis repo ini tidak tepercaya.

## 5. Updater dan Apa yang Memverifikasinya

- Deskriptor rilis (file kecil yang menyebut versi terbaru, alamat installer, dan checksum-nya) diambil
  dari `https://wiradelta.com/api/v1/update/wira-desk/` lewat HTTPS saja. Server kami menyajikan deskriptor itu sendiri,
  tanpa pengalihan ke GitHub. Tidak ada jalur kode yang bisa mengambilnya lewat HTTP biasa, dan
  deskriptor yang lebih besar dari 64 KB ditolak.
- Alamat installer di dalam deskriptor harus HTTPS dan harus berada tepat di host dan repo
  `github.com/wiradeltaid/wira-desk`. Installer tetap diunduh dari GitHub Releases, bukan dari server
  kami. Alamat lain ditolak, jadi deskriptor yang dirusak paling jauh hanya membuat update gagal.
- Unduhan dilakukan oleh Settings, yang berjalan tanpa hak Administrator, dengan batas 192 MB. Checksum
  SHA-256 dihitung saat file ditulis dan dicocokkan dengan checksum di deskriptor. Bila tidak cocok,
  file dihapus dan tidak ada yang dijalankan.
- Installer dijalankan lewat Windows Shell, sehingga Windows selalu menampilkan prompt UAC. Wira Desk
  tidak pernah menaikkan hak secara diam-diam.
- Alamat deskriptor bisa diganti lewat variabel lingkungan hanya di build debug untuk pengembangan.
  Kode itu tidak ikut dikompilasi ke build rilis, jadi tidak ada pengaturan di salinan yang Anda pasang
  yang bisa mengalihkan tempat installer diambil.

Selama file rilis belum ditandatangani, checksum dan HTTPS adalah seluruh verifikasinya. Deskriptor dan
installer diterbitkan oleh proses rilis yang sama, jadi kecocokan checksum membuktikan unduhan sama
dengan yang diterbitkan, bukan siapa yang menerbitkannya.

## 6. Alasan Wira Desk Butuh Hak Administrator

Hak Administrator ada untuk satu tujuan: mengaktifkan dan memindahkan window milik proses dengan
tingkat integritas lebih tinggi, yang kalau tidak akan diblokir Windows (User Interface Privilege
Isolation, UIPI). Tanpa hak itu, daemon bisa menata editor teks Anda, tetapi tidak terminal
Administrator di sebelahnya.

Hak itu **tidak** dipakai untuk membaca memori proses lain. Daemon membuka proses dengan
`PROCESS_QUERY_LIMITED_INFORMATION`, tidak pernah `PROCESS_VM_READ`. Manifest bukan satu-satunya
pemeriksaan: daemon memeriksa ulang token-nya sendiri saat mulai dan menolak berjalan tanpa hak
Administrator.

## 7. Panduan Pengerasan

Dua hal ini lebih penting daripada apa pun di halaman ini:

- **Pasang di folder yang hanya bisa ditulis administrator**, misalnya `%ProgramFiles%`. Tugas
  auto-start menjalankan daemon dengan hak Administrator di setiap logon tanpa prompt, jadi siapa pun
  yang bisa menimpa file executable di path itu mendapat pijakan Administrator tanpa prompt. Tugas itu
  menyimpan path absolut dan tidak menetapkan folder kerja, jadi path-nya sendiri tidak bisa dibajak;
  yang melindunginya adalah izin file.
- **Jangan nyalakan auto-start dari build di `Downloads`, `Desktop`, atau folder lain yang bisa ditulis
  pengguna biasa**, dengan alasan yang sama. Bila Anda memakai versi portable (zip), ekstrak dulu ke
  folder yang hanya bisa ditulis administrator.

Kedua hal itu kini **diperiksa, bukan hanya diminta**, dan perlu jelas apa artinya:

- Daemon membaca izin file executable-nya sendiri dan izin folder tempatnya berada. Bila ada pihak yang
  bukan administrator memegang hak untuk mengganti salah satunya, dan auto-start terdaftar, Anda
  mendapat peringatan: satu baris di `wiradesk.log` dan titik peringatan di ikon tray. **Daemon
  memperingatkan, tidak menolak.** Auto-start tetap menyala, karena pemeriksaan yang memblokir
  jalannya build dari folder build akan dimatikan orang, bukan dipatuhi, dan pilihan itu tetap milik
  Anda untuk diambil dengan sadar. Tidak adanya peringatan di sini tidak berarti aman untuk hal lain
  selain pertanyaan yang satu ini.
- Path yang tersimpan tidak lagi basi. Karena tugas logon menyimpan path absolut yang dibekukan saat
  auto-start dinyalakan, memindahkan file executable dulu membuat tugas itu tetap menunjuk lokasi lama.
  Akibatnya, memasang dengan benar *sesudah* pertama kali menjalankan dari `Downloads` membuat file di
  `Downloads` itu yang dijalankan Windows dengan hak Administrator. Kini daemon mengarahkan ulang tugas
  itu ke dirinya sendiri setiap kali mulai.

Keduanya bukan pengganti memasang di tempat yang benar. Keduanya memberi tahu bila Anda belum
melakukannya.

Installer adalah bagian ketiga, dan bagian yang menjadikan tempat yang benar sebagai default: installer
butuh hak Administrator, memasang ke `%ProgramFiles%\Wira Desk`, dan **tidak menawarkan lokasi
pemasangan per pengguna**. Installer yang menawarkan `%LOCALAPPDATA%` sama saja menawarkan jalur
eskalasi di atas sebagai kemudahan, jadi pilihan itu tidak ada. Installer juga tidak menyalakan
auto-start. Mendaftarkan tugas logon Administrator tanpa prompt adalah keputusan yang tetap di tangan
Anda.

Uninstall menghapus tugas terjadwal itu. Tugas `ONLOGON` dengan `/RL HIGHEST` yang bertahan lebih lama
daripada file executable yang ditunjuknya adalah hal terburuk yang bisa ditinggalkan uninstaller.

## 8. Catatan Desain

- Callback hook dibatasi sejak rancangannya: tidak ada alokasi heap, kunci, I/O file, atau log di jalur
  callback.
- `SetDllDirectoryW` dijalankan sebagai pernyataan pertama di `main` untuk mengeluarkan folder saat ini
  dari urutan pencarian DLL, sehingga DLL yang ditanam orang tidak bisa dimuat dengan hak daemon.
- Pemuatan ulang konfigurasi memakai pesan `WM_APP` yang eksplisit dan berlaku utuh atau tidak sama
  sekali: file yang tidak terbaca, rusak, atau tidak valid membuat konfigurasi sebelumnya tetap berlaku
  dan menghasilkan satu peringatan. **Tidak ada nilai konfigurasi yang pernah menjadi path atau baris
  perintah.**
- Setiap blok `unsafe` membawa komentar `SAFETY:` yang menyebut prasyarat yang diandalkannya, dan
  compiler memaksakannya: `undocumented_unsafe_blocks` dan `missing_safety_doc` diatur `deny` di lint
  workspace, jadi blok tanpa dokumentasi menggagalkan build.
- Dependensi dijaga di CI oleh `cargo-deny` (advisory, lisensi, larangan, dan sumber), dan repo dipindai
  `gitleaks` di seluruh riwayat git dan di working tree.

## 9. Risiko Sisa yang Diketahui

Dicantumkan, bukan disembunyikan:

- Auto-start dari folder yang bisa ditulis pengguna biasa tetap menjadi jalur eskalasi tanpa prompt
  bila Anda melanjutkan melewati peringatan di §7.
- Penulisan `config.toml` yang disengaja oleh pihak jahat bisa memengaruhi perilaku daemon yang berjalan
  dengan hak Administrator, terbatas pada kolom bertipe tanpa path atau eksekusi perintah.
- File rilis belum ditandatangani (§4).
- Di antara saat checksum installer dicocokkan dan saat Windows menjalankannya, program lain yang
  berjalan sebagai pengguna yang sama bisa mengganti file di folder temp. Celah ini kecil, dan baru
  tertutup benar dengan pemeriksaan tanda tangan sesudah file rilis ditandatangani. Risikonya tidak
  lebih buruk daripada mengunduh installer sendiri lalu menjalankannya.
- Antarmuka COM untuk virtual desktop dideklarasikan tangan dan hanya diuji minimal.
- Shortcut bisa dimatikan lewat perebutan mutex atau penyalahgunaan daftar pengecualian, tanpa
  menambah hak.

Analisis lengkapnya, termasuk batas kepercayaan dan alasan setiap hak akses, ada di
`docs/threat-model.md`.

## 10. Perubahan Naskah

Naskah ini diterbitkan dengan isi yang sama di `SECURITY.id.md` di repo Wira Desk dan di
`wiradelta.com/id/wira-desk/security/`. Terjemahan bahasa Inggrisnya ada di `SECURITY.md` dan di
`wiradelta.com/wira-desk/security/`. Bila perilaku yang disebut di sini berubah, naskah ini diubah dan
tanggal di bagian atas diperbarui.

## 11. Bahasa

Bahasa. Naskah ini dibuat dalam bahasa Indonesia dan diterjemahkan ke bahasa Inggris. Bila terdapat
perbedaan tafsir antara keduanya, naskah bahasa Indonesia yang berlaku.
