<script>
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';
  import { createNewDeck } from '$lib/stores/deck';

  onMount(async () => {
    const newDeckId = await createNewDeck();
    await goto(`/deck/${newDeckId}/hand/offset/0`);
  });
</script>

<div>Creating a new poker deck...</div>