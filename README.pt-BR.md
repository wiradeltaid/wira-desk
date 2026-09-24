# Wira Desk

> Alternância nativa e leve entre janelas do mesmo aplicativo, ajuste por zonas e navegação com mouse sem drivers para Windows 11 — escrito em Rust 🦀

[English](README.md) | [Bahasa Indonesia](README.id.md) | [简体中文](README.zh-CN.md) | [日本語](README.ja.md) | [한국어](README.ko.md) | [Español](README.es.md) | [Deutsch](README.de.md) | [Français](README.fr.md) | [Português (Brasil)](README.pt-BR.md) | [Русский](README.ru.md)  
[Website](https://wiradelta.id/wira-desk/) | [Download](https://github.com/wiradeltaid/wira-desk/releases) | [Changelog](CHANGELOG.md) | [Contributing](CONTRIBUTING.md) | [License](LICENSE) | [Security](SECURITY.md) | [Privacy](PRIVACY.md)

---

> **Aviso de tradução:** Este arquivo é uma tradução de [README.md](README.md) fornecida apenas para fins de conveniência. Em caso de divergências ou conflitos de interpretação, a versão oficial em inglês (`README.md`) prevalece como fonte autoritativa. Toda a documentação técnica aprofundada e documentos jurídicos são mantidos em inglês.

> **Se você executa o PowerToys apenas pelo FancyZones e o Logi Options+ apenas pelos botões laterais do mouse, este aplicativo substitui ambos — um único processo na bandeja em vez de dois serviços pesados em segundo plano.**
>
> O que não substitui: PowerRename, Awake, seletor de cores, layouts personalizados desenhados à mão do FancyZones; Logitech Flow, perfis por aplicativo, monitoramento de bateria ou troca de DPI.

## Instalação

### Via Scoop (Recomendado)

```powershell
scoop bucket add wiradesk https://github.com/wiradeltaid/scoop-wiradesk
scoop install wiradesk
```

### Arquivo Instalador (Setup Executable)

Baixe o instalador (`WiraDesk-*-x64-setup.exe`) na [página de lançamentos](https://github.com/wiradeltaid/wira-desk/releases) (espelho no [SourceForge](https://sourceforge.net/projects/wira-desk/files/latest/download)) e valide o hash SHA-256:

```powershell
Get-FileHash .\WiraDesk-*-x64-setup.exe -Algorithm SHA256
```

Instala com privilégios elevados em `%ProgramFiles%\Wira Desk`. A inicialização automática é sugerida durante a integração inicial (onboarding) com uma opção pré-marcada, e pode ser configurada a qualquer momento nas Configurações ou pelo ícone da bandeja.

### Arquivo Portátil (Portable Archive)

Baixe `WiraDesk-*-x64-portable.zip` na [página de lançamentos](https://github.com/wiradeltaid/wira-desk/releases) e extraia em uma pasta com permissões de administrador. Execute `wiradesk.exe` como Administrador.

---

## Recursos Principais

- **Alternância entre janelas do mesmo app (Same-App Window Cycling):** ``Win + ` `` alterna exclusivamente entre as janelas do aplicativo ativo no monitor e na área de trabalho virtual atual (atalho alternativo: ``Alt + ` ``). Toque para alternar instantaneamente, ou segure por 300 ms para abrir a sobreposição visual com miniaturas em tempo real.
- **Ajuste por zonas com uma tecla (One-Key Zone Snapping):** Encaixe instantâneo de janelas em metades (50%), terços (33%) ou proporções direcionais personalizadas (padrão 67%, topo padrão 33%) sem precisar abrir um editor de zonas.
- **Navegação com mouse sem drivers:** Mapeie os botões laterais do polegar (`XBUTTON1`/`XBUTTON2`) e a inclinação horizontal da roda para alternar áreas de trabalho virtuais ou para 20 predefinições sem utilitários pesados de fabricantes.

### Atalhos de Teclado Padrão

| Atalho | Ação |
|---|---|
| ``Win + ` `` | Alternar janelas do app ativo (segure 300 ms para ver o seletor visual) |
| ``Alt + ` `` | Atalho de alternância alternativo (fallback) |
| `Ctrl+Alt+Esquerda/Direita/Cima/Baixo` | Encaixar janela ativa na metade correspondente (50%) |
| `Ctrl+Alt+Shift+Esquerda/Direita/Cima/Baixo` | Encaixar janela na borda com proporção personalizada (67% padrão, topo 33%) |
| `Ctrl+Alt+1/2/3` | Encaixar janela no terço esquerdo, central ou direito |
| `Ctrl+Alt+Enter` | Maximizar janela |
| `Ctrl+Alt+Shift+Enter` | Mover janela para o monitor seguinte |
| `Ctrl+Alt+Shift+S` | Empilhar 3 janelas lado a lado com largura configurável |

### Predefinições de Mouse

Os botões do polegar alternam por padrão para a área de trabalho virtual anterior/seguinte; a inclinação da roda alterna para Mostrar Área de Trabalho / Visão de Tarefas. Cada ação pode ser remapeada para 20 predefinições nas Configurações. As coordenadas do cursor nunca são lidas (consulte [`PRIVACY.md`](PRIVACY.md)).

---

## Por que escolher o Wira Desk

O Windows não tem um recurso nativo para alternar entre as janelas de um mesmo aplicativo. O PowerToys, um download separado da Microsoft, adicionou o Window Hopper na versão 0.101 (desativado por padrão), e as ferramentas dos fabricantes cuidam dos botões do mouse; juntos, eles rodam vários processos em segundo plano. O Wira Desk roda como um único daemon nativo em segundo plano que usa cerca de 4.0 MB de memória privada (abaixo de um orçamento de 5 MB). Sem necessidade de cadastro, sem análise de dados, sem envio de relatórios de falhas.

---

## Configuração e Desenvolvimento

- **Configuração:** As configurações ficam em `%APPDATA%\WiraDesk\config.toml`. Consulte [docs/CONFIGURATION.md](docs/CONFIGURATION.md) para a referência TOML completa.
- **Desenvolvimento:** Desenvolvido com Rust e MSVC. Consulte [DEVELOPMENT.md](DEVELOPMENT.md) para diretrizes de compilação, testes e código unsafe.
- **Contribuição:** Contribuições ao código são muito bem-vindas — veja [CONTRIBUTING.md](CONTRIBUTING.md).

---

## Sobre e Termos Legais

**Wira Delta Indonesia** é o estúdio de software responsável por este projeto.

- **Licença:** [GPL-3.0-only](LICENSE). Os créditos a bibliotecas de terceiros estão listados em [NOTICE](NOTICE). Interface criada com [Slint](https://slint.dev).
- **Privacidade e Segurança:** Sem necessidade de cadastro, sem análise de dados, sem envio de relatórios de falhas. As verificações de atualização consultam wiradelta.id. Consulte [PRIVACY.md](PRIVACY.md) e [SECURITY.md](SECURITY.md).
- **O Nome e o Ícone:** A licença GPL concede direitos sobre o código, não sobre nomes ou logotipos. Os nomes **Wira Desk** e **Wira Delta Indonesia**, bem como o ícone do produto, são propriedade da PT Wira Delta Indonesia.
