<script>
    export let label = 'Button';
    export let onClick;
    export let style = '';
    export let className = '';
    export let type = 'submit';
    export let variant = '';
    export let size = '';
</script>

<button style="{style}" class="{`${variant} ${size} ${className}`}" on:click={onClick} type="{type}">
    {label}
</button>
