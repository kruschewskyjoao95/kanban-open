# Publicar instaladores no GitHub

Como gerar **AppImage, deb, rpm, exe e dmg** e anexar à release do repositório.

## Ideia geral

| Formato | SO onde é gerado | Runner do GitHub Actions |
|---------|------------------|---------------------------|
| `.AppImage` | Linux | `ubuntu-22.04` |
| `.deb` | Linux | `ubuntu-22.04` |
| `.rpm` | Linux | `ubuntu-22.04` |
| `.exe` (NSIS) | Windows | `windows-latest` |
| `.msi` | Windows | `windows-latest` |
| `.dmg` | macOS | `macos-latest` |

Você **não** gera todos no mesmo PC. O workflow [`.github/workflows/release.yml`](../.github/workflows/release.yml) usa **3 máquinas virtuais** do GitHub e junta os arquivos na **Release**.

> **Não commite** os `.AppImage` / `.exe` / `.dmg` no repositório Git.  
> Anexe-os na **GitHub Release** (ou baixe os artifacts da Action).

---

## Pré-requisitos

1. Código no **GitHub** (repo público ou privado com Actions habilitado)
2. Permissão para criar tags e releases
3. Versão alinhada nos manifests (opcional, mas recomendado):
   - `package.json` → `"version"`
   - `src-tauri/Cargo.toml` → `version`
   - `src-tauri/tauri.conf.json` → `"version"`

---

## Passo a passo

### 1. Commit e push do código

```bash
cd /home/joao/Projects/testeAppKanban
git status
git add -A
git commit -m "Prepare release v0.1.0"
git push origin main   # ou sua branch padrão
```

### 2. Criar e enviar a tag

```bash
git tag v0.1.0
git push origin v0.1.0
```

O push da tag `v*` dispara o workflow **Release**.

### 3. Acompanhar a Action

1. Abra o repo no GitHub  
2. Aba **Actions**  
3. Workflow **Release** → jobs `build-linux`, `build-windows`, `build-macos`  
4. Espere todos ficarem verdes (pode levar 10–30 min)

### 4. Publicar a Release

Em tags, o job final cria um **draft release** com os instaladores anexados.

1. Aba **Releases**  
2. Abra o draft `v0.1.0`  
3. Revise notas e arquivos  
4. Clique **Publish release**

Pronto: quem acessar a release baixa AppImage, deb, rpm, exe e dmg.

---

## Sem tag (só gerar e baixar)

1. GitHub → **Actions** → **Release** → **Run workflow**  
2. Ao terminar, em cada job: **Artifacts** (`kanban-linux`, `kanban-windows`, `kanban-macos`)  
3. Baixe o zip e use os instaladores localmente  

*Nesse modo não cria Release automática* (só em tags `v*`).

---

## O que o usuário final baixa

| Arquivo | Como usar |
|---------|-----------|
| `Kanban_*_amd64.AppImage` | `chmod +x … && ./…` |
| `Kanban_*_amd64.deb` | `sudo dpkg -i …` |
| `Kanban-*-1.x86_64.rpm` | `sudo rpm -i …` / `dnf install …` |
| `Kanban_*_x64-setup.exe` | Instalador Windows (NSIS) |
| `Kanban_*_x64_en-US.msi` | MSI Windows |
| `Kanban_*.dmg` | Abrir no macOS, arrastar para Applications |

Nomes exatos podem variar um pouco conforme a versão do Tauri.

---

## Build local (parcial)

Só o que o **seu** SO consegue:

```bash
# Linux (AppImage + deb + rpm) — use o script com NO_STRIP
npm run tauri:build:linux

# Windows (no Windows)
npm run tauri build -- --bundles nsis,msi

# macOS (no Mac)
npm run tauri build -- --bundles dmg
```

Para mandar **todos** juntos no GitHub, use a Action (recomendado).

---

## Checklist de release

- [ ] Versão atualizada em `package.json`, `Cargo.toml`, `tauri.conf.json`
- [ ] `CHANGELOG.md` atualizado
- [ ] `npm run check` ok localmente
- [ ] Push na branch principal
- [ ] `git tag vX.Y.Z && git push origin vX.Y.Z`
- [ ] Actions verde nos 3 SO
- [ ] Draft release revisado e publicado
- [ ] (Opcional) Assinatura Windows / notarização macOS

---

## Limitações honestas

| Tema | Situação |
|------|----------|
| Assinatura Windows | Sem certificado → SmartScreen avisa |
| Notarização macOS | Sem Apple Developer → Gatekeeper avisa |
| AppImage no CI | Pode falhar em runners novos; workflow usa `NO_STRIP` + `APPIMAGE_EXTRACT_AND_RUN` |
| Tamanho | AppImage Linux costuma ser bem maior que deb/rpm (traz libs) |

---

## Workflow de referência

Arquivo: [`.github/workflows/release.yml`](../.github/workflows/release.yml)

Relacionados: [building.md](./building.md) · [distribution.md](./distribution.md) · [README](../README.md)
