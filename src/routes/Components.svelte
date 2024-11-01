<script>
    import Button from '../components/Button.svelte';
    import Input from '../components/Input.svelte';
    import Select from '../components/Select.svelte';

    import Modal from '../components/Modal.svelte';

    let isModalOpen = false;
</script>

<div style="padding: 1rem;">
    <h1>UI Components / Composants UI</h1>
    <Button label="Button" onClick={() => console.log('clicked')} />
    <Input placeholder="Input" value="Hello World" onInput={value => console.log(value)}  />
    <Button label="Button Modal" onClick={() => isModalOpen = true} />
    <Modal {isModalOpen} onClose={() => isModalOpen = false} > 
        <h2>Modal</h2>
        <p>This is a modal</p>
    </Modal>
</div>
