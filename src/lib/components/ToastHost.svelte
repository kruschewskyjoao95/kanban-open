<script lang="ts">
  import { toasts } from "$lib/toast.svelte";
</script>

<div class="toast-host" aria-live="polite" aria-relevant="additions">
  {#each toasts.items as t (t.id)}
    <div class="toast toast-{t.kind}" role="status">
      <span class="msg">{t.message}</span>
      <button
        type="button"
        class="close"
        aria-label="Fechar"
        onclick={() => toasts.dismiss(t.id)}
      >
        ✕
      </button>
    </div>
  {/each}
</div>

<style>
  .toast-host {
    position: fixed;
    right: 1rem;
    bottom: 1rem;
    z-index: 200;
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    max-width: min(360px, calc(100vw - 2rem));
    pointer-events: none;
  }

  .toast {
    pointer-events: auto;
    display: flex;
    align-items: flex-start;
    gap: 0.65rem;
    padding: 0.7rem 0.75rem 0.7rem 0.9rem;
    border-radius: var(--radius-sm);
    border: 1px solid var(--border);
    background: var(--bg-elevated);
    box-shadow: var(--shadow);
    animation: slide-in 0.18s ease-out;
  }

  .toast-success {
    border-color: rgba(34, 197, 94, 0.45);
    background: linear-gradient(135deg, rgba(34, 197, 94, 0.12), var(--bg-elevated));
  }

  .toast-error {
    border-color: rgba(239, 68, 68, 0.5);
    background: linear-gradient(135deg, rgba(239, 68, 68, 0.14), var(--bg-elevated));
  }

  .toast-info {
    border-color: rgba(59, 130, 246, 0.45);
    background: linear-gradient(135deg, rgba(59, 130, 246, 0.12), var(--bg-elevated));
  }

  .msg {
    flex: 1;
    font-size: 13px;
    line-height: 1.4;
    word-break: break-word;
  }

  .close {
    flex-shrink: 0;
    width: 22px;
    height: 22px;
    border-radius: 4px;
    color: var(--text-muted);
    font-size: 11px;
    display: inline-flex;
    align-items: center;
    justify-content: center;
  }

  .close:hover {
    background: rgba(255, 255, 255, 0.08);
    color: var(--text);
  }

  @keyframes slide-in {
    from {
      opacity: 0;
      transform: translateY(8px);
    }
    to {
      opacity: 1;
      transform: translateY(0);
    }
  }
</style>
