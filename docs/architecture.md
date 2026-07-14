# Arquitetura

Visão geral do sistema, responsabilidades e decisões de design.

## Diagrama lógico

```
┌─────────────────────────────────────────────────────────┐
│                     Janela nativa                       │
│  ┌───────────────────────────────────────────────────┐  │
│  │  WebView (WebKitGTK / WebView2 / WKWebView)       │  │
│  │  ┌─────────────────────────────────────────────┐  │  │
│  │  │  SvelteKit SPA (Svelte 5)                   │  │  │
│  │  │  BoardList · BoardView · ListColumn · Cards │  │  │
│  │  │           ↓ invoke() / api.ts               │  │  │
│  │  └─────────────────────────────────────────────┘  │  │
│  └───────────────────────┬───────────────────────────┘  │
│                          │ IPC (Tauri commands)         │
│  ┌───────────────────────▼───────────────────────────┐  │
│  │  Rust (src-tauri)                                 │  │
│  │  commands → models → sqlx → SQLite                │  │
│  └───────────────────────┬───────────────────────────┘  │
│                          │                              │
│                    app_data_dir/kanban.db               │
└─────────────────────────────────────────────────────────┘
```

## Princípios

1. **Local-first** — dados no disco do usuário; sem servidor obrigatório.
2. **Backend fino** — Rust só para persistência, regras e (futuro) sync.
3. **UI web** — produtividade de Svelte + polish de CSS; webview nativo via Tauri.
4. **Multiplataforma** — um codebase; binários por SO.

## Camadas

### 1. Apresentação (Svelte)

- Estado de UI na árvore de componentes (`$state`)
- Não conhece SQL
- Chama `api.*` e re-renderiza com o resultado

### 2. IPC (Tauri)

- Commands assíncronos registrados em `lib.rs`
- Payload JSON (serde)
- Erros como string serializada

### 3. Domínio / persistência (Rust)

- `commands.rs` — casos de uso
- `models.rs` — DTOs e entidades
- `db.rs` — conexão e schema
- `error.rs` — erros tipados

### 4. Armazenamento (SQLite)

- Arquivo único `kanban.db`
- Foreign keys com `ON DELETE CASCADE`
- Índices em `board_id` e `list_id`

## Fluxo de um card movido

```
Usuário solta o card na ListColumn
        ↓
positionBetween(before, after)  // frontend
        ↓
api.moveCard(id, listId, position)
        ↓
invoke("move_card", { input: { id, list_id, position } })
        ↓
commands::move_card → UPDATE cards SET list_id, position
        ↓
BoardView.load() → get_board → UI atualizada
```

## Por que Tauri e não Electron?

| | Tauri 2 | Electron |
|--|---------|----------|
| Runtime UI | WebView do SO | Chromium embutido |
| Backend | Rust | Node |
| Tamanho típico | Dezenas de MB | Centenas de MB |
| RAM | Menor | Maior |

## Por que SvelteKit (e não Svelte puro)?

O template oficial Tauri 2 usa SvelteKit com `@sveltejs/adapter-static` e `ssr = false` (modo SPA). Benefícios:

- Roteamento pronto se o app crescer
- Tooling maduro
- Mesmo fluxo de build (`build/` → `frontendDist`)

## Por que sqlx e não rusqlite?

- API async alinhada a commands Tauri async
- `FromRow` + tipos
- Caminho natural para migrations versionadas depois

Trade-off: binário e tempo de compile maiores que `rusqlite`.

## Posição fracionária

Em vez de reindexar `0..n` a cada drag, cards e listas usam `position REAL`:

- Novo item: `max(position) + 1`
- Entre A e B: `(A + B) / 2`

Isso imita o modelo do Trello e evita updates em massa. Em caso de colisão por precisão float (raro no uso normal), rebalance futuro pode renumerar a lista.

## Segurança (escopo atual)

- App desktop local; sem rede por padrão
- Capabilities Tauri mínimas (`core:default`, `opener:default`)
- Sem eval de conteúdo remoto
- CSP em `tauri.conf.json` está `null` (adequado a SPA local embutida; revisar se carregar URLs externas)

## Mobile (futuro)

Tauri 2 tem suporte mobile experimental. A ideia é:

- Manter o **mesmo frontend** (webview)
- Reutilizar commands Rust
- Ajustar paths de DB e UI touch

Não está no MVP. Ver [Roadmap](./roadmap.md).

## Sync (futuro)

Camada planejada:

1. Continuar SQLite como cache local
2. Introduzir `yrs` (Yjs em Rust) ou Automerge para merges
3. Servidor opcional só para transporte (WebSocket / HTTP)

Arquitetura permanece local-first: offline funciona; sync é add-on.

## Limitações conhecidas do MVP

- Sem multi-janela / multi-usuário
- Sem anexos ou rich text
- Sem reordenação de listas por drag (API `reorder_list` existe; UI não usa ainda)
- Sem undo
- Reload completo do board após mutações (sem store global otimista)
