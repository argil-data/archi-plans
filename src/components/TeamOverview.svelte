<script>
  /**
   * @typedef {Object} TeamMember
   * @property {number} id - L'identifiant unique du membre de l'équipe
   * @property {string} name - Le nom du membre de l'équipe
   * @property {string} role - Le rôle du membre dans l'équipe
   * @property {string} avatar - L'URL de l'avatar du membre
   */

  /** @type {TeamMember[]} */
  export let team = [];
</script>

<div class="team-overview">
    <h2>Aperçu de l'Équipe</h2>
    {#if team.length !== 0}
        <ul>
            {#each team as member (member.id)}
            <li>
                <img src={member.avatar} alt={member.name} class="avatar" />
                <div class="member-info">
                <span class="name">{member.name}</span>
                <span class="role">{member.role}</span>
                </div>
            </li>
            {/each}
        </ul>
    {:else}
        <p>Aucun membre dans l'équipe.</p>
    {/if}
</div>

<style>
  .team-overview {
    /* background-color: #f5e6ff; */
    padding: 1rem;
    border-radius: 8px;
  }

  ul {
    list-style-type: none;
    padding: 0;
    display: flex;
    flex-wrap: wrap;
    flex-direction: column;
    gap: 1rem;
  }

  li {
    display: flex;
    align-items: center;
    background-color: #212121;
    padding: 0.5rem;
    border-radius: 4px;
    box-shadow: 0 2px 4px rgba(0,0,0,0.1);
    max-width: 90%;
  }

  .avatar {
    width: 40px;
    height: 40px;
    border-radius: 50%;
    margin-right: 0.5rem;
  }

  .member-info {
    display: flex;
    flex-direction: column;
  }

  .name {
    font-weight: bold;
  }

  .role {
    font-size: 0.8em;
    color: #e1e1e1;
  }
</style>