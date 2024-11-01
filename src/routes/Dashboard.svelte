<script>
    import { onMount } from 'svelte';
    import { appStore } from '../stores/appStore';
    import { invoke } from '@tauri-apps/api/tauri';
    import ProjectSummary from '../components/ProjectSummary.svelte';
    import TaskList from '../components/TaskList.svelte';
    import TeamOverview from '../components/TeamOverview.svelte';
	import ProjectPrompt from '../components/ProjectPrompt.svelte';
	import ProjectsManagementTable from '../components/ProjectsManagementTable.svelte';
	import ProjectsDelta from '../components/ProjectsDelta.svelte';

    import { Card } from 'argil-svelte';
    import "carbon-components-svelte/css/g90.css";
    import { CopyButton } from "carbon-components-svelte";
    import { Loading } from "carbon-components-svelte";
    import { Badge } from 'flowbite-svelte';


    /** @type {string|null} */
    let errorMessage = null;
    let error = null;

    let loading = false;

    /** @type {Project[]} */
    let projects = [];
    /** @type {Task[]} */
    let tasks = [];
    /** @type {TeamMember[]} */
    let team = [];

    /**
     * @typedef {Object} Project
     * @property {number} id - L'identifiant unique du projet
     * @property {string} name - Le nom du projet
     * @property {string} status - Le statut actuel du projet
     */

    /**
     * @typedef {Object} Task
     * @property {number} id - L'identifiant unique de la tâche
     * @property {string} title - Le titre de la tâche
     * @property {string} assignee - La personne assignée à la tâche
     * @property {string} dueDate - La date d'échéance de la tâche
     * @property {boolean} completed - Indique si la tâche est terminée
     */

    /**
     * @typedef {Object} TeamMember
     * @property {number} id - L'identifiant unique du membre de l'équipe
     * @property {string} name - Le nom du membre de l'équipe
     * @property {string} role - Le rôle du membre dans l'équipe
     * @property {string} avatar - L'URL de l'avatar du membre
     */

    // onMount(async () => {
    //     // Fetch data from your backend or local storage
    //     // This is where you'd integrate with Tauri's API
    //     try {
    //         projects = await fetchProjects();
    //         // tasks = await invoke('fetch_tasks');
    //         // team = await invoke('fetch_team');
    //         tasks = await fetchTasks();
    //         team = await fetchTeam();
    //     } catch (error) {
    //         // Gérer l'erreur (par exemple, afficher un message à l'utilisateur)
    //         console.error('Erreur lors de la récupération des données:', error);
    //         errorMessage = "Une erreur s'est produite lors du chargement des données. Veuillez réessayer.";
    //     }
    // });

    onMount(() => {
        const unsubscribe = appStore.subscribe(state => {
            projects = state.projects;
            tasks = state.tasks;
            team = state.team;
            loading = state.loading;
            error = state.error;
        });

        if (projects.length === 0 && tasks.length === 0 && team.length === 0) {
            appStore.fetchAll();
        }

        return unsubscribe;
    });

    /**
     * Valide et nettoie les données du projet
     * @param {any} project - Les données du projet à valider
     * @returns {Project|null} Le projet validé ou null si invalide
     */
    function validateProject(project) {
        if (typeof project.id !== 'number' || 
            typeof project.name !== 'string' || 
            typeof project.status !== 'string') {
        console.warn('Projet invalide:', project);
        return null;
        }
        return {
        id: project.id,
        name: project.name,
        status: project.status
        };
    }

    /**
     * Récupère et valide les projets depuis le backend Tauri
     * @returns {Promise<Project[]>}
     */
    async function fetchProjects() {
        const rawProjects = await invoke('fetch_projects');
        return rawProjects.map(validateProject).filter(p => p !== null);
    }


  /**
   * Valide et nettoie les données de la tâche
   * @param {any} task - Les données de la tâche à valider
   * @returns {Task|null} La tâche validée ou null si invalide
   */
   function validateTask(task) {
    if (typeof task.id !== 'number' || 
        typeof task.title !== 'string' || 
        typeof task.assignee !== 'string' || 
        typeof task.due_date !== 'string' || 
        typeof task.completed !== 'boolean') {
      console.warn('Tâche invalide:', task);
      return null;
    }
    return {
      id: task.id,
      title: task.title,
      assignee: task.assignee,
      dueDate: task.due_date,
      completed: task.completed
    };
  }

  /**
   * Récupère et valide les tâches depuis le backend Tauri
   * @returns {Promise<Task[]>}
   */
  async function fetchTasks() {
    const rawTasks = await invoke('fetch_tasks');
    return rawTasks.map(validateTask).filter(t => t !== null);
  }

  /**
   * Valide et nettoie les données du membre de l'équipe
   * @param {any} member - Les données du membre à valider
   * @returns {TeamMember|null} Le membre validé ou null si invalide
   */
  function validateTeamMember(member) {
    if (typeof member.id !== 'number' || 
        typeof member.name !== 'string' || 
        typeof member.role !== 'string' || 
        typeof member.avatar !== 'string') {
      console.warn('Membre de l\'équipe invalide:', member);
      return null;
    }
    return {
      id: member.id,
      name: member.name,
      role: member.role,
      avatar: member.avatar
    };
  }

  /**
   * Récupère et valide les membres de l'équipe depuis le backend Tauri
   * @returns {Promise<TeamMember[]>}
   */
  async function fetchTeam() {
    const rawTeam = await invoke('fetch_team');
    return rawTeam.map(validateTeamMember).filter(m => m !== null);
  }
</script>

<div>

    <h1>Dashboard / Tableau de bord</h1>
    <div style="margin: 1rem; display: flex; gap: 1rem;">
        <!-- <Loading withOverlay={false} /> -->
        <CopyButton text="Carbon svelte" />
        <Badge color="purple">Purple</Badge>

    </div>

    <!-- 
    <Card 
        title="Argil Svelte"
        content="Le dashboard de l'application Argil Svelte"
        children="Ceci est un exemple de dashboard"
        url="/projects"
        footer="mijong"
    ></Card> -->

    {#if loading}
    <p>Chargement des données...</p>
    {:else if error}
        <div class="error-message">{error}</div>
    {:else}
        <div class="dashboard-grid">
            <ProjectSummary {projects} />
            <TaskList {tasks} />
            <TeamOverview {team} />
        </div>
    {/if}
    <!-- 
    {#if errorMessage}
        <div class="error-message">{errorMessage}</div>
    {:else}
        <div class="dashboard-grid">
            <ProjectSummary {projects} />
            <TaskList {tasks} />
            <TeamOverview {team} />
        </div>
    {/if} 
    -->
    <ProjectPrompt />
    <ProjectsManagementTable></ProjectsManagementTable>
    <!-- <ProjectsDelta></ProjectsDelta> -->

</div>

<style>
    .dashboard-grid {
        display: grid;
        grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
        gap: 1rem;
    }
    .error-message {
        color: red;
        padding: 1rem;
        border: 1px solid red;
        border-radius: 4px;
        margin-bottom: 1rem;
    }
</style>