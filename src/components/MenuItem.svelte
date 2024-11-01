<!-- MenuItem.svelte -->
<script>
    import ChevronDown from './ChevronDown.svelte';
    import ChevronRight from './ChevronRight.svelte';

    export let title;
    export let children;
    export let expanded;
    export let onToggle;
</script>
  
<li class="mb-2">
    <button class="flex items-center cursor-pointer" on:click={onToggle} type="button"> 
        {#if expanded}
            <ChevronDown className="chevron h-4 w-4 mr-1" />
        {:else}
            <ChevronRight className="chevron h-4 w-4 mr-1" />
        {/if}
        {title}
    </button>
    {#if expanded && children}
        <ul class="ml-4 mt-1">
        {#each children as item, index}
            <li key={index} class="mb-1 text-sm">{item}</li>
        {/each}
        </ul>
    {/if}
</li>

<style>
    /* .chevron {
        width: 1rem;
        height: 1rem;
    } */

</style>