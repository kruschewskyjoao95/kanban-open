# Documentação — Kanban

Índice da documentação do projeto.

## Para quem está começando

1. [Guia do usuário](./user-guide.md) — usar o app sem programar
2. [Getting started](./getting-started.md) — instalar dependências e rodar o app
3. [Desenvolvimento](./development.md) — fluxo de trabalho e scripts
4. [Troubleshooting](./troubleshooting.md) — se algo falhar

## Para entender o sistema

1. [Arquitetura](./architecture.md) — visão geral e decisões
2. [Modelo de dados](./data-model.md) — SQLite, IDs, posições
3. [API IPC](./api.md) — comandos Rust expostos ao frontend
4. [Configuração](./configuration.md) — arquivos de config e paths

## Para publicar

1. [Build e instaladores](./building.md) — gerar no seu SO
2. **[Release no GitHub](./github-release.md)** — AppImage + deb + rpm + exe + dmg via Actions
3. [Distribuição](./distribution.md) — CI, signing, lojas

## Planejamento

- [Próximos passos](./next-steps.md) — checklist priorizado, fases e critérios de pronto
- [Roadmap](./roadmap.md) — visão de produto
- [Contribuindo](../CONTRIBUTING.md)

## Mapa do repositório

```
testeAppKanban/
├── README.md                 # Entrada do projeto
├── CONTRIBUTING.md
├── LICENSE
├── docs/                     # Esta pasta
├── src/                      # Frontend (SvelteKit)
│   ├── lib/
│   │   ├── api.ts            # Wrappers invoke()
│   │   ├── types.ts
│   │   ├── dnd.ts
│   │   └── components/
│   └── routes/
├── src-tauri/                # Backend Tauri + Rust
│   ├── src/
│   │   ├── commands.rs
│   │   ├── db.rs
│   │   ├── models.rs
│   │   ├── error.rs
│   │   ├── lib.rs
│   │   └── main.rs
│   ├── capabilities/
│   ├── icons/
│   ├── tauri.conf.json
│   └── Cargo.toml
├── package.json
├── svelte.config.js
└── vite.config.js
```

## Convenções

- Documentação em **português (Brasil)**
- Código e identificadores em **inglês** (padrão da indústria)
- Exemplos de shell testados em Linux; Windows/macOS indicados quando diferem
