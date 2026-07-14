# Guia do usuário

Como usar o aplicativo Kanban no dia a dia (sem código).

## O que é

Kanban é um quadro de tarefas no estilo Trello:

- **Board** — um projeto / quadro
- **Lista** — coluna (ex.: To Do, Doing, Done)
- **Card** — uma tarefa

Tudo fica **no seu computador**. Não precisa de conta nem internet.

## Primeiros passos

1. Abra o app **Kanban**
2. Clique em **+ Novo board**
3. Digite um nome (ex.: “Trabalho”) e confirme
4. O board já vem com três listas: **To Do**, **Doing**, **Done**

## Boards

| Ação | Como |
|------|------|
| Abrir | Clique no card do board na tela inicial |
| Voltar à lista | Botão **← Boards** no topo |
| Renomear | Clique no nome do board no topo e edite; Enter ou clique fora para salvar |
| Excluir | Na tela inicial, ícone de lixeira no board (confirmação) |

## Listas

| Ação | Como |
|------|------|
| Criar | **+ Adicionar lista** à direita das colunas |
| Renomear | Clique no título da lista |
| Excluir | Ícone de lixeira no cabeçalho da lista (apaga os cards dela) |
| Adicionar card | **+ Adicionar card** no rodapé da lista |

## Cards

| Ação | Como |
|------|------|
| Criar | **+ Adicionar card** → título → Enter ou **Adicionar** |
| Abrir / editar | Clique no card |
| Descrição | No modal, campo **Descrição** → **Salvar** |
| Excluir | No modal, **Excluir** (confirmação) |
| Mover / reordenar | Arraste o card para outra lista ou posição na mesma lista |
| Etiquetas no card | Abra o card → seção **Etiquetas** → clique para marcar/desmarcar → **Salvar** |
| Checklist | Abra o card → seção **Checklist** → adicione itens, marque/desmarque, edite o texto |

## Checklist

Cada card pode ter uma lista de subtarefas:

| Ação | Como |
|------|------|
| Adicionar item | Campo “Adicionar item…” + Enter ou botão |
| Marcar feito | Clique no quadrado à esquerda |
| Renomear | Clique no texto do item |
| Remover | ✕ à direita do item |
| Ver progresso | Barra no modal; no quadro aparece `done/total` no card |

Mudanças de checklist **salvam na hora** (não dependem do botão Salvar do card).

## Etiquetas

Cada board tem suas etiquetas coloridas (ex.: Bug, Feature, Urgente).

| Ação | Como |
|------|------|
| Gerenciar | Botão **Etiquetas** na barra do board (ou tecla `e`) |
| Criar | Nome + cor no painel → **Criar** |
| Editar | Clique na etiqueta no painel |
| Excluir | Ícone de lixeira no painel |
| Aplicar no card | Modal do card → lista de etiquetas |

Boards **novos** já vêm com: Bug, Feature, Melhoria, Urgente.  
Boards antigos: crie as etiquetas manualmente no painel.

## Dicas

- **Escape** cancela formulários e fecha o modal
- Feche o app com segurança — os dados já estão salvos no disco a cada ação
- Para backup, copie o arquivo de banco (ver seção abaixo)
- Mensagens de sucesso/erro aparecem no canto inferior direito

## Atalhos de teclado

| Tecla | Onde | Ação |
|-------|------|------|
| `n` | Lista de boards | Novo board |
| `n` | Dentro do board | Novo card na primeira lista |
| `l` | Dentro do board | Nova lista |
| `e` | Dentro do board | Gerenciar etiquetas |
| `/` | Dentro do board | Focar busca de cards |
| `?` | Dentro do board | Ajuda de atalhos |
| `Esc` | Board / modal | Limpar busca / fechar ajuda / modal |

Arraste o **cabeçalho da lista** (ícone ⋮⋮) para reordenar colunas.  
Arraste **cards** entre listas ou para reordenar.

## Busca

Na barra do board, use o campo **Buscar cards…** (ou tecla `/`).

- Filtra por **título**, **descrição** e **nome de etiqueta**
- Cada lista mostra contagem `visíveis/total` enquanto houver filtro
- Durante a busca, **arrastar cards fica desativado** (reordenar com filtro seria ambíguo)
- `Esc` limpa a busca

## Onde estão meus dados?

Arquivo `kanban.db`:

| Sistema | Pasta |
|---------|--------|
| Linux | `~/.local/share/com.testeapp.kanban/` |
| macOS | `~/Library/Application Support/com.testeapp.kanban/` |
| Windows | `%APPDATA%\com.testeapp.kanban\` |

**Backup:** copie `kanban.db` com o app fechado.  
**Restaurar:** substitua o arquivo (app fechado) e abra de novo.

## Limitações atuais

- Sem sincronização entre dispositivos
- Sem anexos, comentários ou etiquetas
- Sem modo claro (só tema escuro)
- Sem conta / colaboração em tempo real

## Instalação do app

Veja [Building](./building.md) (desenvolvedores) ou a página de **Releases** do repositório para baixar instaladores prontos.
