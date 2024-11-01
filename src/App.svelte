<script>
  import { Router, Link, Route } from "svelte-routing";
  import Dashboard from "./routes/Dashboard.svelte";
  import Projects from "./routes/Projects.svelte";
  import Tasks from "./routes/Tasks.svelte";
  import Team from "./routes/Team.svelte";
  import Components from "./routes/Components.svelte";

  export let url = "";
</script>

<Router {url}>
  <nav class="app-nav">
    <Link style="padding: 0 0.5rem" to="/">Dashboard</Link>
    <Link style="padding: 0 0.5rem" to="/projects">Projects</Link>
    <Link style="padding: 0 0.5rem" to="/tasks">Tasks</Link>
    <Link style="padding: 0 0.5rem" to="/team">Team</Link>
    <Link style="padding: 0 0.5rem" to="/components">Components</Link>
  </nav>
  <main>
    <Route path="/" component={Dashboard} />
    <Route path="/projects" component={Projects} />
    <Route path="/tasks" component={Tasks} />
    <Route path="/team" component={Team} />
    <Route path="/components" component={Components} />
  </main>
</Router>

<style>
  .app-nav {
    padding: 1rem;   
  }
  /* .nav-link {
    padding: 1rem;
    margin: 1rem;
    font-size: 18px;
    text-decoration: underline;
  } */
</style>
