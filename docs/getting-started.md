# Getting started

Guia para clonar, instalar dependências e rodar o Kanban pela primeira vez.

## Requisitos

| Ferramenta | Versão mínima | Notas |
|------------|---------------|--------|
| [Rust](https://www.rust-lang.org/tools/install) | stable (1.77+) | `rustup` recomendado |
| [Node.js](https://nodejs.org/) | 18+ (LTS ou atual) | npm vem junto |
| Sistema com GUI | — | WebView nativo do SO |

### Dependências por sistema operacional

#### Linux

Instale as bibliotecas do Tauri 2. Exemplo **Debian/Ubuntu**:

```bash
sudo apt update
sudo apt install -y \
  libwebkit2gtk-4.1-dev \
  libappindicator3-dev \
  librsvg2-dev \
  patchelf \
  build-essential \
  curl \
  wget \
  file \
  libxdo-dev \
  libssl-dev \
  libayatana-appindicator3-dev
```

**Arch / Omarchy / Manjaro** (exemplo):

```bash
sudo pacman -S --needed \
  webkit2gtk-4.1 \
  base-devel \
  curl \
  wget \
  file \
  openssl \
  appmenu-gtk-module \
  libappindicator-gtk3 \
  librsvg \
  xdotool
```

Documentação oficial: [Tauri Prerequisites](https://v2.tauri.app/start/prerequisites/).

#### Windows

1. [Microsoft C++ Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/)
2. [WebView2](https://developer.microsoft.com/en-us/microsoft-edge/webview2/) (em geral já vem no Windows 10/11)
3. Rust (`rustup-init.exe`) + Node.js

#### macOS

```bash
xcode-select --install
# Rust + Node via rustup e site do Node / Homebrew
```

## Instalação do projeto

```bash
git clone <url-do-repositorio> testeAppKanban
cd testeAppKanban
npm install
```

Isso instala o frontend e o CLI do Tauri (`@tauri-apps/cli`). As crates Rust baixam no primeiro `tauri dev` / `tauri build`.

## Rodar em desenvolvimento

```bash
npm run tauri dev
```

O comando:

1. Sobe o Vite em `http://localhost:1420`
2. Compila o backend Rust
3. Abre a janela nativa com o webview

**Primeira compilação** pode levar vários minutos (baixa e compila dependências). As seguintes são bem mais rápidas.

### Só frontend

Útil para mexer em CSS/layout **sem** IPC real (comandos Rust vão falhar no browser puro):

```bash
npm run dev
```

Abra `http://localhost:1420`.

## Verificar se está ok

```bash
# Typecheck do Svelte/TypeScript
npm run check

# Compilação Rust (sem abrir UI)
cd src-tauri && cargo check
```

## Onde ficam os dados

O banco SQLite é criado no **diretório de dados do app** (não no repositório):

| SO | Caminho típico |
|----|----------------|
| Linux | `~/.local/share/com.testeapp.kanban/kanban.db` |
| macOS | `~/Library/Application Support/com.testeapp.kanban/kanban.db` |
| Windows | `%APPDATA%\com.testeapp.kanban\kanban.db` |

O `identifier` `com.testeapp.kanban` vem de `src-tauri/tauri.conf.json`.

Para “resetar” o app em dev: feche o app e apague o arquivo `kanban.db` (e a pasta se quiser).

## Próximos passos

- [Desenvolvimento](./development.md) — fluxo diário
- [Arquitetura](./architecture.md) — como o código se organiza
- [Build e instaladores](./building.md) — gerar binários de release
