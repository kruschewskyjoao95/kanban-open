# Troubleshooting

Problemas comuns e como resolver.

## Desenvolvimento

### `tauri dev` falha: webkit / gtk não encontrado (Linux)

**Sintoma:** erro de `pkg-config` ou linking `webkit2gtk`.

**Solução:** instale deps do [Getting started](./getting-started.md#linux). Confirme:

```bash
pkg-config --exists webkit2gtk-4.1 && echo OK
```

### Porta 1420 em uso

**Sintoma:** `Port 1420 is already in use`.

```bash
# achar processo
ss -ltnp | grep 1420
# ou
fuser -k 1420/tcp
```

Ou altere a porta em `vite.config.js` e `tauri.conf.json` → `devUrl`.

### Frontend no browser: `invoke` falha

**Esperado.** `npm run dev` sozinho não tem backend Tauri. Use `npm run tauri dev`.

### App abre em branco

1. Abra DevTools do webview (se habilitado) / logs do terminal
2. Confirme que Vite está em `http://localhost:1420`
3. Rode `npm run check` para erros de compile Svelte
4. Limpe e reinstale: `rm -rf node_modules && npm install`

### Rust recompila eternamente

- Salvar arquivos em `src-tauri/target` não deveria; o Vite ignora `src-tauri` no watch do frontend
- Antivírus no Windows pode travar o `target/` — exclua a pasta

### Erro de SQL / foreign keys

- DB antigo incompatível: apague `kanban.db` (ver paths em [Configuração](./configuration.md))
- Confirme que `foreign_keys(true)` está ativo (já está no `db.rs`)

### `not found: board …` após reload

Estado da UI com ID stale (board apagado). Volte à lista de boards.

### Drag-and-drop não funciona

- Alguns ambientes Wayland/X11 com webview têm quirks de HTML5 DnD
- Teste em outra sessão / distro
- Confirme que o drop é sobre a coluna (lista), não só fora da janela

## Build

### `npm run tauri build` falha no frontend

Rode isolado:

```bash
npm run build
```

Corrija erros do adapter-static / Svelte antes do bundle.

### Bundle deb/rpm falha no Linux

Gere só AppImage:

```bash
npm run tauri build -- --bundles appimage
```

### Windows: erro de link MSVC

Instale **Desktop development with C++** / Build Tools com workload C++.

### macOS: “app is damaged” / não abre

Build **sem** notarization. Opções:

1. Clique direito → Abrir
2. Ou: `xattr -cr /path/to/Kanban.app`
3. Ou assinar + notarizar (produção)

### AppImage: `failed to run linuxdeploy`

No Arch (e similares), o bundler AppImage falha ao “stripar” libs com `.relr.dyn`.

```bash
NO_STRIP=1 APPIMAGE_EXTRACT_AND_RUN=1 npm run tauri build
# ou
npm run tauri:build:linux
```

Alternativas sem AppImage:

```bash
npm run tauri:build:deb   # .deb
# binário direto (sem instalador):
src-tauri/target/release/kanban
```

Pacotes que **já costumam ser gerados** mesmo quando o AppImage falha:

- `src-tauri/target/release/bundle/deb/Kanban_*.deb`
- `src-tauri/target/release/bundle/rpm/Kanban-*.rpm`

### AppImage não executa

```bash
chmod +x Kanban_*.AppImage
# se FUSE faltar em distros novas:
./Kanban_*.AppImage --appimage-extract-and-run
```

## Dados

### Perdi meus boards

Verifique se o `identifier` mudou (`com.testeapp.kanban`). O DB fica no path antigo.

### Como inspecionar o SQLite

```bash
# Linux
sqlite3 ~/.local/share/com.testeapp.kanban/kanban.db
.tables
SELECT * FROM boards;
```

App deve estar **fechado** se for escrever; leitura com app aberto pode funcionar (cuidado com locks).

### Corrompi o DB

Restaure backup ou apague o arquivo e recomece.

## Performance

### Primeira compilação muito lenta

Normal (centenas de crates). Use:

```bash
# mold / sccache (opcional, avançado)
```

Depois do primeiro `target/`, incremental é bem mais rápido.

### App lento ao mover card

MVP recarrega o board inteiro após cada mutação. Otimização futura: update otimista no estado local.

## Logs

- Terminal onde rodou `tauri dev` / `tauri build`
- `RUST_LOG=debug` (se instrumentar com tracing no futuro)
- Linux: `journalctl --user` raramente captura app GUI local

## Ainda travado?

1. `npm run check`
2. `cd src-tauri && cargo check 2>&1 | less`
3. Apagar `src-tauri/target` e rebuild (último recurso; demora)
4. Abrir issue com: SO, versões (`rustc -V`, `node -v`), log completo
