<script>
    /**
     * @typedef {Object} Task
     * @property {number} id - L'identifiant unique de la tâche
     * @property {string} title - Le titre de la tâche
     * @property {string} assignee - La personne assignée à la tâche
     * @property {string} due_date - La date d'échéance de la tâche
     * @property {boolean} completed - Indique si la tâche est terminée
     */

    /** @type {Task[]} */
    export let tasks = [];

    /**
     * Bascule l'état de complétion d'une tâche
     * @param {number} taskId - L'identifiant de la tâche à modifier
     */
    function toggleTaskCompletion(taskId) {
        tasks = tasks.map(task => 
        task.id === taskId ? {...task, completed: !task.completed} : task
        );
    }
</script>

<div class="task-list">
    <h2>Liste des Tâches</h2>
    {#if tasks.length !== 0}
        <ul>
            {#each tasks as task}
            <li class:completed={task.completed}>
                <input 
                type="checkbox" 
                checked={task.completed} 
                on:change={() => toggleTaskCompletion(task.id)}
                />
                <span class="title">{task.title}</span>
                <span class="assignee">{task.assignee}</span>
                <span class="due-date">{task.due_date}</span>
            </li>
            {/each}
        </ul>
    {:else}
        <p>Aucune tâche en cours.</p>
    {/if}
</div>

<style>
  .task-list {
    /* background-color: #e6f3ff; */
    padding: 1rem;
    border-radius: 8px;
  }

  ul {
    list-style-type: none;
    padding: 0;
  }

  li {
    display: flex;
    align-items: center;
    margin-bottom: 0.5rem;
  }

  .completed {
    text-decoration: line-through;
    color: #888;
  }

  .title {
    flex-grow: 1;
    margin-left: 0.5rem;
  }

  .assignee, .due-date {
    font-size: 0.8em;
    color: #666;
    margin-left: 1rem;
  }
</style>