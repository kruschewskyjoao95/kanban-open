# API IPC (comandos Tauri)

Referência dos commands Rust expostos ao frontend via `invoke`.

Implementação: `src-tauri/src/commands.rs`  
Cliente TS: `src/lib/api.ts`

## Convenções

| Aspecto | Padrão |
|---------|--------|
| Transporte | Tauri IPC |
| Formato | JSON |
| Campos de payload | **snake_case** (serde default) |
| Datas | strings RFC 3339 |
| IDs | UUID v4 string |
| Erros | string (`AppError` serializado) |
| Sucesso vazio | `null` / `undefined` no JS (`Result<(), _>`) |

Chamada genérica:

```ts
import { invoke } from "@tauri-apps/api/core";
await invoke<ReturnType>("command_name", { /* args */ });
```

Ou via wrapper:

```ts
import { api } from "$lib/api";
await api.listBoards();
```

---

## Boards

### `list_boards`

Lista todos os boards, mais recentes primeiro (`updated_at DESC`).

| | |
|--|--|
| Args | — |
| Retorno | `Board[]` |

```ts
api.listBoards()
// invoke("list_boards")
```

### `create_board`

Cria board e três listas padrão (To Do, Doing, Done).

| | |
|--|--|
| Args | `input: { name: string }` |
| Retorno | `Board` |
| Erros | nome vazio |

```ts
api.createBoard("Meu projeto")
// invoke("create_board", { input: { name: "Meu projeto" } })
```

### `get_board`

Board completo com labels do board, listas e cards (cada card com `labels[]`).

| | |
|--|--|
| Args | `id: string` |
| Retorno | `BoardFull` |
| Erros | board não encontrado |

```ts
api.getBoard(boardId)
// invoke("get_board", { id: boardId })
```

Formato de retorno:

```json
{
  "board": { "id": "...", "name": "...", "created_at": "...", "updated_at": "..." },
  "labels": [
    { "id": "...", "board_id": "...", "name": "Bug", "color": "#ef4444", "created_at": "..." }
  ],
  "lists": [
    {
      "id": "...",
      "board_id": "...",
      "title": "To Do",
      "position": 0,
      "created_at": "...",
      "cards": [
        {
          "id": "...",
          "list_id": "...",
          "title": "...",
          "description": "",
          "position": 0,
          "created_at": "...",
          "updated_at": "...",
          "labels": [],
          "checklist": []
        }
      ]
    }
  ]
}
```

### `rename_board`

| | |
|--|--|
| Args | `input: { id: string, name: string }` |
| Retorno | `Board` |
| Side effect | atualiza `updated_at` |

```ts
api.renameBoard(id, "Novo nome")
```

### `delete_board`

Remove board e, por cascade, listas e cards.

| | |
|--|--|
| Args | `id: string` |
| Retorno | `void` |

```ts
api.deleteBoard(id)
```

---

## Lists

### `create_list`

| | |
|--|--|
| Args | `input: { board_id: string, title: string }` |
| Retorno | `List` |
| Posição | `max(position) + 1` no board |

```ts
api.createList(boardId, "Backlog")
```

### `rename_list`

| | |
|--|--|
| Args | `input: { id: string, title: string }` |
| Retorno | `List` |

```ts
api.renameList(listId, "Em progresso")
```

### `delete_list`

| | |
|--|--|
| Args | `id: string` |
| Retorno | `void` |
| Cascade | apaga cards da lista |

```ts
api.deleteList(listId)
```

### `reorder_list`

Atualiza apenas `position` da lista.

| | |
|--|--|
| Args | `input: { id: string, position: number }` |
| Retorno | `List` |

```ts
api.reorderList(listId, 1.5)
```

> **Nota UI:** o frontend MVP ainda não expõe drag de colunas; o command já existe para uso futuro.

---

## Cards

### `create_card`

| | |
|--|--|
| Args | `input: { list_id: string, title: string, description?: string \| null }` |
| Retorno | `Card` |
| Posição | `max(position) + 1` na lista |

```ts
api.createCard(listId, "Implementar login", "OAuth + e-mail")
```

### `update_card`

Atualização parcial: campos `null`/omitidos no sentido de “não mudar” via `Option` no Rust.

| | |
|--|--|
| Args | `input: { id: string, title?: string \| null, description?: string \| null }` |
| Retorno | `Card` |

```ts
api.updateCard(cardId, "Novo título", "Nova descrição")
// Só título: o wrapper atual envia description: null → limpa description.
// Para partial fino no futuro, ajustar UpdateCardInput / api.ts.
```

> **Comportamento atual do wrapper TS:** `updateCard` envia `title ?? null` e `description ?? null`. Se você omitir um argumento (`undefined`), o Rust recebe `None` e **não** altera aquele campo. Se passar string vazia em description, grava `""`.

### `delete_card`

| | |
|--|--|
| Args | `id: string` |
| Retorno | `void` |

```ts
api.deleteCard(cardId)
```

### `move_card`

Move para outra lista e/ou outra posição.

| | |
|--|--|
| Args | `input: { id: string, list_id: string, position: number }` |
| Retorno | `Card` |

```ts
import { positionBetween } from "$lib/api";

const position = positionBetween(beforePos, afterPos);
api.moveCard(cardId, targetListId, position)
```

---

## Tipos de erro

`AppError` (Rust) serializa para string:

| Prefixo / forma | Significado |
|-----------------|-------------|
| `database error: …` | sqlx / SQLite |
| `io error: …` | filesystem |
| `not found: board|list|card <id>` | entidade inexistente |
| mensagem livre | validação (ex.: nome vazio) |

No frontend:

```ts
try {
  await api.createBoard("");
} catch (e) {
  console.error(String(e)); // "board name cannot be empty"
}
```

---

## Labels

### `list_labels`

| | |
|--|--|
| Args | `board_id: string` |
| Retorno | `Label[]` |

```ts
api.listLabels(boardId)
```

### `create_label`

| | |
|--|--|
| Args | `input: { board_id, name, color }` |
| Retorno | `Label` |
| Erros | nome vazio; cor inválida (`#RGB` / `#RRGGBB`) |

```ts
api.createLabel(boardId, "Bug", "#ef4444")
```

### `update_label`

| | |
|--|--|
| Args | `input: { id, name?, color? }` |
| Retorno | `Label` |

```ts
api.updateLabel(labelId, "Crítico", "#dc2626")
```

### `delete_label`

| | |
|--|--|
| Args | `id: string` |
| Retorno | `void` |
| Cascade | remove de `card_labels` |

```ts
api.deleteLabel(labelId)
```

### `set_card_labels`

Substitui o conjunto de etiquetas do card.

| | |
|--|--|
| Args | `input: { card_id, label_ids: string[] }` |
| Retorno | `Card` (com `labels` atualizado) |
| Erros | card/label inexistente; label de outro board |

```ts
api.setCardLabels(cardId, [labelId1, labelId2])
```

---

## Checklist

### `create_checklist_item`

| | |
|--|--|
| Args | `input: { card_id, title }` |
| Retorno | `ChecklistItem` |

```ts
api.createChecklistItem(cardId, "Escrever testes")
```

### `update_checklist_item`

| | |
|--|--|
| Args | `input: { id, title?, done? }` |
| Retorno | `ChecklistItem` |

```ts
api.updateChecklistItem(itemId, { done: true })
api.updateChecklistItem(itemId, { title: "Novo texto" })
```

### `delete_checklist_item`

| | |
|--|--|
| Args | `id: string` |
| Retorno | `void` |

```ts
api.deleteChecklistItem(itemId)
```

Cada `Card` em `get_board` inclui `checklist: ChecklistItem[]` ordenado por `position`.

---

## Helper: `positionBetween`

Não é command Tauri; vive no frontend.

```ts
positionBetween(undefined, undefined) // 0
positionBetween(undefined, 5)         // 4  (antes do primeiro)
positionBetween(5, undefined)         // 6  (depois do último)
positionBetween(2, 4)                 // 3  (meio)
```

---

## Registro no backend

Em `src-tauri/src/lib.rs`:

```rust
.invoke_handler(tauri::generate_handler![
    commands::list_boards,
    commands::create_board,
    commands::get_board,
    commands::rename_board,
    commands::delete_board,
    commands::create_list,
    commands::rename_list,
    commands::delete_list,
    commands::reorder_list,
    commands::create_card,
    commands::update_card,
    commands::delete_card,
    commands::move_card,
    commands::list_labels,
    commands::create_label,
    commands::update_label,
    commands::delete_label,
    commands::set_card_labels,
    commands::create_checklist_item,
    commands::update_checklist_item,
    commands::delete_checklist_item,
])
```

Todo command novo **precisa** ser adicionado a essa lista.
