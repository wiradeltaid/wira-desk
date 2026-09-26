# Wira Desk

> Alternancia nativa y ligera entre ventanas de una misma aplicación, ajuste por zonas y navegación con ratón sin controladores para Windows 11 — escrito en Rust 🦀

[English](README.md) | [Bahasa Indonesia](README.id.md) | [简体中文](README.zh-CN.md) | [日本語](README.ja.md) | [한국어](README.ko.md) | [Español](README.es.md) | [Deutsch](README.de.md) | [Français](README.fr.md) | [Português (Brasil)](README.pt-BR.md) | [Русский](README.ru.md)  
[Website](https://wiradelta.com/wira-desk/) | [Download](https://github.com/wiradeltaid/wira-desk/releases) | [Changelog](CHANGELOG.md) | [Contributing](CONTRIBUTING.md) | [License](LICENSE) | [Security](SECURITY.md) | [Privacy](PRIVACY.md)

---

> **Aviso de traducción:** Este archivo es una traducción de [README.md](README.md) sólo para fines de conveniencia. En caso de discrepancia o conflicto de interpretación, la versión oficial en inglés (`README.md`) prevalece como autorizada. Toda la documentación técnica profunda y los documentos legales se mantienen en inglés.

> **Si ejecuta PowerToys únicamente por FancyZones y Logi Options+ sólo por los botones laterales del ratón, esta aplicación reemplaza a ambas: un único proceso en la bandeja en lugar de dos pesados servicios en segundo plano.**
>
> Lo que no reemplaza: PowerRename, Awake, Selector de color o diseños personalizados dibujados a mano de FancyZones; Logitech Flow, perfiles por aplicación, supervisión de batería o cambio de DPI.

## Instalación

### Vía Scoop (Recomendado)

```powershell
scoop bucket add wiradesk https://github.com/wiradeltaid/scoop-wiradesk
scoop install wiradesk
```

### Archivo Instalador (Setup Executable)

Descargue el instalador (`WiraDesk-*-x64-setup.exe`) desde la [página de lanzamientos](https://github.com/wiradeltaid/wira-desk/releases) (espejo en [SourceForge](https://sourceforge.net/projects/wira-desk/files/latest/download)) y compruebe el hash SHA-256:

```powershell
Get-FileHash .\WiraDesk-*-x64-setup.exe -Algorithm SHA256
```

Se instala de forma elevada en `%ProgramFiles%\Wira Desk`. El inicio automático se propone durante la configuración inicial (onboarding) con una opción premarcada, y se puede cambiar en cualquier momento en Ajustes o desde el icono de la bandeja.

### Archivo Portátil (Portable Archive)

Descargue `WiraDesk-*-x64-portable.zip` desde la [página de lanzamientos](https://github.com/wiradeltaid/wira-desk/releases) y extráigalo en una carpeta con permisos de administrador. Ejecute `wiradesk.exe` como Administrador.

---

## Características Principales

- **Alternancia entre ventanas de la misma app (Same-App Window Cycling):** ``Win + ` `` alterna únicamente entre las ventanas de la aplicación activa en el monitor y escritorio virtual actual (atajo de respaldo: ``Alt + ` ``). Toque para alternar al instante, o mantenga presionado 300 ms para abrir la ventana superpuesta con miniaturas en tiempo real.
- **Ajuste por zonas con una tecla (One-Key Zone Snapping):** Ajuste instantáneo de ventanas a mitades (50%), tercios (33%) o porcentajes direccionales personalizados (67% por defecto, borde superior 33%) sin abrir un editor de zonas.
- **Navegación con ratón sin controladores:** Asigne los botones laterales del pulgar (`XBUTTON1`/`XBUTTON2`) y la inclinación horizontal de la rueda al cambio de escritorio virtual o a 20 ajustes preestablecidos sin requerir utilidades pesadas del fabricante.

### Atajos de Teclado Predeterminados

| Atajo | Acción |
|---|---|
| ``Win + ` `` | Alternar ventanas de la app activa (mantener 300 ms para ver el selector visual) |
| ``Alt + ` `` | Atajo de alternancia alternativo (fallback) |
| `Ctrl+Alt+Izquierda/Derecha/Arriba/Abajo` | Ajustar ventana activa a esa mitad (50%) |
| `Ctrl+Alt+Shift+Izquierda/Derecha/Arriba/Abajo` | Ajustar ventana a ese borde con porcentaje personalizado (67% defecto, superior 33%) |
| `Ctrl+Alt+1/2/3` | Ajustar ventana al tercio izquierdo, central o derecho |
| `Ctrl+Alt+Enter` | Maximizar ventana |
| `Ctrl+Alt+Shift+Enter` | Mover ventana al monitor siguiente |
| `Ctrl+Alt+Shift+S` | Apilar 3 ventanas en paralelo con ancho configurable |

### Ajustes Preestablecidos del Ratón

Los botones del pulgar cambian por defecto al escritorio virtual anterior/siguiente; la inclinación de la rueda cambia a Mostrar Escritorio / Vista de Tareas. Cada uno es reasignable a 20 preajustes en Configuración. Las coordenadas del cursor nunca se leen; consulte [`PRIVACY.md`](PRIVACY.md).

---

## ¿Por qué Wira Desk?

Windows no incluye una forma nativa de alternar entre las ventanas de una misma aplicación. PowerToys, una descarga aparte de Microsoft, añadió Window Hopper en la versión 0.101 (desactivado por defecto), y las herramientas de los fabricantes gestionan los botones del ratón; en conjunto ejecutan varios procesos en segundo plano. Wira Desk se ejecuta como un único servicio nativo en segundo plano que usa unos 4.0 MB de memoria privada (por debajo de un límite de 5 MB). Sin cuentas de usuario, sin analítica de datos, sin informes de fallos.

---

## Configuración y Desarrollo

- **Configuración:** Los ajustes residen en `%APPDATA%\WiraDesk\config.toml`. Consulte [docs/CONFIGURATION.md](docs/CONFIGURATION.md) para la referencia completa de TOML.
- **Desarrollo:** Desarrollado con Rust y MSVC. Consulte [DEVELOPMENT.md](DEVELOPMENT.md) para directrices de compilación, pruebas y código unsafe.
- **Contribución:** Las contribuciones al código son bienvenidas: consulte [CONTRIBUTING.md](CONTRIBUTING.md).

---

## Información y Términos Legales

**Wira Delta Indonesia** es el estudio de software responsable de este proyecto.

- **Licencia:** [GPL-3.0-only](LICENSE). Los reconocimientos a componentes de terceros se listan en [NOTICE](NOTICE). Construido con [Slint](https://slint.dev).
- **Privacidad y Seguridad:** Sin cuentas de usuario, sin analítica de datos, sin informes de fallos. Las comprobaciones de actualización consultan wiradelta.com. Consulte [PRIVACY.md](PRIVACY.md) y [SECURITY.md](SECURITY.md).
- **El Nombre y el Icono:** La licencia GPL otorga derechos sobre el código, no sobre nombres ni logotipos. Los nombres **Wira Desk** y **Wira Delta Indonesia**, así como el icono del producto, son propiedad de PT Wira Delta Indonesia.
