# Wira Desk

> Windows 11向けの軽量・ネイティブな同一アプリ内ウィンドウ切り替え、ゾーンスナップ、ドライバ不要マウスナビゲーション — Rust 製 🦀

[English](README.md) | [Bahasa Indonesia](README.id.md) | [简体中文](README.zh-CN.md) | [日本語](README.ja.md) | [한국어](README.ko.md) | [Español](README.es.md) | [Deutsch](README.de.md) | [Français](README.fr.md) | [Português (Brasil)](README.pt-BR.md) | [Русский](README.ru.md)  
[Website](https://wiradelta.com/wira-desk/) | [Download](https://github.com/wiradeltaid/wira-desk/releases) | [Changelog](CHANGELOG.md) | [Contributing](CONTRIBUTING.md) | [License](LICENSE) | [Security](SECURITY.md) | [Privacy](PRIVACY.md)

---

> **翻訳に関する注意事項:** 本ファイルは [README.md](README.md) の便宜的な翻訳です。矛盾や解釈の相違がある場合は、公式の英語版（README.md）が優先されます。詳細な技術文書および法的文書はすべて英語で管理されています。

> **PowerToys を FancyZones のためだけに、Logi Options+ を親指ボタンのためだけに使っているなら、本ツールがその両方を置き換えます — 2つの重い常駐プロセスを1つのトレイプロセスに集約。**
>
> 置き換えない機能: PowerRename、Awake、カラーピッカー、手描きのFancyZonesカスタムレイアウト; Logitech Flow、アプリ個別プロファイル、バッテリー残量監視、DPI切り替え。

## インストール

### Scoop 経由（推奨）

```powershell
scoop bucket add wiradesk https://github.com/wiradeltaid/scoop-wiradesk
scoop install wiradesk
```

### インストーラ形式 (Setup Executable)

[Releases ページ](https://github.com/wiradeltaid/wira-desk/releases)（または [SourceForge ミラー](https://sourceforge.net/projects/wira-desk/files/latest/download)）からインストーラ（`WiraDesk-*-x64-setup.exe`）をダウンロードし、SHA-256 を検証してください:

```powershell
Get-FileHash .\WiraDesk-*-x64-setup.exe -Algorithm SHA256
```

`%ProgramFiles%\Wira Desk` に管理者権限でインストールされます。スタートアップ登録は初期設定（オンボーディング）時にチェック済みの選択肢として案内され、後から「設定」やトレイアイコンからいつでも変更可能です。

### ポータブルアーカイブ (Portable)

[Releases ページ](https://github.com/wiradeltaid/wira-desk/releases) から `WiraDesk-*-x64-portable.zip` をダウンロードし、管理者権限ディレクトリに展開して `wiradesk.exe` を管理者として実行してください。

---

## 主な機能

- **同一アプリ内ウィンドウ切り替え (Same-App Window Cycling):** ``Win + ` `` により、現在のディスプレイおよび仮想デスクトップ上でアクティブな同一アプリのウィンドウのみを循環します（代替: ``Alt + ` ``）。素早くタップすれば瞬時に切り替わり、300ms長押しするとリアルタイムサムネイル付きビジュアルオーバーレイが表示されます。
- **ワンキー・ゾーンスナップ (One-Key Zone Snapping):** ゾーンエディタを開くことなく、ウィンドウを素早く半画面 (50%)、3分割 (33%)、または指定の比率（左右下デフォルト67%、上デフォルト33%）に配置します。
- **ドライバレス・マウスナビゲーション:** ベンダー製常駐ツールなしで、親指ボタン（`XBUTTON1`/`XBUTTON2`）やチルトホイールの左右倒しを仮想デスクトップ切り替えや20種類のプリセットに割り当てます。

### デフォルトショートカット

| ショートカット | 動作 |
|---|---|
| ``Win + ` `` | アクティブアプリのウィンドウを切り替え（300ms長押しでビジュアルスイッチャー表示） |
| ``Alt + ` `` | 代替切り替えショートカット |
| `Ctrl+Alt+Left/Right/Up/Down` | アクティブウィンドウを画面の該当半分（50%）に配置 |
| `Ctrl+Alt+Shift+Left/Right/Up/Down` | ウィンドウをカスタム比率で端にスナップ（左右下67%、上33%） |
| `Ctrl+Alt+1/2/3` | ウィンドウを左、中央、右の3分の1にスナップ |
| `Ctrl+Alt+Enter` | ウィンドウを最大化 |
| `Ctrl+Alt+Shift+Enter` | ウィンドウを次のディスプレイへ移動 |
| `Ctrl+Alt+Shift+S` | 3つのウィンドウを設定幅で並べてスタック |

### マウスボタン・プリセット

親指ボタンは標準で前後の仮想デスクトップ切り替えに、チルトホイールはデスクトップ表示 / タスクビューに割り当てられています。設定画面から20種類のプリセットへ変更可能です。カーソル座標を取得することはありません。詳細は [`PRIVACY.md`](PRIVACY.md) を参照してください。

---

## なぜ Wira Desk なのか

Windows には、同じアプリのウィンドウだけを切り替える標準機能がありません。Microsoft が別途配布している PowerToys はバージョン 0.101 で Window Hopper を追加しましたが（既定では無効）、マウスボタンは周辺機器メーカーのツールが担います。これらを合わせると、複数のバックグラウンドプロセスが動作します。Wira Desk は単一のネイティブ常駐プロセスとして動作し、プライベートメモリは約 4.0 MB（予算 5 MB 以下）です。アカウント不要、アナリティクスなし、クラッシュレポート送信なし。

---

## 設定と開発

- **設定:** 設定ファイルは `%APPDATA%\WiraDesk\config.toml` に配置されます。完全なリファレンスは [docs/CONFIGURATION.md](docs/CONFIGURATION.md) をご覧ください。
- **開発:** Rust および MSVC でビルドされています。ビルド、テスト、unsafe コード指針は [DEVELOPMENT.md](DEVELOPMENT.md) を参照してください。
- **貢献:** オープンソースへのコントリビューションを歓迎します — [CONTRIBUTING.md](CONTRIBUTING.md) をご覧ください。

---

## 権利表記と法的事項

**Wira Delta Indonesia** は本プロジェクトの開発元スタジオです。

- **ライセンス:** [GPL-3.0-only](LICENSE)。サードパーティ製コンポーネントの権利表記は [NOTICE](NOTICE) に記載されています。UI は [Slint](https://slint.dev) を使用しています。
- **プライバシーとセキュリティ:** アカウント不要、アナリティクスなし、クラッシュレポート送信なし。更新確認は wiradelta.com へ問い合わせます。[PRIVACY.md](PRIVACY.md) および [SECURITY.md](SECURITY.md) を参照してください。
- **名称とアイコンについて:** GPL ライセンスはコードの権利を許諾するものであり、名称や商標を許諾するものではありません。**Wira Desk** および **Wira Delta Indonesia** の名称、製品アイコンは PT Wira Delta Indonesia の権利物です。
