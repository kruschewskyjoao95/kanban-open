# Kanban

**Kanban local-first** no estilo Trello — boards, listas, tarefas, etiquetas e checklists, tudo no seu computador. Sem conta, sem servidor obrigatório.

Construído com **Tauri 2** + **Rust** + **SQLite** + **Svelte 5**, para Linux, Windows e macOS (mobile no roadmap).

---

## Por que este app?

| | |
|--|--|
| **Local-first** | Dados em SQLite no disco; funciona offline |
| **Leve** | Tauri usa o webview do sistema (não embute Chromium como o Electron) |
| **Multiplataforma** | Um código → binários por SO |
| **Foco em Kanban** | Escopo claro, sem tentar ser um Notion |

---

## Funcionalidades

### Boards e listas
- Criar, renomear e excluir boards
- Listas padrão ao criar board: **To Do · Doing · Done**
- Adicionar / renomear / excluir listas
- Reordenar listas arrastando o cabeçalho (⋮⋮)

### Tarefas (cards)
- Título, descrição, edição em modal
- **Drag-and-drop** entre listas e reordenação
- Posição fracionária (estilo Trello)
- Feedback com toasts e updates otimistas

### Etiquetas
- Cores por board (padrão: Bug, Feature, Melhoria, Urgente)
- Painel de gestão e seletor no modal do card
- Chips coloridos na coluna

### Checklist
- Subtarefas com marcar/desmarcar, renomear e excluir
- Barra de progresso no modal e badge `done/total` no card

### Busca e produtividade
- Busca no board (título, descrição, etiquetas, checklist)
- Atalhos de teclado
- UI dark compacta para desktop

### Atalhos

| Tecla | Onde | Ação |
|-------|------|------|
| `n` | Home | Novo board |
| `n` | Board | Focar nova tarefa na 1ª lista |
| `l` | Board | Nova lista |
| `e` | Board | Gerenciar etiquetas |
| `/` | Board | Focar busca |
| `?` | Board | Ajuda de atalhos |
| `Esc` | Board | Limpar busca / fechar modal |

---

## Stack

| Camada | Tecnologia |
|--------|------------|
| Shell desktop | [Tauri 2](https://v2.tauri.app/) |
| Backend | [Rust](https://www.rust-lang.org/) + [SQLite](https://sqlite.org/) via [sqlx](https://github.com/launchbadge/sqlx) |
| Frontend | [Svelte 5](https://svelte.dev/) + [SvelteKit](https://kit.svelte.dev/) (SPA, `adapter-static`) |
| Build UI | Vite 6 + TypeScript |

```
┌─────────────────────────────────────┐
│  Svelte 5 UI (webview nativo)       │
│         ↓ invoke()                  │
│  Rust commands + sqlx               │
│         ↓                           │
│  SQLite (app_data_dir/kanban.db)    │
└─────────────────────────────────────┘
```

---

## Início rápido

### Pré-requisitos

- **Rust** (stable) — [rustup](https://rustup.rs/)
- **Node.js** 18+
- Dependências do Tauri no SO: [documentação oficial](https://v2.tauri.app/start/prerequisites/)

**Linux (exemplo Arch):**

```bash
sudo pacman -S --needed webkit2gtk-4.1 base-devel curl wget file openssl \
  appmenu-gtk-module libappindicator-gtk3 librsvg xdotool
```

**Linux (exemplo Debian/Ubuntu):** veja [docs/getting-started.md](./docs/getting-started.md).

### Desenvolvimento

```bash
git clone <url-do-repositorio> testeAppKanban
cd testeAppKanban
npm install
npm run tauri dev
```

Isso abre a **janela nativa** do app.  
A primeira compilação Rust pode demorar alguns minutos.

> **Importante:** não use só `npm run dev` no browser para usar o app de verdade.  
> Sem o shell Tauri o IPC/SQLite não existe (`invoke` falha).

### Typecheck

```bash
npm run check
```

---

## Build e instalação

### Linux (recomendado neste projeto)

Em Arch e distros com libs modernas, o AppImage precisa desabilitar o `strip` do linuxdeploy:

```bash
npm run tauri:build:linux
```

Artefatos típicos:

```text
src-tauri/target/release/bundle/appimage/Kanban_0.1.0_amd64.AppImage
src-tauri/target/release/bundle/deb/Kanban_0.1.0_amd64.deb
src-tauri/target/release/bundle/rpm/Kanban-0.1.0-1.x86_64.rpm
src-tauri/target/release/kanban   # binário direto
```

**Rodar AppImage:**

```bash
chmod +x src-tauri/target/release/bundle/appimage/Kanban_*.AppImage
./src-tauri/target/release/bundle/appimage/Kanban_*.AppImage
```

**Instalar .deb:**

```bash
sudo dpkg -i src-tauri/target/release/bundle/deb/Kanban_*.deb
```

**Só o binário:**

```bash
./src-tauri/target/release/kanban
```

### Windows / macOS

```bash
npm run tauri build
```

Gere no SO de destino (ou CI com runners `windows-latest` / `macos-latest`).  
Detalhes: [docs/building.md](./docs/building.md) e [docs/distribution.md](./docs/distribution.md).

### Todos os instaladores no GitHub (AppImage, deb, rpm, exe, dmg)

Não precisa de 3 PCs. O GitHub Actions compila em Linux + Windows + macOS e anexa à **Release**:

```bash
# 1. Push do código
git add -A && git commit -m "Release v0.1.0" && git push origin main

# 2. Tag (dispara o workflow)
git tag v0.1.0
git push origin v0.1.0
```

Depois: **Actions** → aguarde verde → **Releases** → publique o draft.

Guia completo: **[docs/github-release.md](./docs/github-release.md)**.

---

## Scripts npm

| Script | Descrição |
|--------|-----------|
| `npm run tauri dev` | Desenvolvimento desktop (use este) |
| `npm run tauri:build:linux` | Release Linux com AppImage (fix `NO_STRIP`) |
| `npm run tauri:build:appimage` | Só AppImage |
| `npm run tauri:build:deb` | Só pacote `.deb` |
| `npm run tauri build` | Build genérico do SO atual |
| `npm run dev` | Só frontend Vite (sem backend) |
| `npm run build` | Build estático SvelteKit → `build/` |
| `npm run check` | Typecheck Svelte + TypeScript |

---

## Onde ficam os dados

Arquivo: **`kanban.db`**

| SO | Pasta típica |
|----|----------------|
| Linux | `~/.local/share/com.testeapp.kanban/` |
| macOS | `~/Library/Application Support/com.testeapp.kanban/` |
| Windows | `%APPDATA%\com.testeapp.kanban\` |

**Backup:** copie `kanban.db` com o app fechado.  
**Reset (dev):** apague a pasta acima.

Identifier: `com.testeapp.kanban` (`src-tauri/tauri.conf.json`).

---

## Estrutura do repositório

```text
testeAppKanban/
├── src/                      # Frontend SvelteKit
│   ├── lib/
│   │   ├── api.ts            # Wrappers invoke() → Rust
│   │   ├── types.ts          # Tipos TypeScript
│   │   ├── dnd.ts            # Drag-and-drop (estado em memória)
│   │   ├── boardMutations.ts # Updates otimistas
│   │   ├── search.ts         # Filtro client-side
│   │   └── components/       # BoardList, BoardView, ListColumn, Card*
│   └── routes/
├── src-tauri/                # Backend Tauri + Rust
│   ├── src/
│   │   ├── commands.rs       # IPC (boards, lists, cards, labels, checklist)
│   │   ├── db.rs             # SQLite + migrations
│   │   ├── models.rs
│   │   └── error.rs
│   ├── tauri.conf.json
│   └── icons/
├── docs/                     # Documentação completa
├── package.json
└── README.md
```

---

## Documentação

| Documento | Conteúdo |
|-----------|----------|
| [docs/README.md](./docs/README.md) | Índice da documentação |
| [Guia do usuário](./docs/user-guide.md) | Como usar o app |
| [Getting started](./docs/getting-started.md) | Pré-requisitos e setup |
| [Desenvolvimento](./docs/development.md) | Fluxo diário de dev |
| [Arquitetura](./docs/architecture.md) | Camadas e decisões |
| [Modelo de dados](./docs/data-model.md) | Schema SQLite |
| [API IPC](./docs/api.md) | Comandos Tauri |
| [Build](./docs/building.md) | Instaladores por SO |
| [Release no GitHub](./docs/github-release.md) | AppImage, deb, rpm, exe, dmg via CI |
| [Distribuição](./docs/distribution.md) | Releases e signing |
| [Troubleshooting](./docs/troubleshooting.md) | Problemas comuns |
| [Próximos passos](./docs/next-steps.md) | Checklist pós-MVP |
| [Roadmap](./docs/roadmap.md) | Visão de produto |
| [CHANGELOG](./CHANGELOG.md) | Histórico de versões |
| [CONTRIBUTING](./CONTRIBUTING.md) | Como contribuir |

---

## Problemas comuns

| Sintoma | Solução |
|---------|---------|
| `Cannot read properties of undefined (reading 'invoke')` | Use `npm run tauri dev`, não o browser em `localhost:1420` |
| AppImage: `failed to run linuxdeploy` | `npm run tauri:build:linux` (define `NO_STRIP=1`) |
| UI / cliques “estranhos” | Não use CSS `zoom` global; reinicie o app após atualizar |
| Perdi os boards | Confira o path do `kanban.db` (identifier mudou?) |

Mais: [docs/troubleshooting.md](./docs/troubleshooting.md).

---

## Roadmap (resumo)

- [x] MVP local-first (boards / lists / cards)
- [x] Toasts, atalhos, UI otimista
- [x] Labels, busca, checklist
- [ ] Datas de vencimento, temas
- [ ] Sync multi-dispositivo (CRDT / `yrs`)
- [ ] Mobile (Tauri 2 Android / iOS)

Detalhes: [docs/next-steps.md](./docs/next-steps.md).

---

## Contribuindo

1. Fork / branch a partir de `main`
2. `npm install && npm run tauri dev`
3. `npm run check` e `cd src-tauri && cargo check`
4. Abra um PR descrevendo o *quê* e o *porquê*

Veja [CONTRIBUTING.md](./CONTRIBUTING.md).

---

## Licença

MIT — veja [LICENSE](./LICENSE).

---

<p align="center">
  <sub>Local-first · Tauri · Rust · Svelte</sub>
</p>
