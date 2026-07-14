<script lang="ts">
  import BoardList from "$lib/components/BoardList.svelte";
  import BoardView from "$lib/components/BoardView.svelte";

  let activeBoardId = $state<string | null>(null);
</script>

{#if activeBoardId}
  <BoardView boardId={activeBoardId} onBack={() => (activeBoardId = null)} />
{:else}
  <BoardList onOpen={(id) => (activeBoardId = id)} />
{/if}
