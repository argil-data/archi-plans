<script>
    export let options = [];
    export let value;
    export let onSelect;
    export let style = '';
</script>
  
<select bind:value on:change={onSelect} style="{style}">
    {#each options as option}
        <option value={option.value}>{option.label}</option>
    {/each}
</select>
  