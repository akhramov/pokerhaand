<script>
  export let hand;
  import CardSvg from './Card.svelte';
</script>

<div class="hand">
  <h2>Your Hand</h2>
  <CardSvg />

  {#if hand.hand.cards && hand.hand.cards.length > 0}
    <div class="cards">
      {#each hand.hand.cards as card}
        <div class="card">
          <svg viewBox="0 0 170 250"><use href="#{card}" /></svg>
        </div>
      {/each}
    </div>
  {:else}
    <p class="no-cards">No cards in this hand</p>
  {/if}

  <span class="rank">{hand.hand.ranking_category}</span>
</div>

<style>
  .hand {
    border: 1px solid #e2e8f0;
    border-radius: 8px;
    padding: 2rem;
    background: white;
    box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
  }

  .cards {
    display: flex;
    gap: 1rem;
    margin: 1.5rem 0;
    flex-wrap: wrap;
  }

  .card {
    width: 80px;
    height: 120px;
    border-radius: 8px;
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
    font-size: 1.5rem;
    font-weight: bold;
    background: white;
  }

  .card.red {
    color: #e53e3e;
  }

  .rank {
    font-size: 1.8rem;
  }

  .suit {
    font-size: 2rem;
  }

  .hand-strength {
    margin-top: 1.5rem;
    padding-top: 1.5rem;
    border-top: 1px solid #e2e8f0;
  }

  .no-cards {
    text-align: center;
    padding: 2rem;
    color: #718096;
  }
</style>