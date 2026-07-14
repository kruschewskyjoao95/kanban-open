# Modelo de dados

Schema SQLite e regras de domínio.

## Visão geral

```
boards 1──* lists 1──* cards
boards 1──* labels
cards  *──* labels   (via card_labels)
cards  1──* checklist_items
```

Cascade: apagar um **board** apaga listas, cards e labels; apagar uma **list** apaga seus cards; apagar **card** apaga checklist e vínculos de labels; apagar **label** remove vínculos em `card_labels`.

## Tabelas

### `boards`

| Coluna | Tipo | Descrição |
|--------|------|-----------|
| `id` | `TEXT` PK | UUID v4 |
| `name` | `TEXT` | Nome exibido |
| `created_at` | `TEXT` | ISO-8601 (RFC 3339) |
| `updated_at` | `TEXT` | ISO-8601; atualizado em mutações relevantes |

### `lists`

| Coluna | Tipo | Descrição |
|--------|------|-----------|
| `id` | `TEXT` PK | UUID v4 |
| `board_id` | `TEXT` FK → boards | Board dono |
| `title` | `TEXT` | Título da coluna |
| `position` | `REAL` | Ordem na horizontal |
| `created_at` | `TEXT` | ISO-8601 |

Índice: `idx_lists_board (board_id)`.

### `cards`

| Coluna | Tipo | Descrição |
|--------|------|-----------|
| `id` | `TEXT` PK | UUID v4 |
| `list_id` | `TEXT` FK → lists | Lista dona |
| `title` | `TEXT` | Título |
| `description` | `TEXT` | Corpo (plain text); default `''` |
| `position` | `REAL` | Ordem na vertical |
| `created_at` | `TEXT` | ISO-8601 |
| `updated_at` | `TEXT` | ISO-8601 |

Índice: `idx_cards_list (list_id)`.

### `labels`

| Coluna | Tipo | Descrição |
|--------|------|-----------|
| `id` | `TEXT` PK | UUID v4 |
| `board_id` | `TEXT` FK → boards | Board dono |
| `name` | `TEXT` | Nome da etiqueta |
| `color` | `TEXT` | Hex `#RGB` ou `#RRGGBB` |
| `created_at` | `TEXT` | ISO-8601 |

Índice: `idx_labels_board (board_id)`.

### `card_labels`

| Coluna | Tipo | Descrição |
|--------|------|-----------|
| `card_id` | `TEXT` FK → cards | |
| `label_id` | `TEXT` FK → labels | |
| PK | `(card_id, label_id)` | |

Índice: `idx_card_labels_label (label_id)`.

### `checklist_items`

| Coluna | Tipo | Descrição |
|--------|------|-----------|
| `id` | `TEXT` PK | UUID v4 |
| `card_id` | `TEXT` FK → cards | Card dono |
| `title` | `TEXT` | Texto do item |
| `done` | `INTEGER` | 0/1 (bool) |
| `position` | `REAL` | Ordem |
| `created_at` | `TEXT` | ISO-8601 |

Índice: `idx_checklist_card (card_id)`.

## DDL (equivalente às migrations)

Definido em `src-tauri/src/db.rs` no startup:

```sql
CREATE TABLE IF NOT EXISTS boards (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS lists (
    id TEXT PRIMARY KEY NOT NULL,
    board_id TEXT NOT NULL,
    title TEXT NOT NULL,
    position REAL NOT NULL,
    created_at TEXT NOT NULL,
    FOREIGN KEY (board_id) REFERENCES boards(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS cards (
    id TEXT PRIMARY KEY NOT NULL,
    list_id TEXT NOT NULL,
    title TEXT NOT NULL,
    description TEXT NOT NULL DEFAULT '',
    position REAL NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    FOREIGN KEY (list_id) REFERENCES lists(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS labels (
    id TEXT PRIMARY KEY NOT NULL,
    board_id TEXT NOT NULL,
    name TEXT NOT NULL,
    color TEXT NOT NULL,
    created_at TEXT NOT NULL,
    FOREIGN KEY (board_id) REFERENCES boards(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS card_labels (
    card_id TEXT NOT NULL,
    label_id TEXT NOT NULL,
    PRIMARY KEY (card_id, label_id),
    FOREIGN KEY (card_id) REFERENCES cards(id) ON DELETE CASCADE,
    FOREIGN KEY (label_id) REFERENCES labels(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_lists_board ON lists(board_id);
CREATE INDEX IF NOT EXISTS idx_cards_list ON cards(list_id);
CREATE INDEX IF NOT EXISTS idx_labels_board ON labels(board_id);

CREATE TABLE IF NOT EXISTS checklist_items (
    id TEXT PRIMARY KEY NOT NULL,
    card_id TEXT NOT NULL,
    title TEXT NOT NULL,
    done INTEGER NOT NULL DEFAULT 0,
    position REAL NOT NULL,
    created_at TEXT NOT NULL,
    FOREIGN KEY (card_id) REFERENCES cards(id) ON DELETE CASCADE
);
```

## Regras de negócio

### Board novo

Ao chamar `create_board`, o backend cria automaticamente três listas:

| Título | `position` |
|--------|------------|
| To Do | 0 |
| Doing | 1 |
| Done | 2 |

E quatro etiquetas padrão:

| Nome | Cor |
|------|-----|
| Bug | `#ef4444` |
| Feature | `#22c55e` |
| Melhoria | `#3b82f6` |
| Urgente | `#f97316` |

> Boards criados **antes** desta feature não recebem defaults — use o painel **Etiquetas**.

### Validação

- Nome de board, título de lista, título de card e nome de label **não podem ser vazios** (após `trim`)
- Cor de label: `#RGB` ou `#RRGGBB`
- IDs inexistentes → erro `not found: …`

### Ordenação

```sql
-- boards
ORDER BY updated_at DESC

-- lists de um board
ORDER BY position ASC

-- cards de uma lista
ORDER BY position ASC
```

### Posições

Ver [Arquitetura — posição fracionária](./architecture.md#posição-fracionária).

Helper no frontend: `positionBetween(before?, after?)` em `src/lib/api.ts`.

## Tipos TypeScript

Espelho em `src/lib/types.ts`:

```ts
interface Board {
  id: string;
  name: string;
  created_at: string;
  updated_at: string;
}

interface List {
  id: string;
  board_id: string;
  title: string;
  position: number;
  created_at: string;
}

interface Label {
  id: string;
  board_id: string;
  name: string;
  color: string;
  created_at: string;
}

interface ChecklistItem {
  id: string;
  card_id: string;
  title: string;
  done: boolean;
  position: number;
  created_at: string;
}

interface Card {
  id: string;
  list_id: string;
  title: string;
  description: string;
  position: number;
  created_at: string;
  updated_at: string;
  labels: Label[];
  checklist: ChecklistItem[];
}

interface ListWithCards extends List {
  cards: Card[];
}

interface BoardFull {
  board: Board;
  labels: Label[];
  lists: ListWithCards[];
}
```

No JSON de `get_board`, cada item de `lists` vem **flattened** (campos da lista + `cards[]`). Cada card inclui `labels[]`; o board traz o catálogo em `labels`.

## Arquivo físico

| | |
|--|--|
| Nome | `kanban.db` |
| Local | `app.path().app_data_dir()` + `kanban.db` |
| Engine | SQLite (bundled via sqlx / libsqlite3-sys) |

Paths por SO: [Configuração](./configuration.md#dados-locais).

## Evolução do schema

Hoje: `CREATE TABLE IF NOT EXISTS` apenas.

Para mudanças futuras, opções:

1. **sqlx migrate** — pasta `migrations/*.sql` versionadas
2. **pragma user_version** — migrations manuais no `db.rs`

Ao alterar colunas de forma incompatível em dev, apague `kanban.db`.

## Extensões planejadas (não implementadas)

| Entidade | Uso |
|----------|-----|
| `checklists` / `checklist_items` | Subtarefas |
| `attachments` | Arquivos no filesystem + path no DB |
| `activity` | Histórico / audit log |
