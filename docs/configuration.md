# Configuração

Arquivos de configuração, paths e opções relevantes.

## Identidade do app

| Onde | Campo | Valor atual |
|------|-------|-------------|
| `src-tauri/tauri.conf.json` | `productName` | `Kanban` |
| | `version` | `0.1.0` |
| | `identifier` | `com.testeapp.kanban` |
| `src-tauri/Cargo.toml` | `name` | `kanban` |
| `package.json` | `name` | `kanban` |

O **identifier** define pastas de dados e IDs de pacote. Mudar depois “órfão” o DB antigo no path anterior.

## `tauri.conf.json`

### Build

```json
"build": {
  "beforeDevCommand": "npm run dev",
  "devUrl": "http://localhost:1420",
  "beforeBuildCommand": "npm run build",
  "frontendDist": "../build"
}
```

| Campo | Função |
|-------|--------|
| `beforeDevCommand` | Sobe o Vite no `tauri dev` |
| `devUrl` | URL do webview em dev |
| `beforeBuildCommand` | Build do frontend antes do release |
| `frontendDist` | Pasta estática embutida no binário |

### Janela

```json
"app": {
  "windows": [
    {
      "title": "Kanban",
      "width": 1280,
      "height": 800,
      "minWidth": 800,
      "minHeight": 500
    }
  ]
}
```

Ajuste título, tamanho e restrições aqui.

### Segurança / CSP

```json
"security": { "csp": null }
```

SPA local embutida. Se carregar conteúdo remoto no futuro, defina um CSP restritivo.

### Bundle

Ver [Building](./building.md#configuração-de-bundle).

## Capabilities (`src-tauri/capabilities/default.json`)

```json
{
  "identifier": "default",
  "windows": ["main"],
  "permissions": [
    "core:default",
    "opener:default"
  ]
}
```

Permissions Tauri 2 são **explícitas**. Novos plugins (fs, http, dialog…) exigem entradas aqui.

## Dados locais

Criado em runtime:

```rust
app.path().app_data_dir() // + "kanban.db"
```

### Paths por SO

| SO | Diretório (típico) |
|----|---------------------|
| Linux | `~/.local/share/com.testeapp.kanban/` |
| macOS | `~/Library/Application Support/com.testeapp.kanban/` |
| Windows | `%APPDATA%\com.testeapp.kanban\` |

Arquivo: **`kanban.db`**.

### Backup / restore

```bash
# Linux — backup
cp ~/.local/share/com.testeapp.kanban/kanban.db ~/backups/kanban-$(date +%F).db

# restore (app fechado)
cp ~/backups/kanban-YYYY-MM-DD.db ~/.local/share/com.testeapp.kanban/kanban.db
```

### Reset total (dev)

```bash
rm -rf ~/.local/share/com.testeapp.kanban
```

## Frontend

### `svelte.config.js`

- Adapter: `@sveltejs/adapter-static`
- `fallback: "index.html"` — SPA mode

### `src/routes/+layout.ts`

```ts
export const ssr = false;
```

Obrigatório para Tauri (sem servidor Node em produção).

### `vite.config.js`

- Porta fixa **1420** (`strictPort: true`)
- Ignora watch de `src-tauri/`
- HMR ajustado se `TAURI_DEV_HOST` (mobile/dev remoto)

### Alias `$lib`

SvelteKit padrão → `src/lib/`.

## Variáveis de ambiente

| Variável | Uso |
|----------|-----|
| `TAURI_DEV_HOST` | Host HMR em dev especial (mobile/rede) |
| `CI` | Detectado por tools em pipelines |

Não há `.env` obrigatório no MVP (sem API keys).

## Ícones

Pasta: `src-tauri/icons/`

Gerar a partir de um PNG 1024×1024:

```bash
npm run tauri icon ./path/to/app-icon.png
```

Sobrescreve `icons/` e atualiza referências no conf se necessário.

## Cargo features

`src-tauri/Cargo.toml` — dependências principais:

| Crate | Uso |
|-------|-----|
| `tauri` 2 | Shell |
| `sqlx` (sqlite, tokio) | DB |
| `serde` / `serde_json` | IPC |
| `uuid` | IDs |
| `chrono` | timestamps |
| `thiserror` | erros |

## Portas

| Porta | Serviço |
|-------|---------|
| 1420 | Vite dev server |
| 1421 | HMR WebSocket (quando host remoto) |

Se 1420 estiver ocupada, o `tauri dev` falha (`strictPort`). Mate o processo ou mude a porta em `vite.config.js` **e** `devUrl` no conf.
