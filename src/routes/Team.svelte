<script>
    import { onMount } from 'svelte';
    import { appStore } from '../stores/appStore';

    let team = [];
    let loading = false;
    let error = null;

    onMount(() => {
        const unsubscribe = appStore.subscribe(state => {
            team = state.team;
            loading = state.loading;
            error = state.error;
        });

        if (team.length === 0) {
            appStore.fetchAll();
        }

        return unsubscribe;
    });
</script>


<div>
    <h1>Team / Équipe</h1>
    
    {#if loading}
        <p>Chargement de l'équipe...</p>
    {:else if error}
        <p>Erreur : {error}</p>
    {:else}
        <ul>
            {#each team as member (member.id)}
                <li>{member.name} - {member.role}</li>
            {/each}
        </ul>
    {/if}
</div>