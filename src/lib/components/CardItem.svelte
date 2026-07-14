<script lang="ts">
  import type { Card } from "$lib/types";
  import { clearDrag, setCardDrag } from "$lib/dnd";
  import LabelChips from "./LabelChips.svelte";

  interface Props {
    card: Card;
    onOpen: (card: Card) => void;
    draggable?: boolean;
  }

  let { card, onOpen, draggable = true }: Props = $props();

  let suppressClick = false;

  function onDragStart(e: DragEvent) {
    if (!draggable) {
      e.preventDefault();
      return;
    }
    const el = e.currentTarget as HTMLElement;
    setCardDrag(e, { cardId: card.id, fromListId: card.list_id }, el);
    el.classList.add("dragging");
    suppressClick = true;
  }

  function onDragEnd(e: DragEvent) {
    clearDrag();
    if (e.currentTarget instanceof HTMLElement) {
      e.currentTarget.classList.remove("dragging");
    }
    // avoid opening modal after a drag
    setTimeout(() => {
      suppressClick = false;
    }, 50);
  }

  function onClick() {
    if (suppressClick) return;
    onOpen(card);
  }
</script>

<div
  class="card"
  class:no-drag={!draggable}
  draggable={draggable}
  role="button"
  tabindex="0"
  ondragstart={onDragStart}
  ondragend={onDragEnd}
  onclick={onClick}
  onkeydown={(e) => {
    if (e.key === "Enter" || e.key === " ") {
      e.preventDefault();
      onOpen(card);
    }
  }}
>
  {#if card.labels?.length}
    <LabelChips labels={card.labels} showName={false} />
  {/if}
  <p class="title">{card.title}</p>
  {#if card.description}
    <p class="desc">{card.description}</p>
  {/if}
  {#if card.checklist?.length}
    {@const done = card.checklist.filter((i) => i.done).length}
    {@const total = card.checklist.length}
    <p class="check-progress" title="Checklist">
      <span class="box" class:complete={done === total}>✓</span>
      {done}/{total}
    </p>
  {/if}
</div>

<style>
  .card {
    background: var(--bg-card);
    border: 1px solid transparent;
    border-radius: var(--radius-sm);
    padding: 0.4rem 0.5rem;
    box-shadow: 0 1px 2px rgba(0, 0, 0, 0.22);
    cursor: grab;
    transition:
      background 0.12s,
      border-color 0.12s,
      opacity 0.12s;
    user-select: none;
    display: flex;
    flex-direction: column;
    gap: 0.22rem;
    outline: none;
  }

  .card:hover {
    background: var(--bg-card-hover);
    border-color: var(--border);
  }

  .card:focus,
  .card:focus-visible {
    outline: none;
    box-shadow: none;
    border-color: var(--border);
  }

  .card:active {
    cursor: grabbing;
  }

  .card.no-drag {
    cursor: pointer;
  }

  .card.no-drag:active {
    cursor: pointer;
  }

  /* Dimmed placeholder — keep size stable so layout doesn't jump mid-drag */
  .card:global(.dragging) {
    opacity: 0.3;
    border: 1px dashed var(--border);
    background: rgba(39, 52, 73, 0.35);
    box-shadow: none;
  }

  .title {
    margin: 0;
    font-weight: 500;
    font-size: 12.5px;
    word-break: break-word;
  }

  .desc {
    margin: 0;
    font-size: 11px;
    color: var(--text-muted);
    display: -webkit-box;
    -webkit-line-clamp: 2;
    line-clamp: 2;
    -webkit-box-orient: vertical;
    overflow: hidden;
  }

  .check-progress {
    margin: 0;
    display: inline-flex;
    align-items: center;
    gap: 0.3rem;
    font-size: 11px;
    color: var(--text-muted);
    font-variant-numeric: tabular-nums;
  }

  .box {
    width: 14px;
    height: 14px;
    border-radius: 3px;
    border: 1px solid var(--border);
    display: inline-flex;
    align-items: center;
    justify-content: center;
    font-size: 9px;
    font-weight: 700;
    color: transparent;
  }

  .box.complete {
    background: var(--success);
    border-color: var(--success);
    color: white;
  }
</style>
