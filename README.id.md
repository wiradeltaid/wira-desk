# Wira Desk

> Perpindahan jendela satu aplikasi yang ringan dan native, zone snapping, dan navigasi mouse tanpa driver untuk Windows 11 — ditulis dengan Rust 🦀

[English](README.md) | [Bahasa Indonesia](README.id.md) | [简体中文](README.zh-CN.md) | [日本語](README.ja.md) | [한국어](README.ko.md) | [Español](README.es.md) | [Deutsch](README.de.md) | [Français](README.fr.md) | [Português (Brasil)](README.pt-BR.md) | [Русский](README.ru.md)  
[Website](https://wiradelta.id/wira-desk) | [Download](https://github.com/wiradeltaid/wira-desk/releases) | [Changelog](CHANGELOG.md) | [Contributing](CONTRIBUTING.md) | [License](LICENSE) | [Security](SECURITY.md) | [Privacy](PRIVACY.md)

---

> **Pemberitahuan terjemahan:** Berkas ini merupakan terjemahan dari [README.md](README.md) untuk kenyamanan pembaca. Jika terdapat perbedaan makna atau penafsiran, berkas resmi berbahasa Inggris (`README.md`) yang menjadi acuan otoritatif. Seluruh dokumen teknis mendalam dan dokumen hukum dikelola dalam Bahasa Inggris.

> **Jika Anda menjalankan PowerToys hanya untuk FancyZones, dan Logi Options+ hanya untuk tombol jempol, aplikasi ini menggantikan keduanya: satu proses tray, bukan dua.**
>
> Yang tidak digantikan: PowerRename, Awake, Color Picker, tata letak FancyZones kustom; Logitech Flow, profil per aplikasi, pengaturan baterai, atau pengalihan DPI.

## Instalasi

### Melalui Scoop (Direkomendasikan)

```powershell
scoop bucket add wiradesk https://github.com/wiradeltaid/scoop-wiradesk
scoop install wiradesk
```

### File Penginstal (Setup Executable)

Unduh penginstal (`WiraDesk-*-x64-setup.exe`) dari [halaman rilis](https://github.com/wiradeltaid/wira-desk/releases) (cermin di [SourceForge](https://sourceforge.net/projects/wira-desk/files/latest/download)) dan verifikasi SHA-256:

```powershell
Get-FileHash .\WiraDesk-*-x64-setup.exe -Algorithm SHA256
```

Terpasang dengan hak elevasi di `%ProgramFiles%\Wira Desk`. Auto-start bersifat opsional dan dapat diatur dari Pengaturan atau ikon tray.

### Biner Mandiri (Portable)

Unduh berkas biner mandiri `wiradesk.exe` dan `wiradesk-settings.exe` ke direktori khusus administrator dan jalankan `wiradesk.exe` sebagai Administrator.

---

## Fitur Utama

- **Same-App Window Cycling:** ``Win + ` `` beralih hanya di antara jendela aplikasi yang sedang aktif pada monitor dan virtual desktop saat ini (fallback: ``Alt + ` ``). Tekan cepat untuk berpindah seketika, atau tahan 300 ms untuk overlay switcher visual dengan gambar mini langsung.
- **One-Key Zone Snapping:** Penataan jendela instan ke separuh layar (50%), sepertiga (33%), atau persentase kustom terarah (default 67%, tepi atas default 33%) tanpa membuka editor zona.
- **Driverless Mouse Navigation:** Memetakan tombol jempol (`XBUTTON1`/`XBUTTON2`) dan horizontal tilt wheel ke perpindahan virtual desktop atau 20 preset kustom tanpa utilitas latar belakang dari vendor.

### Pintasan Bawaan (Default Shortcuts)

| Pintasan | Aksi |
|---|---|
| ``Win + ` `` | Beralih jendela pada aplikasi aktif (tahan 300 ms untuk visual switcher overlay) |
| ``Alt + ` `` | Pintasan pengalihan cadangan (fallback) |
| `Ctrl+Alt+Left/Right/Up/Down` | Pasang jendela aktif ke separuh layar tersebut (50%) |
| `Ctrl+Alt+Shift+Left/Right/Up/Down` | Pasang jendela ke tepi tersebut dengan persentase kustom (kiri/kanan/bawah default 67%, atas default 33%) |
| `Ctrl+Alt+1/2/3` | Pasang jendela ke sepertiga kiri, tengah, atau kanan |
| `Ctrl+Alt+Enter` | Maksimalkan jendela |
| `Ctrl+Alt+Shift+Enter` | Pindahkan jendela ke monitor berikutnya |
| `Ctrl+Alt+Shift+S` | Susun 3 jendela berdampingan pada lebar yang dapat dikonfigurasi |

### Preset Tombol Mouse

Tombol jempol secara bawaan beralih ke desktop virtual sebelumnya/berikutnya; tilt wheel horizontal beralih ke Show Desktop / Task View. Masing-masing dapat dipetakan ulang ke salah satu dari 20 preset di menu Pengaturan. Koordinat kursor tidak pernah dibaca; lihat [`PRIVACY.md`](PRIVACY.md).

---

## Mengapa Wira Desk

Windows tidak punya fitur bawaan untuk berpindah antarjendela dari aplikasi yang sama. PowerToys, unduhan terpisah dari Microsoft, menambahkan Window Hopper di versi 0.101 (nonaktif secara default), sementara utilitas vendor mengatur tombol mouse; bersama-sama keduanya menjalankan beberapa proses latar belakang yang memakai 150–500 MB RAM. Wira Desk berjalan sebagai satu daemon latar belakang native yang memakai sekitar 4.0 MB memori privat (di bawah anggaran 5 MB), tanpa telemetri.

---

## Konfigurasi & Pengembangan

- **Konfigurasi:** Pengaturan tersimpan di `%APPDATA%\WiraDesk\config.toml`. Lihat [docs/CONFIGURATION.md](docs/CONFIGURATION.md) untuk rujukan TOML lengkap.
- **Pengembangan:** Dibangun dengan Rust dan MSVC. Lihat [DEVELOPMENT.md](DEVELOPMENT.md) untuk panduan kompilasi, pengujian, dan kode unsafe.
- **Kontribusi:** Kontribusi kode sangat terbuka — lihat [CONTRIBUTING.md](CONTRIBUTING.md).

---

## Tentang & Ketentuan Hukum

**Wira Delta Indonesia** adalah studio di balik proyek ini. Dibangun dan dipelihara oleh [@kodesh87](https://github.com/kodesh87).

- **Lisensi:** [GPL-3.0-only](LICENSE). Atribusi pihak ketiga tercantum di [NOTICE](NOTICE). Dibangun menggunakan [Slint](https://slint.dev).
- **Privasi & Keamanan:** Nol telemetri, tanpa pembuatan akun, tanpa layanan pembaruan latar belakang yang berjalan diam-diam. Lihat [PRIVACY.md](PRIVACY.md) dan [SECURITY.md](SECURITY.md).
- **Nama dan Ikon:** Lisensi GPL memberikan hak atas kode, bukan atas nama atau logo. Nama **Wira Desk** dan **Wira Delta Indonesia**, serta ikon produk, tetap merupakan hak milik Wira Delta Indonesia.
