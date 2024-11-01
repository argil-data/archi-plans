<script>
    export let type = 'text';
    export let value = '';
    export let placeholder = '';
    export let onInput;
    export let className = '';

</script>


{#if type === 'text'}
  <input
    type="text"
    bind:value
    placeholder={placeholder}
    on:input={onInput}
    class="input {className}"
  />
{:else if type === 'number'}
  <input
    type="number"
    bind:value
    placeholder={placeholder}
    on:input={onInput}
    class="input {className}"
  />
{:else if type === 'password'}
  <input
    type="password"
    bind:value
    placeholder={placeholder}
    on:input={onInput}
    class="input {className}"
  />
{:else if type === 'email'}
  <input
    type="email"
    bind:value
    placeholder={placeholder}
    on:input={onInput}
    class="input {className}"
  />
{:else if type === 'date'}
  <input
    type="date"
    bind:value
    placeholder={placeholder}
    on:input={onInput}
    class="input {className}"
  />
{:else}
  <input
    type="text"
    bind:value
    placeholder={placeholder}
    on:input={onInput}
    class="input {className}"
  />
{/if}

<style>
    .input {
        width: 100%;
        padding: 5px;
        margin-bottom: 10px;
        max-width: fit-content;    
    }
</style>