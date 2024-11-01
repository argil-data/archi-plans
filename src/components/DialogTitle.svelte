<!-- DialogTitle.svelte -->
<script>
    // export let children;
  </script>
  
  <h2 class="text-xl font-bold">
    <slot></slot>
  </h2>
  
  <style>
    .text-xl {
      font-size: 1.25rem;
    }
    .font-bold {
      font-weight: bold;
    }
  </style>
  