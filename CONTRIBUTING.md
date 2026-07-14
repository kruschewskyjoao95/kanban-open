# Contribuindo

Obrigado por contribuir com o Kanban.

## Antes de começar

1. Leia [docs/getting-started.md](./docs/getting-started.md)
2. Rode o app com `npm run tauri dev`
3. Entenda o fluxo em [docs/architecture.md](./docs/architecture.md)

## Setup

```bash
git clone <repo>
cd testeAppKanban
npm install
npm run tauri dev
```

## Fluxo de PR

1. Crie um branch a partir de `main`:
   ```bash
   git checkout -b feat/minha-feature
   ```
2. Faça commits pequenos e claros (mensagem em PT ou EN, consistente)
3. Antes do push:
   ```bash
   npm run check
   cd src-tauri && cargo check
   ```
4. Abra o Pull Request descrevendo **o que** e **por quê**
5. Atualize documentação se mudar API, schema ou UX relevante:
   - `docs/api.md`
   - `docs/data-model.md`
   - `README.md` se afetar o início rápido

## Padrões de código

### Rust

- `cargo fmt` antes do commit
- Prefira `Result<T, AppError>` nos commands
- Não ignore erros de DB
- IDs com UUID; timestamps RFC 3339

### TypeScript / Svelte

- Svelte 5 runes (`$state`, `$props`, `$effect`)
- Tipos em `src/lib/types.ts` alinhados ao Rust
- Todo IPC novo passa por `src/lib/api.ts` (não espalhar `invoke` solto)
- UI em português no produto (labels, botões, confirmações)

### SQL

- Migrations em `db.rs` (por enquanto)
- Manter foreign keys e índices coerentes
- Documentar mudança em `docs/data-model.md`

## Escopo de PRs

| Bom | Evitar no mesmo PR |
|-----|---------------------|
| Uma feature ou um fix focado | Refactor gigante + feature |
| Docs da feature | Reformatar o repo inteiro |
| Testes quando couber | Trocar stack sem discussão |

## Issues

Use issues para:

- Bugs (passos de reprodução, SO, versão)
- Propostas de feature
- Dúvidas de arquitetura

## Código de conduta (resumo)

- Seja respeitoso
- Assuma boa intenção
- Foque no problema técnico

## Licença

Ao contribuir, você concorda que o código entra sob a licença MIT do projeto.
