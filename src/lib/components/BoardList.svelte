<script lang="ts">
  import { focusOnMount } from "$lib/actions";
  import { api, runningInTauri } from "$lib/api";
  import type { Board } from "$lib/types";
  import { errMsg, toasts } from "$lib/toast.svelte";

  interface Props {
    onOpen: (id: string) => void;
  }

  let { onOpen }: Props = $props();

  let boards = $state<Board[]>([]);
  let loading = $state(true);
  let error = $state("");
  let creating = $state(false);
  let newName = $state("");
  let busy = $state(false);
  const inTauri = runningInTauri();

  async function load() {
    loading = true;
    error = "";
    if (!inTauri) {
      error =
        "Rode com npm run tauri dev — o IPC do Rust não existe no browser puro.";
      loading = false;
      return;
    }
    try {
      boards = await api.listBoards();
    } catch (e) {
      error = errMsg(e);
      toasts.error(error);
    } finally {
      loading = false;
    }
  }

  $effect(() => {
    load();
  });

  async function create() {
    const name = newName.trim();
    if (!name || busy) return;
    busy = true;
    try {
      const board = await api.createBoard(name);
      newName = "";
      creating = false;
      toasts.success(`Board "${board.name}" criado`);
      onOpen(board.id);
    } catch (e) {
      toasts.error(errMsg(e));
    } finally {
      busy = false;
    }
  }

  async function remove(board: Board, e: MouseEvent) {
    e.stopPropagation();
    if (!confirm(`Excluir o board "${board.name}"?`)) return;
    try {
      await api.deleteBoard(board.id);
      boards = boards.filter((b) => b.id !== board.id);
      toasts.success("Board excluído");
    } catch (err) {
      toasts.error(errMsg(err));
    }
  }

  function formatDate(iso: string) {
    try {
      return new Date(iso).toLocaleString("pt-BR", {
        day: "2-digit",
        month: "short",
        year: "numeric",
        hour: "2-digit",
        minute: "2-digit",
      });
    } catch {
      return iso;
    }
  }

  function onKeydown(e: KeyboardEvent) {
    const t = e.target as HTMLElement;
    if (t.tagName === "INPUT" || t.tagName === "TEXTAREA") return;
    if (e.key === "n" || e.key === "N") {
      e.preventDefault();
      creating = true;
    }
  }
</script>

<svelte:window onkeydown={onKeydown} />

<div class="home">
  <header>
    <div>
      <h1>Kanban</h1>
      <p class="subtitle">Local-first · Tauri + Rust + Svelte</p>
    </div>
    <button class="btn btn-primary" type="button" onclick={() => (creating = true)}>
      + Novo board
    </button>
  </header>

  {#if creating}
    <div class="create-box">
      <input
        placeholder="Nome do board…"
        bind:value={newName}
        use:focusOnMount
        onkeydown={(e) => {
          if (e.key === "Enter") create();
          if (e.key === "Escape") {
            creating = false;
            newName = "";
          }
        }}
      />
      <button class="btn btn-primary" type="button" onclick={create} disabled={busy}>
        Criar
      </button>
      <button
        class="btn btn-ghost"
        type="button"
        onclick={() => {
          creating = false;
          newName = "";
        }}
      >
        Cancelar
      </button>
    </div>
  {/if}

  {#if !inTauri}
    <div class="banner">
      <strong>Ambiente incorreto</strong>
      <p>
        Você abriu o frontend sem o shell Tauri. O backend Rust (SQLite / IPC) não está
        disponível.
      </p>
      <pre>npm run tauri dev</pre>
      <p class="hint-sm">
        Não use só <code>npm run dev</code> nem abra <code>http://localhost:1420</code> no Chrome/Firefox.
        Use a <strong>janela nativa</strong> que o Tauri abre.
      </p>
    </div>
  {:else if loading}
    <p class="status">Carregando boards…</p>
  {:else if error}
    <p class="status error">{error}</p>
  {:else if boards.length === 0}
    <div class="empty">
      <p>Nenhum board ainda.</p>
      <p class="hint">
        Crie o primeiro para começar — listas To Do / Doing / Done já vêm prontas.
        <br />
        Atalho: <kbd>n</kbd>
      </p>
    </div>
  {:else}
    <div class="grid">
      {#each boards as board (board.id)}
        <button class="board-card" type="button" onclick={() => onOpen(board.id)}>
          <span class="name">{board.name}</span>
          <span class="meta">Atualizado {formatDate(board.updated_at)}</span>
          <span
            class="delete"
            role="button"
            tabindex="0"
            title="Excluir"
            onclick={(e) => remove(board, e)}
            onkeydown={(e) => e.key === "Enter" && remove(board, e as unknown as MouseEvent)}
          >
            🗑
          </span>
        </button>
      {/each}
    </div>
  {/if}
</div>

<style>
  .home {
    min-height: 100vh;
    width: 100%;
    box-sizing: border-box;
    padding: 0.85rem clamp(0.6rem, 2vw, 1.25rem);
    background:
      radial-gradient(ellipse at 10% 0%, rgba(59, 130, 246, 0.15), transparent 45%),
      var(--bg);
  }

  header {
    display: flex;
    align-items: flex-start;
    justify-content: space-between;
    gap: 0.75rem;
    margin-bottom: 1rem;
  }

  h1 {
    margin: 0;
    font-size: 1.35rem;
    letter-spacing: -0.02em;
  }

  .subtitle {
    margin: 0.15rem 0 0;
    color: var(--text-muted);
    font-size: 11.5px;
  }

  .create-box {
    display: flex;
    flex-wrap: wrap;
    gap: 0.4rem;
    margin-bottom: 1rem;
    max-width: 480px;
  }

  .create-box input {
    flex: 1;
    min-width: 160px;
  }

  .grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(180px, 1fr));
    gap: 0.55rem;
  }

  .board-card {
    position: relative;
    text-align: left;
    background: linear-gradient(145deg, #1e3a5f 0%, #1a2332 100%);
    border: 1px solid var(--border);
    border-radius: var(--radius);
    padding: 0.75rem 0.7rem 0.65rem;
    min-height: 84px;
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
    transition:
      transform 0.12s,
      border-color 0.12s,
      box-shadow 0.12s;
  }

  .board-card:hover {
    transform: translateY(-2px);
    border-color: var(--accent);
    box-shadow: var(--shadow);
  }

  .name {
    font-weight: 650;
    font-size: 13px;
    padding-right: 1.25rem;
  }

  .meta {
    margin-top: auto;
    font-size: 11px;
    color: var(--text-muted);
  }

  .delete {
    position: absolute;
    top: 0.4rem;
    right: 0.4rem;
    opacity: 0;
    width: 22px;
    height: 22px;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    border-radius: var(--radius-sm);
    font-size: 13px;
  }

  .board-card:hover .delete {
    opacity: 0.7;
  }

  .delete:hover {
    opacity: 1 !important;
    background: rgba(239, 68, 68, 0.15);
  }

  .status,
  .empty {
    color: var(--text-muted);
  }

  .error {
    color: var(--danger);
  }

  .empty {
    padding: 3rem 0;
    text-align: center;
  }

  .hint {
    font-size: 13px;
  }

  .banner {
    max-width: 560px;
    padding: 1rem 1.1rem;
    margin-bottom: 1.5rem;
    border-radius: var(--radius);
    border: 1px solid rgba(239, 68, 68, 0.45);
    background: rgba(239, 68, 68, 0.1);
  }

  .banner p {
    margin: 0.45rem 0 0;
    color: var(--text-muted);
    font-size: 13px;
  }

  .banner pre {
    margin: 0.75rem 0 0;
    padding: 0.55rem 0.75rem;
    border-radius: var(--radius-sm);
    background: var(--bg);
    border: 1px solid var(--border);
    font-size: 13px;
    overflow-x: auto;
  }

  .hint-sm {
    font-size: 12px !important;
  }

  .banner code {
    font-size: 12px;
  }

  kbd {
    display: inline-block;
    padding: 0.1rem 0.4rem;
    border-radius: 4px;
    border: 1px solid var(--border);
    background: var(--bg-elevated);
    font-size: 12px;
    font-family: inherit;
  }
</style>
