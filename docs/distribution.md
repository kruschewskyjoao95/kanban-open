# Distribuição

Como publicar o Kanban para usuários finais.

## Estratégias

| Estratégia | Esforço | Ideal para |
|------------|---------|------------|
| Download direto (GitHub Releases) | Baixo | Open source / indie |
| Lojas (MS Store, Mac App Store, Flathub…) | Alto | Alcance / compliance |
| Intranet / MSI corporativo | Médio | Empresas |

Para o estágio atual do projeto, **GitHub Releases + AppImage / NSIS / DMG** é o caminho natural.

## Fluxo recomendado

```
1. Desenvolver em main / PRs
2. Atualizar versão (tauri.conf, Cargo.toml, package.json)
3. Tag: git tag v0.1.0 && git push --tags
4. CI gera artefatos Linux + Windows + macOS
5. Release no GitHub com os instaladores anexados
6. (Opcional) Assinar / notarizar
```

## CI multiplataforma (GitHub Actions)

Workflow: [`.github/workflows/release.yml`](../.github/workflows/release.yml)

| Runner | Artefatos |
|--------|-----------|
| `ubuntu-22.04` | AppImage, deb, rpm (`NO_STRIP` para AppImage) |
| `windows-latest` | NSIS (`.exe`), MSI |
| `macos-latest` | DMG |

Guia passo a passo: **[github-release.md](./github-release.md)**.

### Disparo

```bash
git tag v0.1.0
git push origin v0.1.0
```

Ou **Actions → Release → Run workflow** (só artifacts, sem criar release).

### Action oficial Tauri

Alternativa mais “turnkey”:

- [tauri-apps/tauri-action](https://github.com/tauri-apps/tauri-action)

Cria draft release e anexa bundles automaticamente.

## O que publicar para o usuário

| SO | Arquivo sugerido na Release | Instruções ao usuário |
|----|----------------------------|------------------------|
| Linux | `Kanban_x.y.z_amd64.AppImage` | `chmod +x` e executar |
| Linux (opcional) | `.deb` | `sudo dpkg -i …` |
| Windows | instalador NSIS `.exe` | Next → Next → Finish |
| macOS | `.dmg` | Arrastar para Applications |

Inclua no corpo da Release:

- Changelog curto
- Requisitos (ex.: glibc recente no Linux; WebView2 no Windows)
- Aviso se o build **não** está assinado

## Code signing

### Por que assinar?

| SO | Sem assinatura | Com assinatura |
|----|----------------|----------------|
| Windows | SmartScreen “app desconhecido” | Menos atrito |
| macOS | Gatekeeper bloqueia / avisa | + notarization = ok |
| Linux | Em geral ok | Opcional (pacotes) |

### Windows

1. Certificado Authenticode (comercial ou EV)
2. Configurar variáveis / `tauri.conf` conforme [docs Tauri Windows signing](https://v2.tauri.app/distribute/sign/windows/)
3. CI com secrets do certificado

### macOS

1. Conta [Apple Developer](https://developer.apple.com/) (~US$99/ano)
2. Certificate “Developer ID Application”
3. Code sign + **notarize** + staple  
   Guia: [Tauri macOS signing](https://v2.tauri.app/distribute/sign/macos/)

Sem isso o DMG ainda pode ser distribuído, com instruções manuais de abertura.

### Linux

- AppImage: assinatura opcional (GPG)
- Flathub / Snap: processos próprios de review

## Auto-update (futuro)

Tauri tem plugin de updater. Quando implementar:

1. Servir `latest.json` + artefatos assinados
2. Configurar `plugins.updater` no `tauri.conf.json`
3. Chave pública no app; privada só no CI

Não está no MVP. Ver [Roadmap](./roadmap.md).

## Lojas (referência)

| Loja | Formato / notas |
|------|-----------------|
| Microsoft Store | MSIX / empacotamento específico |
| Mac App Store | Sandbox + certificados de distribuição |
| Flathub | Flatpak (manifest à parte) |
| Snap Store | snapcraft.yaml |

Cada loja tem requisitos extras (sandbox, permissões, review). Só faça quando o produto estiver estável.

## Checklist de release

- [ ] Versão bump em todos os manifests
- [ ] Changelog escrito
- [ ] `npm run check` + smoke test manual
- [ ] Tag `vX.Y.Z` pushada
- [ ] CI verde nos 3 OS
- [ ] Artefatos baixados e smoke-tested
- [ ] Release publicada (não só draft)
- [ ] (Se aplicável) signing / notarization OK

## Privacidade e dados

O app é **local-first**:

- Dados ficam no dispositivo do usuário
- Não há telemetria no MVP
- Backup = copiar `kanban.db` (ver [Configuração](./configuration.md))

Comunique isso na página de download se for público.
