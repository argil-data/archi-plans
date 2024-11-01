<script>
    import Input from "./Input.svelte";
    
    export let headers = [];
    export let rows = [];
    export let onEditCell;

</script>
<table class="table">
    <thead>
      <tr>
        {#each headers as header}
          <th>{header}</th>
        {/each}
      </tr>
    </thead>
    <tbody>
      {#each rows as row}
        <tr>
          {#each Object.keys(row) as key, index}
            <td>
              {#if key === 'original' || key === 'mutations'}
                <Input
                    type="number"
                    bind:value={row[key]}
                    onInput={(e) => onEditCell(row.id, key, e.target.value)}
                    className="table-input"
                />
              {:else}
                {row[key]}
              {/if}
            </td>
          {/each}
        </tr>
      {/each}
    </tbody>
  </table>

<style>
.table {
    /* width: 100%; */
    border-collapse: collapse;
    margin-top: 20px;
    overflow-x: scroll;
  }
  .table th, .table td {
    border: 1px solid #ccc;
    padding: 8px;
    text-align: left;
  }
  /* .table input {
    width: 100%;
    border: none;
    background: transparent;
  } */
</style>