<script lang="ts">
  import type { Card, ListWithCards } from "$lib/types";
  import {
    allowDrop,
    clearDrag,
    getCardDrag,
    isCardDrag,
    onCardDragChange,
    setListDrag,
  } from "$lib/dnd";
  import { positionBetween } from "$lib/api";
  import { focusOnMount } from "$lib/actions";
  import { filterListCards } from "$lib/search";
  import { toasts, errMsg } from "$lib/toast.svelte";
  import CardItem from "./CardItem.svelte";
  import { onMount } from "svelte";

  interface Props {
    list: ListWithCards;
    listIndex: number;
    promptAdd?: boolean;
    searchQuery?: string;
    onOpenCard: (card: Card) => void;
    onAddCard: (listId: string, title: string) => Promise<void>;
    onRenameList: (listId: string, title: string) => Promise<void>;
    onDeleteList: (listId: string) => Promise<void>;
    onMoveCard: (cardId: string, listId: string, position: number) => Promise<void>;
    onPromptAddConsumed?: () => void;
  }

  let {
    list,
    listIndex,
    promptAdd = false,
    searchQuery = "",
    onOpenCard,
    onAddCard,
    onRenameList,
    onDeleteList,
    onMoveCard,
    onPromptAddConsumed,
  }: Props = $props();

  const filtering = $derived(searchQuery.trim().length > 0);
  const visible = $derived(filterListCards(list, searchQuery));

  let newTitle = $state("");
  let editingTitle = $state(false);
  let titleDraft = $state("");
  let dropIndex = $state(0);
  /** Depth counter avoids flicker when moving between children. */
  let enterDepth = $state(0);
  let cardDragActive = $state(false);
  let busy = $state(false);
  let titleInputEl = $state<HTMLInputElement | null>(null);
  let cardsEl = $state<HTMLDivElement | null>(null);

  const isDropTarget = $derived(enterDepth > 0 && cardDragActive);

  onMount(() => onCardDragChange((d) => {
    cardDragActive = d;
    if (!d) {
      enterDepth = 0;
      dropIndex = 0;
    }
  }));

  $effect(() => {
    titleDraft = list.title;
  });

  $effect(() => {
    if (promptAdd) {
      onPromptAddConsumed?.();
      queueMicrotask(() => titleInputEl?.focus());
    }
  });

  async function submitCard() {
    const t = newTitle.trim();
    if (!t) {
      toasts.info("Digite um título para a tarefa");
      titleInputEl?.focus();
      return;
    }
    if (busy) return;
    busy = true;
    try {
      await onAddCard(list.id, t);
      newTitle = "";
      titleInputEl?.focus();
    } catch (e) {
      toasts.error(errMsg(e));
    } finally {
      busy = false;
    }
  }

  async function saveTitle() {
    const t = titleDraft.trim();
    if (!t || t === list.title) {
      editingTitle = false;
      titleDraft = list.title;
      return;
    }
    try {
      await onRenameList(list.id, t);
      editingTitle = false;
    } catch (e) {
      toasts.error(errMsg(e));
    }
  }

  function indexFromPointer(clientY: number): number {
    if (!cardsEl) return visible.cards.length;

    const cardEls = [...cardsEl.querySelectorAll<HTMLElement>("[data-card-index]")];
    if (cardEls.length === 0) return 0;

    for (const el of cardEls) {
      const r = el.getBoundingClientRect();
      const mid = r.top + r.height / 2;
      if (clientY < mid) return Number(el.dataset.cardIndex);
    }
    return cardEls.length;
  }

  function onDragEnterList(e: DragEvent) {
    if (filtering || !isCardDrag()) return;
    allowDrop(e);
    // +1 per enter (bubbles from children) — balanced by leave
    enterDepth += 1;
  }

  function onDragOverList(e: DragEvent) {
    if (filtering || !isCardDrag()) return;
    allowDrop(e);
    e.stopPropagation();
    if (enterDepth === 0) enterDepth = 1;
    dropIndex = indexFromPointer(e.clientY);
  }

  function onDragLeaveList(_e: DragEvent) {
    // Always decrement (pairs with bubbling dragenter from children)
    enterDepth = Math.max(0, enterDepth - 1);
    if (enterDepth === 0) dropIndex = 0;
  }

  async function onDrop(e: DragEvent) {
    if (filtering) return;
    const data = getCardDrag();
    if (!data) return;

    e.preventDefault();
    e.stopPropagation();

    let index = dropIndex;
    // Recompute from pointer at drop time (most accurate)
    index = indexFromPointer(e.clientY);

    enterDepth = 0;
    dropIndex = 0;
    clearDrag();

    const cards = list.cards.filter((c) => c.id !== data.cardId);
    index = Math.max(0, Math.min(index, cards.length));
    const before = cards[index - 1]?.position;
    const after = cards[index]?.position;
    const position = positionBetween(before, after);
    await onMoveCard(data.cardId, list.id, position);
  }

  function onListDragStart(e: DragEvent) {
    const t = e.target as HTMLElement;
    if (t.closest("input, textarea, button, .add-footer")) {
      e.preventDefault();
      return;
    }
    const el = e.currentTarget as HTMLElement;
    setListDrag(e, { listId: list.id }, el);
    el.classList.add("list-dragging");
  }

  function onListDragEnd(e: DragEvent) {
    clearDrag();
    enterDepth = 0;
    dropIndex = 0;
    if (e.currentTarget instanceof HTMLElement) {
      e.currentTarget.classList.remove("list-dragging");
    }
  }
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<section
  class="list"
  class:is-drop-target={isDropTarget}
  class:card-drag-active={cardDragActive}
  aria-label={list.title}
  data-list-index={listIndex}
  data-list-id={list.id}
  ondragenter={onDragEnterList}
  ondragover={onDragOverList}
  ondragleave={onDragLeaveList}
  ondrop={onDrop}
>
  <header
    class="list-header"
    draggable="true"
    ondragstart={onListDragStart}
    ondragend={onListDragEnd}
    title="Arraste para reordenar a lista"
  >
    <span class="grip" aria-hidden="true">⋮⋮</span>
    {#if editingTitle}
      <input
        class="title-input"
        bind:value={titleDraft}
        onblur={saveTitle}
        onkeydown={(e) => {
          if (e.key === "Enter") saveTitle();
          if (e.key === "Escape") {
            editingTitle = false;
            titleDraft = list.title;
          }
        }}
        use:focusOnMount
      />
    {:else}
      <button type="button" class="list-title" onclick={() => (editingTitle = true)}>
        {list.title}
        <span
          class="count"
          title={filtering ? `${visible.cards.length} de ${list.cards.length}` : undefined}
        >
          {#if filtering}
            {visible.cards.length}/{list.cards.length}
          {:else}
            {list.cards.length}
          {/if}
        </span>
      </button>
    {/if}
    <button
      class="icon-btn"
      type="button"
      title="Excluir lista"
      aria-label="Excluir lista"
      onclick={() => {
        if (confirm(`Excluir a lista "${list.title}"?`)) onDeleteList(list.id);
      }}
    >
      🗑
    </button>
  </header>

  <div class="cards" bind:this={cardsEl}>
    {#each visible.cards as card, i (card.id)}
      <!-- Fixed-height line: only color changes (no layout jump) -->
      <div
        class="drop-line"
        class:active={isDropTarget && dropIndex === i}
        aria-hidden="true"
      ></div>
      <div class="card-wrap" data-card-index={i}>
        <CardItem {card} onOpen={onOpenCard} draggable={!filtering} />
      </div>
    {:else}
      {#if filtering}
        <p class="empty-filter">Nenhum card</p>
      {/if}
    {/each}

    <!-- End of list — always present, fixed size -->
    {#if !filtering}
      <div
        class="drop-line end"
        class:active={isDropTarget && dropIndex === visible.cards.length}
        aria-hidden="true"
      ></div>
      <!-- Permanent empty hit area (does not appear/disappear) -->
      <div
        class="drop-pad"
        class:hot={isDropTarget}
        data-card-index={visible.cards.length}
      ></div>
    {/if}
  </div>

  <form
    class="add-footer"
    onsubmit={(e) => {
      e.preventDefault();
      submitCard();
    }}
  >
    <input
      type="text"
      class="add-input"
      placeholder="+ Nova tarefa (Enter)"
      bind:value={newTitle}
      bind:this={titleInputEl}
      disabled={busy}
      autocomplete="off"
    />
    <button class="btn btn-primary btn-sm add-submit" type="submit" disabled={busy}>
      {busy ? "…" : "Add"}
    </button>
  </form>
</section>

<style>
  .list {
    background: var(--bg-list);
    border-radius: var(--radius);
    width: var(--list-width, 200px);
    min-width: var(--list-width, 200px);
    max-height: 100%;
    display: flex;
    flex-direction: column;
    border: 1px solid var(--border);
    overflow: hidden;
    align-self: flex-start;
    transition:
      border-color 0.12s,
      box-shadow 0.12s,
      background 0.12s;
  }

  .list.is-drop-target {
    border-color: var(--accent);
    box-shadow: inset 0 0 0 1px rgba(59, 130, 246, 0.45);
    background: color-mix(in srgb, var(--accent) 8%, var(--bg-list));
  }

  .list-header {
    display: flex;
    align-items: center;
    gap: 0.25rem;
    padding: 0.4rem 0.35rem 0.3rem 0.4rem;
    cursor: grab;
    user-select: none;
    flex-shrink: 0;
  }

  .list-header:active {
    cursor: grabbing;
  }

  .list-header:global(.list-dragging) {
    opacity: 0.3;
  }

  .list:has(:global(.list-dragging)) {
    opacity: 0.45;
    outline: 1px dashed var(--border);
  }

  .grip {
    color: var(--text-muted);
    font-size: 10px;
    letter-spacing: -2px;
    opacity: 0.55;
    padding: 0 0.1rem;
  }

  .list-title {
    margin: 0;
    flex: 1;
    font-size: 12px;
    font-weight: 600;
    cursor: pointer;
    display: flex;
    align-items: center;
    gap: 0.3rem;
    min-width: 0;
    text-align: left;
    padding: 0;
    background: none;
    border: none;
    color: inherit;
  }

  .count {
    font-size: 10px;
    font-weight: 500;
    color: var(--text-muted);
    background: rgba(255, 255, 255, 0.06);
    padding: 0.05rem 0.3rem;
    border-radius: 999px;
  }

  .title-input {
    flex: 1;
    font-weight: 600;
    padding: 0.25rem 0.35rem;
  }

  .cards {
    flex: 1 1 auto;
    min-height: 72px;
    overflow-x: hidden;
    overflow-y: auto;
    padding: 0.25rem 0.35rem 0.15rem;
    display: flex;
    flex-direction: column;
  }

  /*
   * Fixed-height drop markers — only background changes when active.
   * Never change height while dragging (prevents dragleave flicker).
   */
  .drop-line {
    flex-shrink: 0;
    height: 10px;
    margin: 0;
    border-radius: 3px;
    background: transparent;
  }

  .drop-line.active {
    background: rgba(59, 130, 246, 0.7);
    box-shadow: 0 0 0 1px rgba(59, 130, 246, 0.9);
  }

  /* Extra permanent hit area at the bottom of the list (empty or not) */
  .drop-pad {
    flex: 1 1 auto;
    min-height: 48px;
    border-radius: var(--radius-sm);
    border: 1px dashed transparent;
    margin-top: 2px;
  }

  .list.card-drag-active .drop-pad {
    border-color: rgba(148, 163, 184, 0.25);
  }

  .drop-pad.hot {
    border-color: rgba(59, 130, 246, 0.55);
    background: rgba(59, 130, 246, 0.1);
  }

  .card-wrap {
    flex-shrink: 0;
  }

  .add-footer {
    flex-shrink: 0;
    display: flex;
    gap: 0.25rem;
    padding: 0.35rem;
    border-top: 1px solid var(--border);
    background: rgba(0, 0, 0, 0.15);
  }

  .add-input {
    flex: 1;
    min-width: 0;
    font-size: 11px;
  }

  .add-submit {
    flex-shrink: 0;
  }

  .empty-filter {
    margin: 0.25rem 0;
    padding: 0.35rem 0.15rem;
    font-size: 11px;
    color: var(--text-muted);
    text-align: center;
  }
</style>
