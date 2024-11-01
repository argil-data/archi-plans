<!-- ChevronRight.svelte -->
<script>
    export let className = '';
  </script>
  
  <svg class="chevron {className}" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
    <polyline points="9 18 15 12 9 6"/>
  </svg>

  <style>
    .chevron {
        width: 1rem;
        height: 1rem;
    }

</style>