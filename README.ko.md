# Wira Desk

> Windows 11을 위한 가볍고 네이티브한 동일 앱 창 전환, 영역 스냅, 드라이버 없는 마우스 탐색 도구 — Rust로 작성됨 🦀

[English](README.md) | [Bahasa Indonesia](README.id.md) | [简体中文](README.zh-CN.md) | [日本語](README.ja.md) | [한국어](README.ko.md) | [Español](README.es.md) | [Deutsch](README.de.md) | [Français](README.fr.md) | [Português (Brasil)](README.pt-BR.md) | [Русский](README.ru.md)  
[Website](https://wiradelta.id/wira-desk) | [Download](https://github.com/wiradeltaid/wira-desk/releases) | [Changelog](CHANGELOG.md) | [Contributing](CONTRIBUTING.md) | [License](LICENSE) | [Security](SECURITY.md) | [Privacy](PRIVACY.md)

---

> **번역 안내:** 본 문서는 편의를 위해 [README.md](README.md)를 번역한 참고용 문서입니다. 내용상 상충이나 해석의 차이가 있을 경우 영문 공식 문서(`README.md`)가 우선합니다. 세부 기술 문서 및 법적 문서는 영어로 관리됩니다.

> **PowerToys를 FancyZones 때문에만, Logi Options+를 엄지 버튼 때문에만 사용하신다면, 본 도구가 둘 모두를 대체합니다 — 무거운 두 백그라운드 프로세스 대신 하나의 트레이 프로세스로 충분합니다.**
>
> 대체하지 않는 기능: PowerRename, Awake, 색상 선택기, 커스텀 FancyZones 레이아웃; Logitech Flow, 앱별 프로필, 배터리 잔량 모니터링, DPI 전환.

## 설치 안내

### Scoop 패키지 매니저 (권장)

```powershell
scoop bucket add wiradesk https://github.com/wiradeltaid/scoop-wiradesk
scoop install wiradesk
```

### 설치 프로그램 (Setup Executable)

[GitHub Releases 페이지](https://github.com/wiradeltaid/wira-desk/releases)(또는 [SourceForge 미러](https://sourceforge.net/projects/wira-desk/files/latest/download))에서 설치 프로그램(`WiraDesk-*-x64-setup.exe`)을 다운로드하고 SHA-256 해시를 검증하십시오:

```powershell
Get-FileHash .\WiraDesk-*-x64-setup.exe -Algorithm SHA256
```

관리자 권한으로 `%ProgramFiles%\Wira Desk`에 설치됩니다. 시작 프로그램 등록은 선택 사항이며 설정 화면이나 트레이 아이콘에서 조정할 수 있습니다.

### 포터블 단독 실행 바이너리 (Portable)

단독 `wiradesk.exe` 및 `wiradesk-settings.exe`를 관리자 전용 폴더에 다운로드하고 관리자 권한으로 `wiradesk.exe`를 실행하십시오.

---

## 주요 기능

- **동일 앱 창 전환 (Same-App Window Cycling):** ``Win + ` `` 키를 눌러 현재 모니터 및 현재 가상 데스크톱에서 활성화된 동일 앱의 창만 순환합니다(대체 단축키: ``Alt + ` ``). 가볍게 누르면 즉시 전환되며, 300ms 동안 길게 누르면 실시간 썸네일이 포함된 시각적 스위처 오버레이가 나타납니다.
- **원키 영역 스냅 (One-Key Zone Snapping):** 번거로운 영역 편집기를 열지 않고도 창을 화면의 절반(50%), 3분의 1(33%) 또는 사용자 정의 비율(좌/우/하 기본 67%, 상 기본 33%)로 즉시 스냅합니다.
- **드라이버 없는 마우스 탐색:** 제조사의 무거운 백그라운드 유틸리티 없이 마우스 엄지 버튼(`XBUTTON1`/`XBUTTON2`) 및 휠 좌우 틸트를 가상 데스크톱 전환이나 20가지 사용자 지정 프리셋에 매핑합니다.

### 기본 단축키 목록

| 단축키 | 동작 |
|---|---|
| ``Win + ` `` | 활성 앱 창 순환 (300ms 길게 누르면 시각적 스위처 오버레이 표시) |
| ``Alt + ` `` | 대체 창 순환 단축키 |
| `Ctrl+Alt+Left/Right/Up/Down` | 활성 창을 해당 절반 영역(50%)에 스냅 |
| `Ctrl+Alt+Shift+Left/Right/Up/Down` | 창을 사용자 정의 비율로 해당 가장자리에 스냅 (좌/우/하 기본 67%, 상 기본 33%) |
| `Ctrl+Alt+1/2/3` | 창을 좌측, 중앙, 우측 3분의 1에 스냅 |
| `Ctrl+Alt+Enter` | 창 최대화 |
| `Ctrl+Alt+Shift+Enter` | 창을 다음 모니터로 이동 |
| `Ctrl+Alt+Shift+S` | 설정된 너비로 창 3개를 나란히 정렬 |

### 마우스 프리셋

엄지 버튼은 기본적으로 이전/다음 가상 데스크톱으로 전환하며, 틸트 휠은 바탕화면 보기 / 작업 보기에 매핑되어 있습니다. 설정 창에서 20가지 프리셋 중 하나로 자유롭게 변경할 수 있습니다. 커서 좌표는 절대 읽지 않습니다([`PRIVACY.md`](PRIVACY.md) 참조).

---

## 왜 Wira Desk인가

Windows에는 같은 앱의 창끼리만 전환하는 기본 기능이 없습니다. Microsoft가 별도로 배포하는 PowerToys는 0.101 버전에서 Window Hopper를 추가했고(기본값은 꺼짐), 마우스 버튼은 제조사 유틸리티가 맡습니다. 이들을 함께 쓰면 여러 백그라운드 프로세스가 150~500 MB의 RAM을 사용합니다. Wira Desk는 약 4.0 MB의 프라이빗 메모리(예산 5 MB 이하)를 쓰는 단일 네이티브 백그라운드 데몬으로 동작하며, 원격 측정이 없습니다.

---

## 설정 및 개발

- **설정:** 환경설정 파일은 `%APPDATA%\WiraDesk\config.toml`에 위치합니다. 전체 설명은 [docs/CONFIGURATION.md](docs/CONFIGURATION.md)를 참조하십시오.
- **개발:** Rust 및 MSVC 환경에서 빌드되었습니다. 빌드, 테스트 및 unsafe 코드 가이드라인은 [DEVELOPMENT.md](DEVELOPMENT.md)를 참조하십시오.
- **기여:** 오픈소스 기여를 진심으로 환영합니다 — [CONTRIBUTING.md](CONTRIBUTING.md)를 참조하십시오.

---

## 정보 및 법적 고지

**Wira Delta Indonesia**는 본 프로젝트를 운영하는 소프트웨어 스튜디오입니다. [@kodesh87](https://github.com/kodesh87)이 개발 및 관리하고 있습니다.

- **라이선스:** [GPL-3.0-only](LICENSE). 타사 오픈소스 라이선스 고지는 [NOTICE](NOTICE)에 정리되어 있습니다. UI는 [Slint](https://slint.dev)로 구현되었습니다.
- **개인정보 보호 및 보안:** 원격 측정 0, 계정 생성 불필요, 백그라운드 자동 업데이트 서비스 없음. [PRIVACY.md](PRIVACY.md) 및 [SECURITY.md](SECURITY.md)를 참조하십시오.
- **명칭 및 상표권 고지:** GPL 라이선스는 소스 코드에 대한 권리를 부여하며 제품명이나 로고 상표권을 부여하지 않습니다. **Wira Desk** 및 **Wira Delta Indonesia** 명칭과 제품 아이콘은 PT Wira Delta Indonesia의 자산입니다.
