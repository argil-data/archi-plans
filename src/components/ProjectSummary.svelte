<script>
    /**
     * @typedef {Object} Project
     * @property {number} id - L'identifiant unique du projet
     * @property {string} name - Le nom du projet
     * @property {string} status - Le statut actuel du projet
     * @property {number} progress - Le pourcentage d'avancement du projet
     */

    /** @type {Project[]} */
    export let projects = [];

</script>

<div class="project-summary">
    <h2>Résumé des Projets</h2>
    <!-- {#each projects as project}
        <li><a href="/projects/{project.id}" class="btn btn-primary">{project.name}</a></li>
    {/each} -->

    {#if projects.length !== 0 }
        <ul>
            {#each projects as project (project.id)}
                <li>
                <strong>{project.name}</strong>
                <p class="status">{project.status}</p>
                <!-- <progress value={project.progress} max="100"></progress> -->
                </li>
            {/each}
        </ul>
    {:else}
        <p>Aucun projet en cours.</p>
    {/if}

</div>

<style>
    .project-summary {
        /* background-color: #f0f0f0; */
        padding: 1rem;
        border-radius: 8px;
        display: block;
    }

    ul {
        list-style-type: none;
        padding: 0;
    }

    li {
        margin-bottom: 1rem;
    }

    .status {
        font-size: 0.8em;
        color: #666;
        margin-left: 0.5rem;
    }

    /* progress {
        width: 100%;
        height: 8px;
    } */
</style>
