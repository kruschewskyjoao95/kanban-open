<script lang="ts">
  import type { Label } from "$lib/types";

  interface Props {
    labels: Label[];
    size?: "sm" | "md";
    /** Show name text; if false, only color bars (compact card view). */
    showName?: boolean;
  }

  let { labels, size = "sm", showName = true }: Props = $props();
</script>

{#if labels.length > 0}
  <div class="chips" class:compact={!showName} class:md={size === "md"}>
    {#each labels as label (label.id)}
      <span
        class="chip"
        style="--c: {label.color}"
        title={label.name}
      >
        {#if showName}
          {label.name}
        {/if}
      </span>
    {/each}
  </div>
{/if}

<style>
  .chips {
    display: flex;
    flex-wrap: wrap;
    gap: 0.3rem;
  }

  .chips.compact {
    gap: 0.25rem;
  }

  .chip {
    display: inline-flex;
    align-items: center;
    max-width: 100%;
    padding: 0.05rem 0.3rem;
    border-radius: 3px;
    font-size: 9.5px;
    font-weight: 600;
    line-height: 1.25;
    color: #fff;
    background: var(--c);
    text-shadow: 0 1px 1px rgba(0, 0, 0, 0.25);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .chips.compact .chip {
    width: 28px;
    height: 6px;
    padding: 0;
    border-radius: 2px;
  }

  .chips.md .chip {
    font-size: 11px;
    padding: 0.2rem 0.5rem;
  }
</style>
