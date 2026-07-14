<script lang="ts">
  import type { Label } from "$lib/types";
  import { LABEL_COLORS } from "$lib/types";

  interface Props {
    labels: Label[];
    onCreate: (name: string, color: string) => Promise<void>;
    onUpdate: (id: string, name: string, color: string) => Promise<void>;
    onDelete: (id: string) => Promise<void>;
    onClose: () => void;
  }

  let { labels, onCreate, onUpdate, onDelete, onClose }: Props = $props();

  let newName = $state("");
  let newColor = $state<string>(LABEL_COLORS[5]);
  let busy = $state(false);
  let error = $state("");
  let editingId = $state<string | null>(null);
  let editName = $state("");
  let editColor = $state("");

  async function create() {
    const name = newName.trim();
    if (!name || busy) return;
    busy = true;
    error = "";
    try {
      await onCreate(name, newColor);
      newName = "";
      newColor = LABEL_COLORS[Math.floor(Math.random() * LABEL_COLORS.length)];
    } catch (e) {
      error = String(e);
    } finally {
      busy = false;
    }
  }

  function startEdit(label: Label) {
    editingId = label.id;
    editName = label.name;
    editColor = label.color;
  }

  async function saveEdit() {
    if (!editingId) return;
    const name = editName.trim();
    if (!name) {
      error = "Nome obrigatório";
      return;
    }
    busy = true;
    error = "";
    try {
      await onUpdate(editingId, name, editColor);
      editingId = null;
    } catch (e) {
      error = String(e);
    } finally {
      busy = false;
    }
  }

  async function remove(label: Label) {
    if (!confirm(`Excluir a etiqueta "${label.name}"?`)) return;
    busy = true;
    error = "";
    try {
      await onDelete(label.id);
      if (editingId === label.id) editingId = null;
    } catch (e) {
      error = String(e);
    } finally {
      busy = false;
    }
  }
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<div class="backdrop" role="presentation" onclick={onClose} onkeydown={(e) => e.key === "Escape" && onClose()}>
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <div
    class="panel"
    role="dialog"
    aria-modal="true"
    aria-label="Gerenciar etiquetas"
    tabindex="-1"
    onclick={(e) => e.stopPropagation()}
    onkeydown={(e) => e.stopPropagation()}
  >
    <header>
      <h2>Etiquetas do board</h2>
      <button class="icon-btn" type="button" onclick={onClose} aria-label="Fechar">✕</button>
    </header>

    <div class="create">
      <input
        placeholder="Nova etiqueta…"
        bind:value={newName}
        onkeydown={(e) => e.key === "Enter" && create()}
      />
      <div class="swatches">
        {#each LABEL_COLORS as c}
          <button
            type="button"
            class="swatch"
            class:on={newColor === c}
            style="background: {c}"
            title={c}
            aria-label="Cor {c}"
            onclick={() => (newColor = c)}
          ></button>
        {/each}
      </div>
      <button class="btn btn-primary btn-sm" type="button" onclick={create} disabled={busy}>
        Criar
      </button>
    </div>

    {#if error}
      <p class="error">{error}</p>
    {/if}

    <ul class="list">
      {#each labels as label (label.id)}
        <li>
          {#if editingId === label.id}
            <div class="edit-row">
              <span class="preview" style="background: {editColor}"></span>
              <input bind:value={editName} />
              <div class="swatches">
                {#each LABEL_COLORS as c}
                  <button
                    type="button"
                    class="swatch"
                    class:on={editColor === c}
                    style="background: {c}"
                    title={c}
                    aria-label="Cor {c}"
                    onclick={() => (editColor = c)}
                  ></button>
                {/each}
              </div>
              <button class="btn btn-primary btn-sm" type="button" onclick={saveEdit} disabled={busy}
                >OK</button
              >
              <button class="btn btn-ghost btn-sm" type="button" onclick={() => (editingId = null)}
                >Cancelar</button
              >
            </div>
          {:else}
            <button type="button" class="row" onclick={() => startEdit(label)}>
              <span class="preview" style="background: {label.color}"></span>
              <span class="name">{label.name}</span>
            </button>
            <button
              class="icon-btn"
              type="button"
              title="Excluir"
              aria-label="Excluir {label.name}"
              onclick={() => remove(label)}
              disabled={busy}
            >
              🗑
            </button>
          {/if}
        </li>
      {:else}
        <li class="empty">Nenhuma etiqueta ainda.</li>
      {/each}
    </ul>
  </div>
</div>

<style>
  .backdrop {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.55);
    z-index: 110;
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 1rem;
  }

  .panel {
    background: var(--bg-elevated);
    border: 1px solid var(--border);
    border-radius: var(--radius);
    width: min(440px, 100%);
    max-height: min(80vh, 640px);
    overflow-y: auto;
    padding: 1.1rem 1.2rem 1.2rem;
    box-shadow: var(--shadow);
    display: flex;
    flex-direction: column;
    gap: 0.85rem;
  }

  header {
    display: flex;
    align-items: center;
    justify-content: space-between;
  }

  h2 {
    margin: 0;
    font-size: 1.05rem;
  }

  .create {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .swatches {
    display: flex;
    flex-wrap: wrap;
    gap: 0.35rem;
  }

  .swatch {
    width: 22px;
    height: 22px;
    border-radius: 4px;
    border: 2px solid transparent;
    padding: 0;
  }

  .swatch.on {
    border-color: white;
    box-shadow: 0 0 0 1px rgba(0, 0, 0, 0.4);
  }

  .list {
    list-style: none;
    margin: 0;
    padding: 0;
    display: flex;
    flex-direction: column;
    gap: 0.35rem;
  }

  .list li {
    display: flex;
    align-items: center;
    gap: 0.35rem;
  }

  .row {
    flex: 1;
    display: flex;
    align-items: center;
    gap: 0.55rem;
    padding: 0.45rem 0.5rem;
    border-radius: var(--radius-sm);
    text-align: left;
    color: inherit;
  }

  .row:hover {
    background: rgba(255, 255, 255, 0.05);
  }

  .preview {
    width: 28px;
    height: 12px;
    border-radius: 3px;
    flex-shrink: 0;
  }

  .name {
    font-weight: 500;
  }

  .edit-row {
    flex: 1;
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    gap: 0.4rem;
  }

  .edit-row input {
    flex: 1;
    min-width: 100px;
  }

  .empty {
    color: var(--text-muted);
    font-size: 13px;
    padding: 0.5rem 0;
  }

  .error {
    margin: 0;
    color: var(--danger);
    font-size: 13px;
  }
</style>
