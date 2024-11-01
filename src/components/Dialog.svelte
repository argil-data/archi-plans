<!-- Dialog.svelte -->
<script>
    export let open = false;
    // export let onOpenChange;
  </script>
  
  {#if open}
    <div class="fixed inset-0 flex items-center justify-center bg-black bg-opacity-50">
      <div class="bg-white p-6 rounded shadow-lg text-gray-700 dark:text-gray-100 dark:bg-gray-900">
        <slot></slot>
      </div>
    </div>
  {/if}
  
  <style>
    .fixed {
      position: fixed;
    }
    .inset-0 {
      top: 0;
      right: 0;
      bottom: 0;
      left: 0;
    }
    .flex {
      display: flex;
    }
    .items-center {
      align-items: center;
    }
    .justify-center {
      justify-content: center;
    }
    .bg-black {
      background-color: black;
    }
    .bg-opacity-50 {
      background-color: rgba(0, 0, 0, 0.5);
    }
    /* .bg-white {
      background-color: white;
    } */
    .p-6 {
      padding: 1.5rem;
    }
    .rounded {
      border-radius: 0.25rem;
    }
    .shadow-lg {
      box-shadow: 0 10px 15px -3px rgba(0, 0, 0, 0.1), 0 4px 6px -2px rgba(0, 0, 0, 0.05);
    }
  </style>
  