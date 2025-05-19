<script>
  import { page } from '$app/stores';
  import { goto } from '$app/navigation';
  import { onMount } from 'svelte';
  import {
    currentDeck,
    currentOffset,
    currentHand,
    error,
    createNewDeck,
    fetchHand
  } from '$lib/stores/deck';
  import Hand from '$lib/components/Hand.svelte';

  // Initialize from URL params
  $: {
    const { deckId, offset } = $page.params;
    const numOffset = parseInt(offset) || 0;

    if (deckId && deckId !== $currentDeck) {
      currentDeck.set(deckId);
    }
    if (numOffset !== $currentOffset) {
      currentOffset.set(numOffset);
    }
  }

  // Fetch hand when params change
  $: {
    if ($currentDeck) {
      fetchHand($currentOffset).catch(console.error);
    }
  }

  // Navigation handlers
  async function handleNewDeck() {
    try {
      const newDeckId = await createNewDeck();
      await goto(`/deck/${newDeckId}/hand/offset/0`);
    } catch (err) {
      console.error('Failed to create new deck:', err);
    }
  }

  async function handleNextHand() {
    const newOffset = $currentOffset + 5;
    await goto(`/deck/${$currentDeck}/hand/offset/${newOffset}`);
  }

  async function handlePrevHand() {
    const newOffset = Math.max(0, $currentOffset - 5);
    await goto(`/deck/${$currentDeck}/hand/offset/${newOffset}`);
  }

  function currentHandNumber() {
    return ($currentOffset / 5) + 1;
  }
</script>

<svelte:head>
  <title>Deck {$currentDeck} - Hand {currentHandNumber()}</title>
</svelte:head>

<main class="hand-container">
  <div class="navigation-controls">
    <div class="deck-info">
      <h2>Deck: {$currentDeck}</h2>
      <span class="hand-number">Hand #{currentHandNumber()}</span>
    </div>

    <div class="action-buttons">
      <button on:click={handlePrevHand} disabled={$currentOffset == 0}>
        ← Previous
      </button>
      <button on:click={handleNextHand} disabled={$currentOffset >= 45}>
        Next →
      </button>
      <button on:click={handleNewDeck} class="new-deck">
        New Deck
      </button>
    </div>
  </div>

  {#if $error}
    <div class="error-state">
      <p>Error loading hand: {$error}</p>
      <button on:click={() => fetchHand($currentOffset)}>Retry</button>
    </div>
  {:else if $currentHand}
    <Hand hand={$currentHand} />
  {/if}

  <div class="nav-links">
    <a href="/history">🕓 View History</a>
  </div>
</main>

<style>
  .hand-container {
    max-width: 800px;
    margin: 0 auto;
    padding: 1rem;
  }

  .navigation-controls {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 2rem;
    flex-wrap: wrap;
    gap: 1rem;
  }

  .deck-info {
    display: flex;
    align-items: baseline;
    gap: 0.2rem;
  }

  .deck-info h2 {
    margin: 0;
    font-size: 1.25rem;
    color: #333;
  }

  .hand-number {
    background: #f0f0f0;
    padding: 0.25rem 0.5rem;
    border-radius: 4px;
    font-size: 0.9rem;
  }

  .action-buttons {
    display: flex;
    gap: 0.5rem;
  }

  button {
    padding: 0.5rem 1rem;
    background: #4a6fa5;
    color: white;
    border: none;
    border-radius: 4px;
    cursor: pointer;
    font-size: 0.9rem;
    transition: background 0.2s;
  }

  button:hover {
    background: #3a5a80;
  }

  button:disabled {
    background: #cccccc;
    cursor: not-allowed;
  }

  button.new-deck {
    background: #e74c3c;
  }

  button.new-deck:hover {
    background: #c0392b;
  }

  .nav-links {
    margin-top: 1rem;
  }

  .nav-links a {
    color: #4a6fa5;
    text-decoration: none;
    font-weight: bold;
  }

  .nav-links a:hover {
    text-decoration: underline;
  }

  .error-state {
    background: #ffebee;
    border-left: 4px solid #f44336;
    padding: 1rem;
    margin-bottom: 1rem;
    border-radius: 0 4px 4px 0;
  }

  .error-state button {
    margin-top: 0.5rem;
    background: #f44336;
  }

  .loading-state {
    display: flex;
    gap: 1rem;
    justify-content: center;
    padding: 2rem 0;
  }

  .card-skeleton {
    width: 80px;
    height: 120px;
    background: #f0f0f0;
    border-radius: 8px;
    animation: pulse 1.5s infinite ease-in-out;
  }

  @keyframes pulse {
    0%, 100% { opacity: 0.6; }
    50% { opacity: 0.3; }
  }
</style>
