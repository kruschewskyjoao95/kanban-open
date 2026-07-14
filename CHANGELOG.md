# Changelog

Todas as mudanças notáveis deste projeto são documentadas aqui.

O formato é baseado em [Keep a Changelog](https://keepachangelog.com/pt-BR/1.1.0/),
e este projeto adere a [Semantic Versioning](https://semver.org/lang/pt-BR/).

## [Unreleased]

### Adicionado

- **Checklist no card**: itens com done/título, progresso, badge na coluna
  - Tabela `checklist_items`; commands `create/update/delete_checklist_item`
- **Busca no board**: filtro client-side (título, descrição, etiquetas, checklist), atalho `/`
- **Etiquetas (labels)** nos cards: cores, painel de gestão, chips na coluna
  - Tabelas `labels` + `card_labels`; defaults em board novo
  - Commands: `list_labels`, `create_label`, `update_label`, `delete_label`, `set_card_labels`
  - UI: botão **Etiquetas**, atalho `e`, seletor no modal do card
- Toasts de sucesso/erro (`ToastHost` + `toast.svelte.ts`)
- Reordenar listas por drag-and-drop (cabeçalho ⋮⋮)
- Atalhos: `n` (card/board), `l` (lista), `e` (etiquetas), `?` (ajuda), `Esc`
- Updates otimistas no board com rollback em falha (`boardMutations.ts`)
- Documentação completa em `docs/`
- [Próximos passos](docs/next-steps.md) (checklist priorizado pós-MVP)
- Workflow de release multiplataforma (GitHub Actions)
- CONTRIBUTING.md, LICENSE, CHANGELOG.md

## [0.1.0] — 2026-07-13

### Adicionado

- App Kanban local-first (Tauri 2 + Rust + SQLite + Svelte 5)
- Boards com listas padrão To Do / Doing / Done
- Cards com título e descrição
- Drag-and-drop de cards entre listas
- Persistência SQLite em `app_data_dir`
- Comandos IPC: boards, lists, cards (CRUD + move/reorder)
- UI dark desktop

[Unreleased]: https://github.com/example/kanban/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/example/kanban/releases/tag/v0.1.0
