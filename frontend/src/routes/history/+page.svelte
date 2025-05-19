<script>
  import { page } from '$app/stores';
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';
  import {
    historyItems,
    historyNextOffset,
    fetchHistory,
  } from '$lib/stores/history.js';

  // Initial load
  onMount(() => {
    fetchHistory(0);
  });

  async function loadNext() {
    if ($historyNextOffset !== null) {
      await fetchHistory($historyNextOffset);
    }
  }

  function handleNavigate(deck, offset) {
    goto(`/deck/${deck}/hand/offset/${offset}`);
  }
</script>

<svelte:head>
  <title>History</title>
</svelte:head>

<main class="history-container">
  <h2>Hand History</h2>

  {#if $historyItems.length === 0}
    <p>No history yet.</p>
  {:else}
    <table>
      <thead>
        <tr>
          <th>Deck</th>
          <th>Offset</th>
          <th>Timestamp</th>
        </tr>
      </thead>
      <tbody>
        {#each $historyItems as item}
          <tr>
            <td>{item.deck}</td>
            <td>
              <a href="#" on:click|preventDefault={() => handleNavigate(item.deck, item.offset)}>
                {item.offset}
              </a>
            </td>
            <td>{new Date(item.time).toLocaleString()}</td>
          </tr>
        {/each}
      </tbody>
    </table>

    {#if $historyNextOffset !== null}
      <button on:click={loadNext}>Load more</button>
    {/if}
  {/if}
</main>

<style>
  .history-container {
    max-width: 800px;
    margin: 0 auto;
    padding: 1rem;
  }

  table {
    width: 100%;
    border-collapse: collapse;
    margin-bottom: 1rem;
  }

  th, td {
    border: 1px solid #ddd;
    padding: 0.75rem;
    text-align: left;
  }

  th {
    background-color: #f4f4f4;
  }

  a {
    color: #4a6fa5;
    text-decoration: none;
  }

  a:hover {
    text-decoration: underline;
  }

  button {
    padding: 0.5rem 1rem;
    background: #4a6fa5;
    color: white;
    border: none;
    border-radius: 4px;
    cursor: pointer;
    font-size: 0.9rem;
  }

  button:hover {
    background: #3a5a80;
  }

  .error-state {
    background: #ffebee;
    border-left: 4px solid #f44336;
    padding: 1rem;
    margin-bottom: 1rem;
    border-radius: 0 4px 4px 0;
  }
</style>
