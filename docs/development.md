# Desenvolvimento

Fluxo de trabalho diário no código do Kanban.

## Comandos principais

| Comando | Quando usar |
|---------|-------------|
| `npm run tauri dev` | Desenvolvimento normal do app desktop |
| `npm run dev` | Só UI, sem Rust |
| `npm run check` | Typecheck Svelte + TS |
| `npm run check:watch` | Typecheck contínuo |
| `npm run build` | Build estático do frontend (`build/`) |
| `npm run tauri build` | Release + bundles |
| `cd src-tauri && cargo check` | Validar Rust rápido |
| `cd src-tauri && cargo clippy` | Lints Rust (se instalado) |
| `cd src-tauri && cargo fmt` | Formatar Rust |

## Estrutura de pastas (trabalho)

### Frontend — `src/`

| Caminho | Papel |
|---------|--------|
| `routes/+page.svelte` | Shell: lista de boards ↔ board ativo |
| `routes/+layout.ts` | `ssr = false` (SPA para Tauri) |
| `lib/api.ts` | Todos os `invoke()` para o Rust |
| `lib/types.ts` | Tipos TS espelhando o backend |
| `lib/dnd.ts` | Payload HTML5 drag-and-drop |
| `lib/components/BoardList.svelte` | Home / lista de boards |
| `lib/components/BoardView.svelte` | Board com colunas |
| `lib/components/ListColumn.svelte` | Lista + drop zone |
| `lib/components/CardItem.svelte` | Card na coluna |
| `lib/components/CardModal.svelte` | Edição de card |
| `app.css` | Tokens de design / CSS global |

### Backend — `src-tauri/src/`

| Caminho | Papel |
|---------|--------|
| `main.rs` | Entry point do binário |
| `lib.rs` | `run()`, setup do DB, registro de commands |
| `commands.rs` | Handlers `#[tauri::command]` |
| `db.rs` | Pool SQLite + migrations |
| `models.rs` | Structs + inputs serde |
| `error.rs` | `AppError` serializável para o frontend |

## Padrão de feature nova

Exemplo: adicionar **labels** nos cards.

1. **Schema** — migration em `db.rs` (`ALTER TABLE` ou nova tabela)
2. **Models** — campos em `models.rs`
3. **Commands** — create/update/list em `commands.rs` + registrar em `lib.rs`
4. **Types + API** — `types.ts` e `api.ts`
5. **UI** — componentes Svelte
6. **Docs** — atualizar `docs/api.md` e `docs/data-model.md`

## IPC (frontend ↔ Rust)

O frontend **nunca** acessa o SQLite direto. Sempre:

```ts
import { api } from "$lib/api";

const boards = await api.listBoards();
```

No Rust, o argumento `input` agrupa payloads (serde snake_case):

```ts
// Frontend
invoke("create_list", { input: { board_id: id, title } });
```

```rust
// Backend
#[tauri::command]
pub async fn create_list(db: State<'_, DbState>, input: CreateListInput) -> Result<List, AppError>
```

Erros do Rust chegam como **string** no reject do `invoke` (ver `AppError` + `Serialize`).

## Hot reload

- **Frontend:** HMR do Vite ao salvar `.svelte` / `.ts` / `.css`
- **Rust:** o Tauri recompila e reinicia o app ao salvar em `src-tauri/` (mais lento)

## Svelte 5

O projeto usa **runes**:

- `$state` — estado local
- `$props` — props do componente
- `$effect` — side effects (load de dados, sync de drafts)

Não use a API legada `export let` / stores a menos que seja necessário.

## Banco em desenvolvimento

- Arquivo: ver [Configuração — dados locais](./configuration.md#dados-locais)
- Migrations rodam em todo startup (`CREATE IF NOT EXISTS`)
- Ainda **não** há sistema versionado de migrations (sqlx migrate); mudanças destrutivas exigem apagar o DB ou migrar manualmente

## Testes (estado atual)

MVP sem suíte automatizada. Checklist manual sugerido:

1. Criar board → 3 listas padrão
2. Criar cards em cada lista
3. Arrastar card entre listas e reordenar
4. Editar título/descrição no modal
5. Renomear board e lista
6. Excluir card, lista e board
7. Fechar e reabrir o app → dados persistem

Quando adicionar testes:

- **Rust:** `cargo test` em funções puras / DB com tempfile
- **Frontend:** Vitest + Testing Library (a configurar)

## Git e branches

Sugestão:

```
main          # estável
feat/...      # features
fix/...       # correções
docs/...      # só documentação
```

Ver [CONTRIBUTING.md](../CONTRIBUTING.md).

## IDE

- **Rust:** rust-analyzer
- **Svelte:** extensão oficial Svelte
- **Tauri:** opcional [Tauri VS Code](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode)

Workspace já pode incluir `.vscode/` com recomendações de extensões.
