<script>
  import { page } from '$app/stores';
</script>

<svelte:head>
  <title>Error</title>
</svelte:head>

<div class="error-container">
  <h1>{$page.status}: {$page.error.message}</h1>
  <p>Sorry, something went wrong with the poker game.</p>
  <a href="/" class="home-link">Return to game</a>
</div>

<style>
  .error-container {
    max-width: 600px;
    margin: 2rem auto;
    padding: 2rem;
    text-align: center;
  }

  .home-link {
    display: inline-block;
    margin-top: 1rem;
    padding: 0.75rem 1.5rem;
    background: #4299e1;
    color: white;
    border-radius: 4px;
    text-decoration: none;
  }

  .home-link:hover {
    background: #3182ce;
  }
</style>