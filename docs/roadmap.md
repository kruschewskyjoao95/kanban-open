# Roadmap

Estado atual e visão de produto.

> **Checklist acionável (o que fazer agora, arquivos, critérios):**  
> → [next-steps.md](./next-steps.md)

## Feito (MVP 0.1)

- [x] Scaffold Tauri 2 + SvelteKit + TypeScript
- [x] SQLite local (`sqlx`) com boards / lists / cards
- [x] CRUD completo via IPC
- [x] Listas padrão To Do / Doing / Done
- [x] Drag-and-drop de cards
- [x] Modal de edição de card
- [x] UI dark básica
- [x] Documentação do projeto

## Feito (Fase 1 — UX)

- [x] Toasts de sucesso / erro
- [x] Reordenar listas por drag
- [x] Atalhos de teclado (`n`, `l`, `?`, `Esc`)
- [x] Update otimista na UI com rollback

## Feito (Fase 2.1 — Labels)

- [x] Tabelas `labels` / `card_labels`
- [x] Etiquetas padrão em board novo
- [x] Painel de gestão + chips no card + seletor no modal

## Feito (Fase 2.2 — Busca)

- [x] Busca client-side no board (título, descrição, etiquetas)
- [x] Atalho `/`, contagem de resultados, DnD desativado ao filtrar

## Feito (Fase 2.3 — Checklist)

- [x] Itens de checklist por card (CRUD + toggle)
- [x] Progresso no modal e badge no card

## Curto prazo

- [ ] Testes unitários Rust (DB com tempfile)
- [ ] Migrations versionadas (sqlx migrate)
- [ ] CI de PR validado no GitHub

## Médio prazo

- [ ] Checklist dentro do card
- [ ] Datas de vencimento e filtros
- [ ] Temas (claro/escuro/sistema)
- [ ] Export / import JSON ou CSV
- [ ] Migrations versionadas (sqlx migrate)
- [ ] Ícone e branding finais
- [ ] Instaladores assinados (Windows + macOS notarized)
- [ ] GitHub Releases automatizado (já há workflow base)

## Longo prazo

- [ ] Sync multi-dispositivo com CRDT (`yrs`)
- [ ] Servidor opcional de relay (self-hosted)
- [ ] Anexos de arquivo
- [ ] Colaboração em tempo real
- [ ] Mobile Android / iOS (Tauri 2 mobile)
- [ ] Auto-updater Tauri

## Fora de escopo (proposital)

Não competir com Notion de frente:

- Blocks aninhados tipo documento
- Databases relacionais complexas
- Wiki / páginas hierárquicas completas

Foco: **Kanban excelente, local-first, multiplataforma**.

## Como contribuir no roadmap

Abra issue com label `enhancement` ou envie PR. Prioridade relativa:

1. Correção de bugs de dados / perda de informação  
2. UX core do board (drag, teclado, feedback)  
3. Features de organização (labels, filtros)  
4. Sync / mobile  

Ver [CONTRIBUTING.md](../CONTRIBUTING.md).
