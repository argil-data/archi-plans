
<script>
    import { onMount } from 'svelte';
    import { appStore } from '../stores/appStore';

    let tasks = [];
    let loading = false;
    let error = null;

    onMount(() => {
        const unsubscribe = appStore.subscribe(state => {
            tasks = state.tasks;
            loading = state.loading;
            error = state.error;
        });

        if (tasks.length === 0) {
            appStore.fetchAll();
        }

        return unsubscribe;
    });
</script>

<div>
    <h1>Tasks / Tâches</h1>
    
    {#if loading}
        <p>Chargement des tâches...</p>
    {:else if error}
        <p>Erreur : {error}</p>
    {:else}
        <ul>
            {#each tasks as task (task.id)}
                <li>{task.title} - Assigné à : {task.assignee} - Échéance : {task.due_date}</li>
            {/each}
        </ul>
    {/if}

</div>