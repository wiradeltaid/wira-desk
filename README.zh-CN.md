# Wira Desk

> 轻量、原生的 Windows 11 同应用窗口轮转、分区吸附与免驱动鼠标导航工具 —— 基于 Rust 编写 🦀

[English](README.md) | [Bahasa Indonesia](README.id.md) | [简体中文](README.zh-CN.md) | [日本語](README.ja.md) | [한국어](README.ko.md) | [Español](README.es.md) | [Deutsch](README.de.md) | [Français](README.fr.md) | [Português (Brasil)](README.pt-BR.md) | [Русский](README.ru.md)  
[Website](https://wiradelta.com/wira-desk/) | [Download](https://github.com/wiradeltaid/wira-desk/releases) | [Changelog](CHANGELOG.md) | [Contributing](CONTRIBUTING.md) | [License](LICENSE) | [Security](SECURITY.md) | [Privacy](PRIVACY.md)

---

> **翻译说明：** 本文件是 [README.md](README.md) 的参考译文。如存在任何语义分歧或解释冲突，一律以官方英文版（README.md）为准。所有深度技术文档与法律条款均以英文维护。

> **如果您运行 PowerToys 仅为了 FancyZones，运行 Logi Options+ 仅为了拇指按键，Wira Desk 可以同时替代两者 —— 用一个托盘进程取代两个庞大后台。**
>
> 本工具不替代的功能：PowerRename、Awake、屏幕取色器、自定义手绘 FancyZones 布局；Logitech Flow、分应用专属配置、电池电量监控或 DPI 切换。

## 安装指南

### 通过 Scoop 安装（推荐）

```powershell
scoop bucket add wiradesk https://github.com/wiradeltaid/scoop-wiradesk
scoop install wiradesk
```

### 安装包程序 (Setup Executable)

从 [GitHub Releases 页面](https://github.com/wiradeltaid/wira-desk/releases)（或 [SourceForge 镜像](https://sourceforge.net/projects/wira-desk/files/latest/download)）下载安装包（`WiraDesk-*-x64-setup.exe`）并验证 SHA-256 哈希值：

```powershell
Get-FileHash .\WiraDesk-*-x64-setup.exe -Algorithm SHA256
```

安装至 `%ProgramFiles%\Wira Desk`（需要管理员权限）。开机自启在初次引导 (onboarding) 时提供预选勾选框，可随时在“设置”或托盘图标中修改。

### 便携压缩包 (Portable Archive)

从 [GitHub Releases 页面](https://github.com/wiradeltaid/wira-desk/releases) 下载 `WiraDesk-*-x64-portable.zip` 并解压到管理员专用目录。以管理员身份运行 `wiradesk.exe` 即可。

---

## 核心特性

- **同应用窗口轮转 (Same-App Window Cycling)：** 按下 ``Win + ` `` 仅在当前显示器和当前虚拟桌面上轮转活动应用的窗口（备用热键：``Alt + ` ``）。轻按瞬间切换，按住 300 毫秒则显示带实时缩略图的可视化窗口切换浮层。
- **一键分区吸附 (One-Key Zone Snapping)：** 无需打开复杂的区域编辑器，一键将窗口精准吸附至半屏 (50%)、三分屏 (33%) 或方向性自定义比例（默认 67%，顶部默认 33%）。
- **免驱动鼠标导航 (Driverless Mouse Navigation)：** 无需常驻后台的厂商外设软件，直接将拇指侧键（`XBUTTON1`/`XBUTTON2`）和滚轮横向摆动映射至虚拟桌面切换或 20 种预设操作。

### 默认快捷键

| 快捷键 | 功能 |
|---|---|
| ``Win + ` `` | 轮转当前应用的窗口（按住 300 ms 唤出可视化缩略图浮层） |
| ``Alt + ` `` | 备用窗口轮转热键 |
| `Ctrl+Alt+Left/Right/Up/Down` | 将活动窗口吸附至该半屏 (50%) |
| `Ctrl+Alt+Shift+Left/Right/Up/Down` | 按自定义比例吸附至边缘（左/右/下默认 67%，上默认 33%） |
| `Ctrl+Alt+1/2/3` | 将窗口吸附至左侧、中间或右侧三分之一区域 |
| `Ctrl+Alt+Enter` | 窗口最大化 |
| `Ctrl+Alt+Shift+Enter` | 将窗口移动至下一台显示器 |
| `Ctrl+Alt+Shift+S` | 按可配置宽度并排堆叠 3 个窗口 |

### 鼠标按键预设

拇指按键默认切换上一个/下一个虚拟桌面；滚轮横向左右摆动默认触发显示桌面 / 任务视图。各项均可在“设置”中重新映射至 20 种预设之一。程序绝不读取鼠标坐标；详见 [`PRIVACY.md`](PRIVACY.md)。

---

## 为什么选择 Wira Desk

Windows 没有内置的同一应用窗口切换功能。PowerToys 是 Microsoft 另行提供下载的工具，在 0.101 版加入了 Window Hopper（默认关闭），鼠标按键则由厂商外设工具负责；两者合计会运行多个后台进程。Wira Desk 作为单个原生后台守护进程运行，私有内存约 4.0 MB（低于 5 MB 预算）。无账号体系，无数据分析，无崩溃报告。

---

## 配置与开发

- **配置说明：** 设置文件位于 `%APPDATA%\WiraDesk\config.toml`。完整 TOML 配置项说明见 [docs/CONFIGURATION.md](docs/CONFIGURATION.md)。
- **编译开发：** 基于 Rust 与 MSVC 构建。构建、测试与 unsafe 代码规范见 [DEVELOPMENT.md](DEVELOPMENT.md)。
- **参与贡献：** 欢迎开源贡献 —— 详见 [CONTRIBUTING.md](CONTRIBUTING.md)。

---

## 关于与法律条款

**Wira Delta Indonesia** 为本项目背后的软件工作室。

- **开源许可：** 采用 [GPL-3.0-only](LICENSE) 许可证。第三方开源致谢详见 [NOTICE](NOTICE)。界面基于 [Slint](https://slint.dev) 构建。
- **隐私与安全：** 无账号体系，无数据分析，无崩溃报告。更新检查请求 wiradelta.com。详见 [PRIVACY.md](PRIVACY.md) 与 [SECURITY.md](SECURITY.md)。
- **名称与商标声明：** GPL 许可证仅授予代码相关权利，不授予产品名称与商标权。**Wira Desk** 和 **Wira Delta Indonesia** 名称以及产品图标均为 PT Wira Delta Indonesia 保留财产。
