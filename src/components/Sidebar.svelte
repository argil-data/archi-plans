<script>
    export let items = [];
</script>

<div class="sidebar">
    <h3>Paramètres</h3>
    {#each items as item}
        <div class="sidebar-item">{item}</div>
    {/each}
</div>

<style>
    .sidebar {
        width: 18%;
        /* background-color: #f0f0f0; */
        padding: 20px;
        border-right: 1px solid #ccc;
    }
    .sidebar h3 {
        margin-top: 0;
    }
    .sidebar-item {
        padding: 8px 0;
        cursor: pointer;
    }
</style>
