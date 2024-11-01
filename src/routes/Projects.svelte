<script>
    import { onMount } from 'svelte';
    import { appStore } from '../stores/appStore';
	import ProjectsManagementTable from '../components/ProjectsManagementTable.svelte';
	import ProjectsDelta from '../components/ProjectsDelta.svelte';

    import Card from '@smui/card';
    import * as Accordion from "$lib/components/ui/accordion";

    // import DataTable from "./data-table.svelte";
    // import { Card } from 'argil-svelte';

    let projects = [];
    let loading = false;
    let error = null;

    onMount(() => {
        const unsubscribe = appStore.subscribe(state => {
            projects = state.projects;
            loading = state.loading;
            error = state.error;
        });

        if (projects.length === 0) {
            appStore.fetchAll();
        }

        return unsubscribe;
    });
</script>

<div>
    <h1>Projects Projets</h1>
    <!-- <Card title="Argil Svelte" content="Le dashboard de l'application Argil Svelte" children="Ceci est un exemple de dashboard" url="/projects" footer="mijong"></Card> -->
    <div class="card-container">
        <Card variant="outlined" padded>An outlined, padded card.</Card>
        <Accordion.Root>
            <Accordion.Item value="item-1">
              <Accordion.Trigger>Is it accessible?</Accordion.Trigger>
              <Accordion.Content>
                Yes. It adheres to the WAI-ARIA design pattern.
              </Accordion.Content>
            </Accordion.Item>
          </Accordion.Root>
          
        <!-- <DataTable /> -->
    </div>

    <div style="margin: 2rem 0;">
        {#if loading}
            <p>Chargement des projets...</p>
        {:else if error}
            <p>Erreur : {error}</p>
        {:else}
            <ul>
                {#each projects as project (project.id)}
                    <li>{project.name} - {project.status}</li>
                {/each}
            </ul>
        {/if}
    </div>

    <ProjectsManagementTable></ProjectsManagementTable>
    <ProjectsDelta></ProjectsDelta>
</div>

