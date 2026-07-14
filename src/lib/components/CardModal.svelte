<script lang="ts">
  import { focusOnMount } from "$lib/actions";
  import type { Card, ChecklistItem, Label } from "$lib/types";
  import LabelChips from "./LabelChips.svelte";

  interface Props {
    card: Card;
    boardLabels: Label[];
    onSave: (title: string, description: string) => Promise<void>;
    onSetLabels: (labelIds: string[]) => Promise<void>;
    onAddChecklistItem: (title: string) => Promise<void>;
    onToggleChecklistItem: (item: ChecklistItem, done: boolean) => Promise<void>;
    onRenameChecklistItem: (item: ChecklistItem, title: string) => Promise<void>;
    onDeleteChecklistItem: (item: ChecklistItem) => Promise<void>;
    onDelete: () => Promise<void>;
    onClose: () => void;
  }

  let {
    card,
    boardLabels,
    onSave,
    onSetLabels,
    onAddChecklistItem,
    onToggleChecklistItem,
    onRenameChecklistItem,
    onDeleteChecklistItem,
    onDelete,
    onClose,
  }: Props = $props();

  let title = $state("");
  let description = $state("");
  let selectedIds = $state<string[]>([]);
  let saving = $state(false);
  let error = $state("");
  let newItemTitle = $state("");
  let addingItem = $state(false);
  let editingItemId = $state<string | null>(null);
  let editItemTitle = $state("");

  $effect(() => {
    title = card.title;
    description = card.description;
    selectedIds = (card.labels ?? []).map((l) => l.id);
  });

  const checklist = $derived(card.checklist ?? []);
  const doneCount = $derived(checklist.filter((i) => i.done).length);
  const totalCount = $derived(checklist.length);
  const progress = $derived(totalCount === 0 ? 0 : Math.round((doneCount / totalCount) * 100));

  const selectedLabels = $derived(
    boardLabels.filter((l) => selectedIds.includes(l.id)),
  );

  function toggleLabel(id: string) {
    if (selectedIds.includes(id)) {
      selectedIds = selectedIds.filter((x) => x !== id);
    } else {
      selectedIds = [...selectedIds, id];
    }
  }

  async function save() {
    if (!title.trim()) {
      error = "Título obrigatório";
      return;
    }
    saving = true;
    error = "";
    try {
      await onSave(title.trim(), description);
      const prev = new Set((card.labels ?? []).map((l) => l.id));
      const next = new Set(selectedIds);
      const same =
        prev.size === next.size && [...prev].every((id) => next.has(id));
      if (!same) {
        await onSetLabels(selectedIds);
      }
      onClose();
    } catch (e) {
      error = String(e);
    } finally {
      saving = false;
    }
  }

  async function remove() {
    if (!confirm("Excluir este card?")) return;
    saving = true;
    try {
      await onDelete();
      onClose();
    } catch (e) {
      error = String(e);
      saving = false;
    }
  }

  async function addItem() {
    const t = newItemTitle.trim();
    if (!t || addingItem) return;
    addingItem = true;
    error = "";
    try {
      await onAddChecklistItem(t);
      newItemTitle = "";
    } catch (e) {
      error = String(e);
    } finally {
      addingItem = false;
    }
  }

  async function toggleItem(item: ChecklistItem) {
    error = "";
    try {
      await onToggleChecklistItem(item, !item.done);
    } catch (e) {
      error = String(e);
    }
  }

  function startEditItem(item: ChecklistItem) {
    editingItemId = item.id;
    editItemTitle = item.title;
  }

  async function commitEditItem(item: ChecklistItem) {
    const t = editItemTitle.trim();
    editingItemId = null;
    if (!t || t === item.title) return;
    error = "";
    try {
      await onRenameChecklistItem(item, t);
    } catch (e) {
      error = String(e);
    }
  }

  async function removeItem(item: ChecklistItem) {
    error = "";
    try {
      await onDeleteChecklistItem(item);
    } catch (e) {
      error = String(e);
    }
  }

  function onKey(e: KeyboardEvent) {
    if (e.key === "Escape") onClose();
  }
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<div class="backdrop" onclick={onClose} onkeydown={onKey} role="presentation">
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div
    class="modal"
    role="dialog"
    aria-modal="true"
    aria-label="Editar card"
    tabindex="-1"
    onclick={(e) => e.stopPropagation()}
    onkeydown={(e) => e.stopPropagation()}
  >
    <header>
      <h2>Card</h2>
      <button class="icon-btn" type="button" onclick={onClose} aria-label="Fechar">✕</button>
    </header>

    {#if selectedLabels.length}
      <LabelChips labels={selectedLabels} size="md" />
    {/if}

    <label>
      <span>Título</span>
      <input bind:value={title} use:focusOnMount />
    </label>

    <label>
      <span>Descrição</span>
      <textarea rows="4" bind:value={description} placeholder="Adicione detalhes…"></textarea>
    </label>

    <div class="checklist-section">
      <div class="check-head">
        <span class="section-label">Checklist</span>
        {#if totalCount > 0}
          <span class="check-meta">{doneCount}/{totalCount} · {progress}%</span>
        {/if}
      </div>
      {#if totalCount > 0}
        <div class="progress" aria-hidden="true">
          <div class="progress-bar" style="width: {progress}%"></div>
        </div>
      {/if}

      <ul class="check-list">
        {#each checklist as item (item.id)}
          <li class:done={item.done}>
            <button
              type="button"
              class="check-box"
              class:on={item.done}
              aria-label={item.done ? "Desmarcar" : "Marcar como feito"}
              onclick={() => toggleItem(item)}
            >
              {#if item.done}✓{/if}
            </button>
            {#if editingItemId === item.id}
              <input
                class="item-input"
                bind:value={editItemTitle}
                onblur={() => commitEditItem(item)}
                onkeydown={(e) => {
                  if (e.key === "Enter") commitEditItem(item);
                  if (e.key === "Escape") editingItemId = null;
                }}
              />
            {:else}
              <button type="button" class="item-title" onclick={() => startEditItem(item)}>
                {item.title}
              </button>
            {/if}
            <button
              type="button"
              class="icon-btn item-del"
              title="Remover item"
              aria-label="Remover item"
              onclick={() => removeItem(item)}
            >
              ✕
            </button>
          </li>
        {/each}
      </ul>

      <div class="add-item">
        <input
          placeholder="Adicionar item…"
          bind:value={newItemTitle}
          onkeydown={(e) => {
            if (e.key === "Enter") {
              e.preventDefault();
              addItem();
            }
          }}
        />
        <button
          class="btn btn-primary btn-sm"
          type="button"
          onclick={addItem}
          disabled={addingItem || !newItemTitle.trim()}
        >
          Adicionar
        </button>
      </div>
    </div>

    <div class="labels-section">
      <span class="section-label">Etiquetas</span>
      {#if boardLabels.length === 0}
        <p class="hint">Nenhuma etiqueta neste board. Crie em “Etiquetas” na barra superior.</p>
      {:else}
        <div class="label-picker">
          {#each boardLabels as label (label.id)}
            <button
              type="button"
              class="label-opt"
              class:on={selectedIds.includes(label.id)}
              style="--c: {label.color}"
              onclick={() => toggleLabel(label.id)}
            >
              <span class="dot"></span>
              {label.name}
              {#if selectedIds.includes(label.id)}
                <span class="check">✓</span>
              {/if}
            </button>
          {/each}
        </div>
      {/if}
    </div>

    {#if error}
      <p class="error">{error}</p>
    {/if}

    <footer>
      <button class="btn btn-danger" type="button" onclick={remove} disabled={saving}>
        Excluir
      </button>
      <div class="spacer"></div>
      <button class="btn btn-ghost" type="button" onclick={onClose} disabled={saving}>
        Cancelar
      </button>
      <button class="btn btn-primary" type="button" onclick={save} disabled={saving}>
        {saving ? "Salvando…" : "Salvar"}
      </button>
    </footer>
  </div>
</div>

<style>
  .backdrop {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.55);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 100;
    padding: 1rem;
  }

  .modal {
    background: var(--bg-elevated);
    border: 1px solid var(--border);
    border-radius: var(--radius);
    width: min(420px, 100%);
    padding: 0.85rem 0.95rem 0.95rem;
    box-shadow: var(--shadow);
    display: flex;
    flex-direction: column;
    gap: 0.6rem;
    max-height: min(88vh, 640px);
    overflow-y: auto;
  }

  header {
    display: flex;
    align-items: center;
    justify-content: space-between;
  }

  h2 {
    margin: 0;
    font-size: 14px;
  }

  label {
    display: flex;
    flex-direction: column;
    gap: 0.35rem;
  }

  label span,
  .section-label {
    font-size: 12px;
    color: var(--text-muted);
    font-weight: 500;
  }

  textarea {
    resize: vertical;
    min-height: 80px;
  }

  .checklist-section,
  .labels-section {
    display: flex;
    flex-direction: column;
    gap: 0.45rem;
  }

  .check-head {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 0.5rem;
  }

  .check-meta {
    font-size: 11px;
    color: var(--text-muted);
    font-variant-numeric: tabular-nums;
  }

  .progress {
    height: 6px;
    border-radius: 999px;
    background: rgba(255, 255, 255, 0.08);
    overflow: hidden;
  }

  .progress-bar {
    height: 100%;
    background: var(--success);
    border-radius: 999px;
    transition: width 0.15s ease;
  }

  .check-list {
    list-style: none;
    margin: 0;
    padding: 0;
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
  }

  .check-list li {
    display: flex;
    align-items: center;
    gap: 0.4rem;
    min-height: 32px;
  }

  .check-box {
    width: 20px;
    height: 20px;
    border-radius: 4px;
    border: 1.5px solid var(--border);
    flex-shrink: 0;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    font-size: 12px;
    font-weight: 700;
    color: white;
    background: transparent;
  }

  .check-box.on {
    background: var(--success);
    border-color: var(--success);
  }

  .item-title {
    flex: 1;
    text-align: left;
    color: inherit;
    padding: 0.25rem 0.2rem;
    border-radius: 4px;
    min-width: 0;
    word-break: break-word;
  }

  .item-title:hover {
    background: rgba(255, 255, 255, 0.05);
  }

  li.done .item-title {
    text-decoration: line-through;
    color: var(--text-muted);
  }

  .item-input {
    flex: 1;
    padding: 0.3rem 0.45rem;
  }

  .item-del {
    opacity: 0.45;
  }

  .check-list li:hover .item-del {
    opacity: 1;
  }

  .add-item {
    display: flex;
    gap: 0.4rem;
  }

  .add-item input {
    flex: 1;
  }

  .hint {
    margin: 0;
    font-size: 12px;
    color: var(--text-muted);
  }

  .label-picker {
    display: flex;
    flex-direction: column;
    gap: 0.35rem;
  }

  .label-opt {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.45rem 0.55rem;
    border-radius: var(--radius-sm);
    border: 1px solid var(--border);
    background: var(--bg);
    text-align: left;
    color: var(--text);
    transition:
      border-color 0.12s,
      background 0.12s;
  }

  .label-opt:hover {
    border-color: var(--c);
    background: color-mix(in srgb, var(--c) 12%, var(--bg));
  }

  .label-opt.on {
    border-color: var(--c);
    background: color-mix(in srgb, var(--c) 22%, var(--bg));
  }

  .dot {
    width: 12px;
    height: 12px;
    border-radius: 3px;
    background: var(--c);
    flex-shrink: 0;
  }

  .check {
    margin-left: auto;
    color: var(--c);
    font-weight: 700;
  }

  footer {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    margin-top: 0.25rem;
  }

  .spacer {
    flex: 1;
  }

  .error {
    margin: 0;
    color: var(--danger);
    font-size: 13px;
  }
</style>
