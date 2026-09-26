# Wira Desk

> Basculement léger et natif entre fenêtres d'une même application, ancrage par zones et navigation à la souris sans pilote pour Windows 11 — écrit en Rust 🦀

[English](README.md) | [Bahasa Indonesia](README.id.md) | [简体中文](README.zh-CN.md) | [日本語](README.ja.md) | [한국어](README.ko.md) | [Español](README.es.md) | [Deutsch](README.de.md) | [Français](README.fr.md) | [Português (Brasil)](README.pt-BR.md) | [Русский](README.ru.md)  
[Website](https://wiradelta.com/wira-desk/) | [Download](https://github.com/wiradeltaid/wira-desk/releases) | [Changelog](CHANGELOG.md) | [Contributing](CONTRIBUTING.md) | [License](LICENSE) | [Security](SECURITY.md) | [Privacy](PRIVACY.md)

---

> **Avis de traduction :** Ce fichier est une traduction de [README.md](README.md) fournie uniquement à titre indicatif. En cas de divergence ou de conflit d'interprétation, la version officielle en langue anglaise (`README.md`) prévaut. L'ensemble de la documentation technique approfondie et des documents juridiques est maintenu en anglais.

> **Si vous n'utilisez PowerToys que pour FancyZones et Logi Options+ uniquement pour les boutons de pouce, cette application remplace les deux — un seul processus dans la barre des tâches au lieu de deux services en arrière-plan.**
>
> Ce qu'elle ne remplace pas : PowerRename, Awake, la pipette de couleurs, les agencements personnalisés FancyZones ; Logitech Flow, les profils par application, la surveillance de batterie ou le réglage DPI.

## Installation

### Via Scoop (Recommandé)

```powershell
scoop bucket add wiradesk https://github.com/wiradeltaid/scoop-wiradesk
scoop install wiradesk
```

### Fichier d'Installation (Setup Executable)

Téléchargez le programme d'installation (`WiraDesk-*-x64-setup.exe`) depuis la [page des versions](https://github.com/wiradeltaid/wira-desk/releases) (miroir sur [SourceForge](https://sourceforge.net/projects/wira-desk/files/latest/download)) et vérifiez le hash SHA-256 :

```powershell
Get-FileHash .\WiraDesk-*-x64-setup.exe -Algorithm SHA256
```

S'installe avec élévation de privilèges dans `%ProgramFiles%\Wira Desk`. Le lancement au démarrage est proposé lors de la configuration initiale (onboarding) avec une option précochée, et se modifie à tout moment dans les Paramètres ou depuis l'icône de la barre d'état.

### Archive Portable (Portable Archive)

Téléchargez `WiraDesk-*-x64-portable.zip` depuis la [page des versions](https://github.com/wiradeltaid/wira-desk/releases) et extrayez-le dans un dossier réservé aux administrateurs. Exécutez `wiradesk.exe` en tant qu'Administrateur.

---

## Fonctionnalités Principales

- **Basculement inter-fenêtres d'une même app (Same-App Window Cycling) :** ``Win + ` `` bascule uniquement entre les fenêtres de l'application active sur l'écran et le bureau virtuel actuels (raccourci de secours : ``Alt + ` ``). Une pression brève change de fenêtre instantanément, un appui de 300 ms affiche la vue superposée avec vignettes miniatures en direct.
- **Ancrage par zones à une touche (One-Key Zone Snapping) :** Ancrage instantané aux moitiés (50%), tiers (33%) ou pourcentages personnalisés (défaut 67%, haut 33%) sans ouvrir d'éditeur de zones.
- **Navigation à la souris sans pilote :** Mappe les boutons de pouce (`XBUTTON1`/`XBUTTON2`) et l'inclinaison de la molette sur le changement de bureau virtuel ou sur 20 préréglages sans utilitaire propriétaire lourd.

### Raccourcis Clavier par Défaut

| Raccourci | Action |
|---|---|
| ``Win + ` `` | Basculer entre fenêtres de l'app active (maintenir 300 ms pour afficher le sélecteur) |
| ``Alt + ` `` | Raccourci de secours pour le basculement |
| `Ctrl+Alt+Gauche/Droite/Haut/Bas` | Ancrer la fenêtre active sur cette moitié d'écran (50%) |
| `Ctrl+Alt+Maj+Gauche/Droite/Haut/Bas` | Ancrer au bord avec ratio personnalisé (67% défaut, haut 33%) |
| `Ctrl+Alt+1/2/3` | Ancrer la fenêtre au tiers gauche, central ou droit |
| `Ctrl+Alt+Entrée` | Maximiser la fenêtre |
| `Ctrl+Alt+Maj+Entrée` | Déplacer la fenêtre vers l'écran suivant |
| `Ctrl+Alt+Maj+S` | Aligner 3 fenêtres côte à côte à largeur configurable |

### Préréglages Souris

Les boutons de pouce basculent par défaut vers le bureau virtuel précédent/suivant ; l'inclinaison de la molette active Afficher le bureau / Vue des tâches. Chacun est réassignable parmi 20 préréglages dans les Paramètres. Les coordonnées du curseur ne sont jamais collectées (voir [`PRIVACY.md`](PRIVACY.md)).

---

## Pourquoi Choisir Wira Desk

Windows n'offre pas de moyen intégré pour passer d'une fenêtre à l'autre d'une même application. PowerToys, un téléchargement séparé de Microsoft, a ajouté Window Hopper dans la version 0.101 (désactivé par défaut), et les outils des fabricants gèrent les boutons de la souris ; ensemble, ils font tourner plusieurs processus d'arrière-plan. Wira Desk s'exécute comme un seul démon natif en arrière-plan utilisant environ 4.0 Mo de mémoire privée (sous un budget de 5 Mo). Aucun compte requis, aucune analyse de données, aucun rapport d'incident.

---

## Configuration & Développement

- **Configuration :** Les paramètres sont stockés dans `%APPDATA%\WiraDesk\config.toml`. Consultez [docs/CONFIGURATION.md](docs/CONFIGURATION.md) pour la référence TOML complète.
- **Développement :** Développé avec Rust et MSVC. Consultez [DEVELOPMENT.md](DEVELOPMENT.md) pour les instructions de compilation, de test et les règles de code unsafe.
- **Contribution :** Les contributions open source sont les bienvenues — voir [CONTRIBUTING.md](CONTRIBUTING.md).

---

## À Propos & Mentions Légales

**Wira Delta Indonesia** est le studio de développement à l'origine de ce projet.

- **Licence :** [GPL-3.0-only](LICENSE). Les attributions tierces sont listées dans [NOTICE](NOTICE). Interface développée avec [Slint](https://slint.dev).
- **Confidentialité & Sécurité :** Aucun compte requis, aucune analyse de données, aucun rapport d'incident. Les vérifications de mise à jour contactent wiradelta.com. Consultez [PRIVACY.md](PRIVACY.md) et [SECURITY.md](SECURITY.md).
- **Nom et Marque :** La licence GPL accorde des droits sur le code, pas sur les noms ou logos. Les noms **Wira Desk** et **Wira Delta Indonesia**, ainsi que l'icône du produit, demeurent la propriété de PT Wira Delta Indonesia.
