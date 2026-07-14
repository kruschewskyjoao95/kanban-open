<script lang="ts">
  import { api, positionBetween } from "$lib/api";
  import type { BoardFull, Card, ChecklistItem, ListWithCards } from "$lib/types";
  import {
    applyAddCard,
    applyAddChecklistItem,
    applyAddLabel,
    applyAddList,
    applyDeleteCard,
    applyDeleteChecklistItem,
    applyDeleteLabel,
    applyDeleteList,
    applyMoveCard,
    applyRenameBoard,
    applyRenameList,
    applyReorderList,
    applySetCardLabels,
    applyUpdateCard,
    applyUpdateChecklistItem,
    applyUpdateLabel,
    cloneBoard,
  } from "$lib/boardMutations";
  import { focusOnMount } from "$lib/actions";
  import { allowDrop, clearDrag, getListDrag, isListDrag } from "$lib/dnd";
  import { countMatchingCards } from "$lib/search";
  import { errMsg, toasts } from "$lib/toast.svelte";
  import ListColumn from "./ListColumn.svelte";
  import CardModal from "./CardModal.svelte";
  import LabelsPanel from "./LabelsPanel.svelte";

  interface Props {
    boardId: string;
    onBack: () => void;
  }

  let { boardId, onBack }: Props = $props();

  let data = $state<BoardFull | null>(null);
  let loading = $state(true);
  let error = $state("");
  let selectedCard = $state<Card | null>(null);
  let addingList = $state(false);
  let newListTitle = $state("");
  let renaming = $state(false);
  let nameDraft = $state("");
  let listDropIndex = $state<number | null>(null);
  let promptAddListId = $state<string | null>(null);
  let showHelp = $state(false);
  let showLabels = $state(false);
  let searchQuery = $state("");
  let searchInputEl = $state<HTMLInputElement | null>(null);

  const matchCount = $derived(
    data ? countMatchingCards(data.lists, searchQuery) : 0,
  );
  const totalCards = $derived(
    data ? data.lists.reduce((n, l) => n + l.cards.length, 0) : 0,
  );
  const isFiltering = $derived(searchQuery.trim().length > 0);

  async function load(opts?: { quiet?: boolean }) {
    if (!opts?.quiet) loading = true;
    error = "";
    try {
      data = await api.getBoard(boardId);
      nameDraft = data.board.name;
    } catch (e) {
      error = errMsg(e);
      if (!opts?.quiet) toasts.error(error);
    } finally {
      loading = false;
    }
  }

  $effect(() => {
    void boardId;
    load();
  });

  function isTypingTarget(el: EventTarget | null): boolean {
    if (!(el instanceof HTMLElement)) return false;
    const tag = el.tagName;
    return tag === "INPUT" || tag === "TEXTAREA" || el.isContentEditable;
  }

  function focusSearch() {
    searchInputEl?.focus();
    searchInputEl?.select();
  }

  function onKeydown(e: KeyboardEvent) {
    // Allow Escape to clear search even while focused in the search box
    if (e.key === "Escape" && isTypingTarget(e.target) && e.target === searchInputEl) {
      if (searchQuery) {
        e.preventDefault();
        searchQuery = "";
      } else {
        searchInputEl?.blur();
      }
      return;
    }

    if (isTypingTarget(e.target)) return;

    if (e.key === "Escape") {
      if (showLabels) {
        showLabels = false;
        return;
      }
      if (showHelp) {
        showHelp = false;
        return;
      }
      if (selectedCard) {
        selectedCard = null;
        return;
      }
      if (addingList) {
        addingList = false;
        newListTitle = "";
        return;
      }
      if (searchQuery) {
        searchQuery = "";
      }
      return;
    }

    // Help: "?" (Shift+/ on many layouts)
    if (e.key === "?" || (e.shiftKey && e.key === "/")) {
      e.preventDefault();
      showHelp = !showHelp;
      return;
    }

    // Focus search: "/" without Shift
    if (e.key === "/" && !e.shiftKey && !e.ctrlKey && !e.metaKey) {
      e.preventDefault();
      focusSearch();
      return;
    }

    if (e.key === "n" || e.key === "N") {
      e.preventDefault();
      if (!data?.lists.length) {
        toasts.info("Crie uma lista primeiro");
        return;
      }
      promptAddListId = data.lists[0].id;
      return;
    }

    if (e.key === "l" || e.key === "L") {
      e.preventDefault();
      addingList = true;
      return;
    }

    if (e.key === "e" || e.key === "E") {
      e.preventDefault();
      showLabels = !showLabels;
    }
  }

  async function addList() {
    const t = newListTitle.trim();
    if (!t || !data) return;
    const snapshot = cloneBoard(data);
    try {
      const list = await api.createList(data.board.id, t);
      const full: ListWithCards = { ...list, cards: [] };
      data = applyAddList(data, full);
      newListTitle = "";
      addingList = false;
      toasts.success("Lista criada");
    } catch (e) {
      data = snapshot;
      toasts.error(errMsg(e));
    }
  }

  async function renameBoard() {
    if (!data) return;
    const n = nameDraft.trim();
    if (!n || n === data.board.name) {
      renaming = false;
      nameDraft = data.board.name;
      return;
    }
    const snapshot = cloneBoard(data);
    data = applyRenameBoard(data, n);
    renaming = false;
    try {
      await api.renameBoard(data.board.id, n);
      toasts.success("Board renomeado");
    } catch (e) {
      data = snapshot;
      nameDraft = snapshot.board.name;
      toasts.error(errMsg(e));
    }
  }

  async function handleMoveCard(cardId: string, listId: string, position: number) {
    if (!data) return;
    const snapshot = cloneBoard(data);
    const optimistic = applyMoveCard(data, cardId, listId, position);
    if (optimistic) data = optimistic;
    try {
      await api.moveCard(cardId, listId, position);
    } catch (e) {
      data = snapshot;
      toasts.error(errMsg(e));
    }
  }

  async function handleAddCard(listId: string, title: string) {
    if (!data) return;
    const snapshot = cloneBoard(data);
    try {
      const card = await api.createCard(listId, title);
      data = applyAddCard(data, {
        ...card,
        labels: card.labels ?? [],
        checklist: card.checklist ?? [],
      });
      toasts.success("Tarefa adicionada");
    } catch (e) {
      data = snapshot;
      toasts.error(errMsg(e));
      throw e;
    }
  }

  async function handleRenameList(listId: string, title: string) {
    if (!data) return;
    const snapshot = cloneBoard(data);
    data = applyRenameList(data, listId, title);
    try {
      await api.renameList(listId, title);
      toasts.success("Lista renomeada");
    } catch (e) {
      data = snapshot;
      toasts.error(errMsg(e));
    }
  }

  async function handleDeleteList(listId: string) {
    if (!data) return;
    const snapshot = cloneBoard(data);
    data = applyDeleteList(data, listId);
    try {
      await api.deleteList(listId);
      toasts.success("Lista excluída");
    } catch (e) {
      data = snapshot;
      toasts.error(errMsg(e));
    }
  }

  async function handleReorderList(listId: string, position: number) {
    if (!data) return;
    const snapshot = cloneBoard(data);
    data = applyReorderList(data, listId, position);
    try {
      await api.reorderList(listId, position);
    } catch (e) {
      data = snapshot;
      toasts.error(errMsg(e));
    }
  }

  async function saveCard(title: string, description: string) {
    if (!selectedCard || !data) return;
    const snapshot = cloneBoard(data);
    try {
      const updated = await api.updateCard(selectedCard.id, title, description);
      data = applyUpdateCard(data, updated);
      selectedCard = updated;
      toasts.success("Card salvo");
    } catch (e) {
      data = snapshot;
      toasts.error(errMsg(e));
      throw e;
    }
  }

  async function handleSetCardLabels(labelIds: string[]) {
    if (!selectedCard || !data) return;
    const snapshot = cloneBoard(data);
    try {
      const updated = await api.setCardLabels(selectedCard.id, labelIds);
      data = applySetCardLabels(data, updated.id, updated.labels);
      selectedCard = updated;
    } catch (e) {
      data = snapshot;
      toasts.error(errMsg(e));
      throw e;
    }
  }

  async function deleteCard() {
    if (!selectedCard || !data) return;
    const id = selectedCard.id;
    const snapshot = cloneBoard(data);
    data = applyDeleteCard(data, id);
    selectedCard = null;
    try {
      await api.deleteCard(id);
      toasts.success("Card excluído");
    } catch (e) {
      data = snapshot;
      toasts.error(errMsg(e));
    }
  }

  async function handleCreateLabel(name: string, color: string) {
    if (!data) return;
    const snapshot = cloneBoard(data);
    try {
      const label = await api.createLabel(data.board.id, name, color);
      data = applyAddLabel(data, label);
      toasts.success("Etiqueta criada");
    } catch (e) {
      data = snapshot;
      toasts.error(errMsg(e));
      throw e;
    }
  }

  async function handleUpdateLabel(id: string, name: string, color: string) {
    if (!data) return;
    const snapshot = cloneBoard(data);
    try {
      const label = await api.updateLabel(id, name, color);
      data = applyUpdateLabel(data, label);
      if (selectedCard) {
        selectedCard = {
          ...selectedCard,
          labels: selectedCard.labels.map((l) => (l.id === label.id ? label : l)),
        };
      }
      toasts.success("Etiqueta atualizada");
    } catch (e) {
      data = snapshot;
      toasts.error(errMsg(e));
      throw e;
    }
  }

  async function handleDeleteLabel(id: string) {
    if (!data) return;
    const snapshot = cloneBoard(data);
    data = applyDeleteLabel(data, id);
    if (selectedCard) {
      selectedCard = {
        ...selectedCard,
        labels: selectedCard.labels.filter((l) => l.id !== id),
      };
    }
    try {
      await api.deleteLabel(id);
      toasts.success("Etiqueta excluída");
    } catch (e) {
      data = snapshot;
      toasts.error(errMsg(e));
      throw e;
    }
  }

  function syncSelectedCard(cardId: string) {
    if (!data || !selectedCard || selectedCard.id !== cardId) return;
    for (const list of data.lists) {
      const found = list.cards.find((c) => c.id === cardId);
      if (found) {
        selectedCard = found;
        break;
      }
    }
  }

  async function handleAddChecklistItem(title: string) {
    if (!selectedCard || !data) return;
    const snapshot = cloneBoard(data);
    try {
      const item = await api.createChecklistItem(selectedCard.id, title);
      data = applyAddChecklistItem(data, item);
      syncSelectedCard(selectedCard.id);
    } catch (e) {
      data = snapshot;
      syncSelectedCard(selectedCard.id);
      toasts.error(errMsg(e));
      throw e;
    }
  }

  async function handleToggleChecklistItem(item: ChecklistItem, done: boolean) {
    if (!data) return;
    const snapshot = cloneBoard(data);
    const optimistic = { ...item, done };
    data = applyUpdateChecklistItem(data, optimistic);
    syncSelectedCard(item.card_id);
    try {
      const updated = await api.updateChecklistItem(item.id, { done });
      data = applyUpdateChecklistItem(data, updated);
      syncSelectedCard(item.card_id);
    } catch (e) {
      data = snapshot;
      syncSelectedCard(item.card_id);
      toasts.error(errMsg(e));
      throw e;
    }
  }

  async function handleRenameChecklistItem(item: ChecklistItem, title: string) {
    if (!data) return;
    const snapshot = cloneBoard(data);
    const optimistic = { ...item, title };
    data = applyUpdateChecklistItem(data, optimistic);
    syncSelectedCard(item.card_id);
    try {
      const updated = await api.updateChecklistItem(item.id, { title });
      data = applyUpdateChecklistItem(data, updated);
      syncSelectedCard(item.card_id);
    } catch (e) {
      data = snapshot;
      syncSelectedCard(item.card_id);
      toasts.error(errMsg(e));
      throw e;
    }
  }

  async function handleDeleteChecklistItem(item: ChecklistItem) {
    if (!data) return;
    const snapshot = cloneBoard(data);
    data = applyDeleteChecklistItem(data, item.card_id, item.id);
    syncSelectedCard(item.card_id);
    try {
      await api.deleteChecklistItem(item.id);
    } catch (e) {
      data = snapshot;
      syncSelectedCard(item.card_id);
      toasts.error(errMsg(e));
      throw e;
    }
  }

  function onBoardDragOver(e: DragEvent) {
    if (!isListDrag() || !data) return;
    allowDrop(e);
    const el = (e.target as HTMLElement).closest("[data-list-index]") as HTMLElement | null;
    if (el) {
      const idx = Number(el.dataset.listIndex);
      const rect = el.getBoundingClientRect();
      const mid = rect.left + rect.width / 2;
      listDropIndex = e.clientX < mid ? idx : idx + 1;
    } else {
      listDropIndex = data.lists.length;
    }
  }

  function onBoardDragLeave(e: DragEvent) {
    const related = e.relatedTarget as Node | null;
    if (!related || !(e.currentTarget as HTMLElement).contains(related)) {
      listDropIndex = null;
    }
  }

  async function onBoardDrop(e: DragEvent) {
    e.preventDefault();
    if (!data) return;
    const drag = getListDrag();
    const index = listDropIndex ?? data.lists.length;
    listDropIndex = null;
    if (!drag) return;
    clearDrag();

    const others = data.lists.filter((l) => l.id !== drag.listId);
    const before = others[index - 1]?.position;
    const after = others[index]?.position;
    const position = positionBetween(before, after);
    await handleReorderList(drag.listId, position);
  }
</script>

<svelte:window onkeydown={onKeydown} />

<div class="board-page">
  <header class="topbar">
    <button class="btn btn-ghost" type="button" onclick={onBack}>← Boards</button>

    {#if data}
      {#if renaming}
        <input
          class="board-name-input"
          bind:value={nameDraft}
          onblur={renameBoard}
          onkeydown={(e) => {
            if (e.key === "Enter") renameBoard();
            if (e.key === "Escape") {
              renaming = false;
              nameDraft = data!.board.name;
            }
          }}
          use:focusOnMount
        />
      {:else}
        <button type="button" class="board-name" onclick={() => (renaming = true)}>
          {data.board.name}
        </button>
      {/if}
    {/if}

    <div class="topbar-spacer"></div>
    {#if data}
      <div class="search-wrap">
        <input
          class="search-input"
          type="search"
          placeholder="Buscar cards…  /"
          bind:this={searchInputEl}
          bind:value={searchQuery}
          aria-label="Buscar cards"
        />
        {#if isFiltering}
          <span class="search-meta" title="Resultados da busca">
            {matchCount}/{totalCards}
          </span>
          <button
            type="button"
            class="icon-btn clear-search"
            title="Limpar busca"
            aria-label="Limpar busca"
            onclick={() => {
              searchQuery = "";
              searchInputEl?.focus();
            }}
          >
            ✕
          </button>
        {/if}
      </div>
      <button
        class="btn btn-ghost btn-sm"
        type="button"
        title="Etiquetas (E)"
        onclick={() => (showLabels = true)}
      >
        Etiquetas
      </button>
    {/if}
    <button
      class="btn btn-ghost btn-sm help-btn"
      type="button"
      title="Atalhos (?)"
      onclick={() => (showHelp = !showHelp)}
    >
      ?
    </button>
  </header>

  {#if data && isFiltering}
    <div class="search-banner" role="status">
      {#if matchCount === 0}
        Nenhum card corresponde a “{searchQuery.trim()}”
      {:else}
        {matchCount}
        {matchCount === 1 ? "card encontrado" : "cards encontrados"}
        · arrastar cards fica desativado durante a busca
      {/if}
    </div>
  {/if}

  {#if loading && !data}
    <p class="status">Carregando…</p>
  {:else if error && !data}
    <p class="status error">{error}</p>
  {:else if data}
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <div
      class="board-scroll"
      ondragover={onBoardDragOver}
      ondragleave={onBoardDragLeave}
      ondrop={onBoardDrop}
    >
      {#each data.lists as list, i (list.id)}
        {#if listDropIndex === i}
          <div class="list-drop-indicator"></div>
        {/if}
        <ListColumn
          {list}
          listIndex={i}
          searchQuery={searchQuery}
          promptAdd={promptAddListId === list.id}
          onOpenCard={(c) => (selectedCard = c)}
          onAddCard={handleAddCard}
          onRenameList={handleRenameList}
          onDeleteList={handleDeleteList}
          onMoveCard={handleMoveCard}
          onPromptAddConsumed={() => (promptAddListId = null)}
        />
      {/each}
      {#if listDropIndex === data.lists.length}
        <div class="list-drop-indicator"></div>
      {/if}

      <div class="add-list">
        {#if addingList}
          <input
            placeholder="Nome da lista…"
            bind:value={newListTitle}
            onkeydown={(e) => {
              if (e.key === "Enter") addList();
              if (e.key === "Escape") {
                addingList = false;
                newListTitle = "";
              }
            }}
            use:focusOnMount
          />
          <div class="add-list-actions">
            <button class="btn btn-primary btn-sm" type="button" onclick={addList}>
              Adicionar lista
            </button>
            <button
              class="btn btn-ghost btn-sm"
              type="button"
              onclick={() => {
                addingList = false;
                newListTitle = "";
              }}
            >
              Cancelar
            </button>
          </div>
        {:else}
          <button class="add-list-btn" type="button" onclick={() => (addingList = true)}>
            + Adicionar lista
          </button>
        {/if}
      </div>
    </div>
  {/if}
</div>

{#if selectedCard && data}
  <CardModal
    card={selectedCard}
    boardLabels={data.labels}
    onSave={saveCard}
    onSetLabels={handleSetCardLabels}
    onAddChecklistItem={handleAddChecklistItem}
    onToggleChecklistItem={handleToggleChecklistItem}
    onRenameChecklistItem={handleRenameChecklistItem}
    onDeleteChecklistItem={handleDeleteChecklistItem}
    onDelete={deleteCard}
    onClose={() => (selectedCard = null)}
  />
{/if}

{#if showLabels && data}
  <LabelsPanel
    labels={data.labels}
    onCreate={handleCreateLabel}
    onUpdate={handleUpdateLabel}
    onDelete={handleDeleteLabel}
    onClose={() => (showLabels = false)}
  />
{/if}

{#if showHelp}
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div
    class="help-backdrop"
    role="presentation"
    onclick={() => (showHelp = false)}
    onkeydown={(e) => e.key === "Escape" && (showHelp = false)}
  >
    <!-- svelte-ignore a11y_click_events_have_key_events -->
    <div
      class="help-panel"
      role="dialog"
      aria-modal="true"
      aria-label="Atalhos de teclado"
      tabindex="-1"
      onclick={(e) => e.stopPropagation()}
      onkeydown={(e) => e.stopPropagation()}
    >
      <header>
        <h2>Atalhos</h2>
        <button class="icon-btn" type="button" onclick={() => (showHelp = false)} aria-label="Fechar"
          >✕</button
        >
      </header>
      <dl>
        <div><dt><kbd>/</kbd></dt><dd>Focar busca de cards</dd></div>
        <div><dt><kbd>n</kbd></dt><dd>Nova tarefa (primeira lista)</dd></div>
        <div><dt><kbd>l</kbd></dt><dd>Nova lista</dd></div>
        <div><dt><kbd>e</kbd></dt><dd>Gerenciar etiquetas</dd></div>
        <div><dt><kbd>?</kbd></dt><dd>Mostrar / ocultar esta ajuda</dd></div>
        <div><dt><kbd>Esc</kbd></dt><dd>Limpar busca / fechar modal</dd></div>
      </dl>
      <p class="help-note">
        Arraste o cabeçalho da lista (⋮⋮) para reordenar colunas. Busca filtra título, descrição e
        nome de etiqueta.
      </p>
    </div>
  </div>
{/if}

<style>
  .board-page {
    height: 100vh;
    min-height: 100vh;
    width: 100%;
    display: flex;
    flex-direction: column;
    background:
      radial-gradient(ellipse at 20% 0%, rgba(59, 130, 246, 0.12), transparent 50%),
      radial-gradient(ellipse at 80% 100%, rgba(34, 197, 94, 0.08), transparent 45%),
      var(--bg);
  }

  .topbar {
    display: flex;
    align-items: center;
    gap: 0.45rem;
    padding: 0.4rem 0.65rem;
    border-bottom: 1px solid var(--border);
    background: rgba(15, 20, 25, 0.75);
    backdrop-filter: blur(8px);
  }

  .topbar-spacer {
    flex: 1;
  }

  .search-wrap {
    display: flex;
    align-items: center;
    gap: 0.2rem;
    max-width: 200px;
    min-width: 120px;
    flex: 1;
  }

  .search-input {
    flex: 1;
    min-width: 0;
    padding: 0.28rem 0.45rem;
    font-size: 12px;
    background: rgba(0, 0, 0, 0.25);
  }

  .search-meta {
    font-size: 10px;
    color: var(--text-muted);
    white-space: nowrap;
    font-variant-numeric: tabular-nums;
  }

  .clear-search {
    flex-shrink: 0;
  }

  .search-banner {
    padding: 0.28rem 0.65rem;
    font-size: 11px;
    color: var(--text-muted);
    background: rgba(59, 130, 246, 0.08);
    border-bottom: 1px solid var(--border);
  }

  .help-btn {
    width: 24px;
    height: 24px;
    padding: 0;
    border-radius: 999px;
    border: 1px solid var(--border);
    font-weight: 700;
    flex-shrink: 0;
    font-size: 12px;
  }

  .board-name {
    margin: 0;
    font-size: 14px;
    font-weight: 650;
    cursor: pointer;
    background: none;
    border: none;
    color: inherit;
    padding: 0;
    text-align: left;
  }

  .board-name:hover {
    color: white;
  }

  .board-name-input {
    max-width: 280px;
    font-size: 14px;
    font-weight: 650;
    padding: 0.28rem 0.45rem;
  }

  .board-scroll {
    flex: 1;
    min-height: 0;
    display: flex;
    align-items: flex-start;
    gap: 0.5rem;
    padding: 0.55rem;
    overflow-x: auto;
    overflow-y: hidden;
  }

  .list-drop-indicator {
    width: 4px;
    min-width: 4px;
    align-self: stretch;
    min-height: 120px;
    background: var(--accent);
    border-radius: 2px;
    margin: 0 -0.25rem;
  }

  .add-list {
    min-width: var(--list-width, 244px);
    width: var(--list-width, 244px);
  }

  .add-list-btn {
    width: 100%;
    text-align: left;
    padding: 0.5rem 0.65rem;
    background: rgba(255, 255, 255, 0.06);
    border-radius: var(--radius);
    color: var(--text);
    font-weight: 500;
    font-size: 12px;
  }

  .add-list-btn:hover {
    background: rgba(255, 255, 255, 0.1);
  }

  .add-list input {
    margin-bottom: 0.4rem;
  }

  .add-list-actions {
    display: flex;
    gap: 0.35rem;
  }

  .status {
    padding: 2rem;
    color: var(--text-muted);
  }

  .error {
    color: var(--danger);
  }

  .help-backdrop {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.5);
    z-index: 150;
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 1rem;
  }

  .help-panel {
    background: var(--bg-elevated);
    border: 1px solid var(--border);
    border-radius: var(--radius);
    padding: 1rem 1.2rem 1.15rem;
    width: min(360px, 100%);
    box-shadow: var(--shadow);
  }

  .help-panel header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: 0.75rem;
  }

  .help-panel h2 {
    margin: 0;
    font-size: 1.05rem;
  }

  .help-panel dl {
    margin: 0;
    display: flex;
    flex-direction: column;
    gap: 0.55rem;
  }

  .help-panel dl div {
    display: flex;
    align-items: center;
    gap: 0.75rem;
  }

  .help-panel dt {
    margin: 0;
    min-width: 3rem;
  }

  .help-panel dd {
    margin: 0;
    color: var(--text-muted);
    font-size: 13px;
  }

  kbd {
    display: inline-block;
    padding: 0.15rem 0.45rem;
    border-radius: 4px;
    border: 1px solid var(--border);
    background: var(--bg);
    font-size: 12px;
    font-family: inherit;
    font-weight: 600;
  }

  .help-note {
    margin: 0.9rem 0 0;
    font-size: 12px;
    color: var(--text-muted);
  }
</style>
