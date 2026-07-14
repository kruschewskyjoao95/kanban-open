# Build e instaladores

Como gerar binários e instaladores para **Linux**, **Windows** e **macOS**.

## Regra de ouro

> Cada instalável é gerado **no SO de destino** (ou em CI com runner daquele SO).

| Você está em… | Gera nativamente… |
|---------------|-------------------|
| Linux | AppImage, deb, rpm (conforme ferramentas) |
| Windows | NSIS (`.exe`), MSI |
| macOS | `.app`, DMG |

Cross-compile Tauri (ex.: Linux → Windows) **não é recomendado** para o dia a dia.

## Build local

```bash
npm install
npm run tauri build
```

### Linux (Arch / distros recentes) — AppImage e `linuxdeploy`

Em Arch e outras distros com libs modernas, o AppImage pode falhar com:

```text
failed to bundle project: failed to run linuxdeploy
```

Causa: o `strip` embutido no `linuxdeploy` não entende seções `.relr.dyn` das libs do sistema.

**Solução:**

```bash
# AppImage + deb + rpm (recomendado no Linux)
npm run tauri:build:linux

# Só AppImage
npm run tauri:build:appimage

# Só .deb (sem linuxdeploy — costuma funcionar sem NO_STRIP)
npm run tauri:build:deb
```

Equivalente manual:

```bash
NO_STRIP=1 APPIMAGE_EXTRACT_AND_RUN=1 npm run tauri build
```

Artefato AppImage:

```text
src-tauri/target/release/bundle/appimage/Kanban_0.1.0_amd64.AppImage
```

O pipeline:

1. `npm run build` → frontend estático em `build/`
2. `cargo build --release` → binário Rust
3. Bundler Tauri → instaladores em `src-tauri/target/release/bundle/`

### Só alguns formatos

```bash
# Linux
npm run tauri build -- --bundles appimage
npm run tauri build -- --bundles deb
npm run tauri build -- --bundles rpm

# Windows (no Windows)
npm run tauri build -- --bundles nsis
npm run tauri build -- --bundles msi

# macOS (no macOS)
npm run tauri build -- --bundles app
npm run tauri build -- --bundles dmg
```

### Binário sem instalador

```bash
npm run tauri build -- --no-bundle
# binário: src-tauri/target/release/kanban  (nome do package Cargo)
```

## Artefatos por plataforma

### Linux

| Bundle | Extensão | Destino típico |
|--------|----------|----------------|
| AppImage | `.AppImage` | Qualquer distro (portátil) |
| Debian | `.deb` | Ubuntu, Debian, Pop!_OS… |
| RPM | `.rpm` | Fedora, RHEL, openSUSE… |

Caminhos:

```
src-tauri/target/release/bundle/appimage/
src-tauri/target/release/bundle/deb/
src-tauri/target/release/bundle/rpm/
```

**AppImage — uso:**

```bash
chmod +x Kanban_*.AppImage
./Kanban_*.AppImage
```

Ferramentas extras no host podem ser necessárias para deb/rpm; se falhar, use só `appimage`.

### Windows

| Bundle | Extensão | Notas |
|--------|----------|--------|
| NSIS | `.exe` | Instalador clássico |
| MSI | `.msi` | Bom para deploy corporativo |

```
src-tauri/target/release/bundle/nsis/
src-tauri/target/release/bundle/msi/
```

WebView2: em Windows 10/11 costuma já existir; o instalador Tauri pode lidar com bootstrap conforme config.

### macOS

| Bundle | Extensão | Notas |
|--------|----------|--------|
| App | `.app` | Bundle da aplicação |
| DMG | `.dmg` | Disco para download direto |

```
src-tauri/target/release/bundle/macos/
src-tauri/target/release/bundle/dmg/
```

Sem **code signing + notarization**, o Gatekeeper pode bloquear a abertura (usuário: clique direito → Abrir). Ver [Distribuição](./distribution.md).

## Configuração de bundle

Em `src-tauri/tauri.conf.json`:

```json
{
  "productName": "Kanban",
  "version": "0.1.0",
  "identifier": "com.testeapp.kanban",
  "bundle": {
    "active": true,
    "targets": "all",
    "icon": [
      "icons/32x32.png",
      "icons/128x128.png",
      "icons/128x128@2x.png",
      "icons/icon.icns",
      "icons/icon.ico"
    ]
  }
}
```

| Campo | Efeito |
|-------|--------|
| `productName` | Nome do app / instalador |
| `version` | Versão do pacote |
| `identifier` | ID único (reverse domain) |
| `targets` | `"all"` ou lista (`["appimage","deb"]`) |
| `icon` | Ícones multiplataforma |

Para limitar targets no config (em vez da CLI):

```json
"targets": ["appimage", "deb"]
```

## Versionamento

Mantenha alinhados quando for release:

1. `src-tauri/tauri.conf.json` → `version`
2. `src-tauri/Cargo.toml` → `version`
3. `package.json` → `version`

Sugestão de tag Git: `v0.1.0`.

## Otimização de release

Cargo já usa profile `release` no `tauri build`. Opcionais em `Cargo.toml`:

```toml
[profile.release]
panic = "abort"
codegen-units = 1
lto = true
opt-level = "s"   # ou 3 para velocidade
strip = true
```

Trade-off: compile mais lento × binário menor/mais rápido.

## Arquiteturas

| SO | Alvos comuns |
|----|----------------|
| Linux | `x86_64-unknown-linux-gnu`, `aarch64-unknown-linux-gnu` |
| Windows | `x86_64-pc-windows-msvc` |
| macOS | `x86_64-apple-darwin`, `aarch64-apple-darwin`, universal |

Universal macOS (Intel + Apple Silicon):

```bash
npm run tauri build -- --target universal-apple-darwin
```

(requer toolchains e libs para ambos os arches no Mac.)

## Checklist antes do build de release

- [ ] `npm run check` sem erros
- [ ] `cargo check` / app testado em `tauri dev`
- [ ] Versão atualizada
- [ ] Ícones corretos
- [ ] README / changelog (se houver) atualizados
- [ ] Teste do instalador em máquina limpa (ou VM)

## Próximo

- [Distribuição](./distribution.md) — GitHub Releases, CI, signing  
- [Troubleshooting](./troubleshooting.md) — erros de build
