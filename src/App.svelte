<h1>svelte-spa-router sample</h1>
<h2>Basic routing</h2>

<!-- Show the router -->
Routes:
<Router {routes} />

<script>
// Import the router component
// Normally, this would be: `import Router from 'svelte-spa-router'`
  import Router from 'svelte-spa-router';

// Import the list of routes
  import routes from './routes';
</script>
