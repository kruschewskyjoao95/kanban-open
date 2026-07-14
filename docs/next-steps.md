# Próximos passos

Plano de trabalho **acionável** depois do MVP 0.1.  
Visão de produto de mais alto nível: [roadmap.md](./roadmap.md).

Use este arquivo como checklist: marque `- [x]` conforme for concluindo e atualize o [CHANGELOG](../CHANGELOG.md).

---

## Onde estamos

| Item | Status |
|------|--------|
| App desktop Tauri + Svelte + SQLite | ✅ |
| CRUD boards / lists / cards | ✅ |
| Drag-and-drop de cards | ✅ |
| Documentação + CI base | ✅ |
| Toasts, reorder de listas, atalhos, UI otimista | ✅ (Fase 1) |
| Labels / cores nos cards | ✅ (2.1) |
| Busca no board | ✅ (2.2) |
| Checklist no card | ✅ (2.3) |
| Sync, mobile | ❌ |

**Rodar o estado atual:**

```bash
npm install
npm run tauri dev
```

---

## Ordem sugerida (prioridade)

Faça nesta ordem se estiver sozinho — maximiza valor com menos risco.

```
1. UX imediata (toasts, atalhos, reorder listas)
2. Organização (labels, busca)
3. Qualidade (testes, migrations)
4. Polish de release (ícone, instaladores)
5. Sync / mobile (só depois de 1–4 estáveis)
```

---

## Fase 1 — UX do board ✅

Implementado em 2026-07-13.

| Item | Status | Onde |
|------|--------|------|
| 1.1 Toasts sucesso/erro | ✅ | `toast.svelte.ts`, `ToastHost.svelte` |
| 1.2 Reordenar listas (drag no cabeçalho ⋮⋮) | ✅ | `BoardView`, `ListColumn`, `dnd.ts` |
| 1.3 Atalhos (`n`, `l`, `?`, `Esc`) | ✅ | `BoardView`, `BoardList` |
| 1.4 Update otimista + rollback | ✅ | `boardMutations.ts`, handlers em `BoardView` |

---

## Fase 2 — Organização (2–3 semanas)

### 2.1 Labels / cores nos cards ✅

Implementado: tabelas `labels` + `card_labels`, defaults no board novo, painel **Etiquetas**, chips no card e seletor no modal.

---

### 2.2 Busca no board ✅

Implementado: campo na topbar, atalho `/`, filtro client-side (título, descrição, etiquetas), contagem de resultados; DnD de cards desativado durante a busca.

---

### 2.3 Checklist dentro do card ✅

Implementado: tabela `checklist_items`, CRUD IPC, modal com progresso, badge `done/total` no card da coluna; busca também considera títulos dos itens.

---

## Fase 3 — Qualidade e base técnica (paralelo / 1–2 semanas)

### 3.1 Migrations versionadas

| | |
|--|--|
| Esforço | Médio |
| Abordagem | `sqlx migrate` com pasta `src-tauri/migrations/` |
| Critério de pronto | Schema novo sobe sem apagar `kanban.db` em upgrades |

**Passos:**

1. Extrair DDL atual para `migrations/001_init.sql`
2. Rodar `sqlx::migrate!` no `init_pool`
3. Documentar em `data-model.md`

---

### 3.2 Testes Rust (DB)

| | |
|--|--|
| Esforço | Médio |
| Arquivos | `src-tauri/src/` + `#[cfg(test)]` ou `tests/` |
| Critério de pronto | `cargo test` cobre create board, move card, delete cascade |

Usar SQLite em tempfile ou `:memory:`.

---

### 3.3 CI de PR (já esboçado)

Arquivo: `.github/workflows/ci.yml`

| | |
|--|--|
| Esforço | Pequeno (validar) |
| Critério de pronto | PR vermelho se `npm run check` ou `cargo check` falhar |

Confirmar no GitHub após o primeiro push do repo.

---

## Fase 4 — Release “de verdade” (1 semana + contas)

### 4.1 Branding

- [ ] Ícone 1024×1024 → `npm run tauri icon icon.png`
- [ ] Nome final / screenshot para README
- [ ] Tema claro ou preferência do sistema

### 4.2 Instaladores multi-OS

| Passo | Doc |
|-------|-----|
| Build local Linux | [building.md](./building.md) |
| CI em tag `v*` | [distribution.md](./distribution.md), `.github/workflows/release.yml` |
| Smoke test AppImage / NSIS / DMG | Máquina ou VM de cada SO |

### 4.3 Assinatura (quando for público)

- [ ] Windows Authenticode  
- [ ] macOS Developer ID + notarization  
- Ver [distribution.md](./distribution.md#code-signing)

### 4.4 Primeira release pública

```bash
# 1. Bump version em package.json, Cargo.toml, tauri.conf.json
# 2. Atualizar CHANGELOG.md
git tag v0.2.0
git push origin v0.2.0
# 3. Revisar draft release no GitHub e publicar
```

---

## Fase 5 — Sync multi-dispositivo (mês+ / depois de estável)

Não comece cedo demais — muda o modelo mental do app.

| Etapa | Detalhe |
|-------|---------|
| 5.1 | Escolher CRDT: `yrs` (Yjs em Rust) recomendado |
| 5.2 | Mapear boards/lists/cards → shared types Yjs |
| 5.3 | SQLite como cache / persistência de snapshot |
| 5.4 | Servidor relay mínimo (WebSocket) — opcional self-host |
| 5.5 | Resolução offline e testes de conflito |

Docs a criar depois: `docs/sync.md`.

---

## Fase 6 — Mobile (depois do sync ou em paralelo experimental)

| Etapa | Detalhe |
|-------|---------|
| 6.1 | `npm run tauri android init` / iOS no macOS |
| 6.2 | Ajustar UI touch (colunas scroll, modal full-screen) |
| 6.3 | Paths de DB e permissões |
| 6.4 | Builds de loja (Play / App Store) — longo prazo |

Referência: [Tauri mobile](https://v2.tauri.app/start/create-project/).

---

## Backlog rápido (sem ordem fixa)

Copie para issues quando for atacar:

- [ ] Datas de vencimento + filtro “atrasados”
- [ ] Export / import JSON do board
- [ ] Arquivar card (soft delete) em vez de só excluir
- [ ] Contagem de cards por lista (já tem badge; gráfico opcional)
- [ ] Undo (Ctrl+Z) da última ação
- [ ] Multi-janela / abrir board em janela separada
- [ ] Auto-updater Tauri
- [ ] Flatpak / Flathub
- [ ] Anexos de arquivo no card
- [ ] Preferências (pasta do DB customizada)

---

## Fora de escopo (não faça agora)

Para não virar um clone de Notion de 2 anos:

- Editor de blocos aninhados  
- Wiki / páginas hierárquicas  
- Database relacional tipo Notion  
- Realtime multiplayer completo no dia 1  

Foco: **Kanban local-first excelente** → depois sync → depois mobile.

---

## Próxima ação concreta (agora)

Fase 2 (organização) concluída. Próximo valor alto:

1. **Migrations versionadas** (3.1)  
2. **Testes Rust** (3.2)  
3. **Datas de vencimento** (backlog) ou polish de release (Fase 4)

```bash
npm run tauri dev
```

---

## Manutenção deste arquivo

| Quando | O quê |
|--------|--------|
| Ao concluir item | Marcar `[x]` e mover nota para `CHANGELOG.md` |
| Ao mudar prioridade | Reordenar seções / avisar no PR |
| A cada release | Revisar se a “Próxima ação concreta” ainda faz sentido |

Relacionados: [roadmap.md](./roadmap.md) · [development.md](./development.md) · [CONTRIBUTING.md](../CONTRIBUTING.md)
