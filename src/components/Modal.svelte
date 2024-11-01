<script>
    export let isModalOpen = false;
    export let onClose;
</script>

{#if isModalOpen}
    <div style="position: fixed; top: 0; left: 0; right: 0; bottom: 0; background-color: rgba(0, 0, 0, 0.5); display: flex; align-items: center; justify-content: center;">
        <div style="background: white; padding: 20px; border-radius: 5px; width: 500px; color: black;">
            <slot />
            <button on:click={onClose} style="margin-top: 10px;">Fermer</button>
        </div>
    </div>
{/if}

<style>

</style>