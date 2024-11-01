<script>
    import Modal from './Modal.svelte';
    import Sidebar from './Sidebar.svelte';
    import MainContent from './MainContent.svelte';
    import Button from './Button.svelte';
    import Input from './Input.svelte';
    import Select from './Select.svelte';
    import Table from './Table.svelte';
  
    let activeTab = 'projects';
    let isModalOpen = false;

    const sidebarItems = [
        '📊 Données du bureau et du projet',
        '📍 Adresses du projet',
        '📝 Textes des formulaires',
        '💰 Estimation des coûts',
        '📄 Devis original',
        '📊 Devis général révisé',
        '📋 Liste des soumissions/contrats',
        '📒 Journal des travaux',
        '💼 Soumissions / devis détaillés',
        '🔄 Comparaisons des offres',
        '📑 Contrats, adjudications',
        '📏 Saisir les métrés',
        '💳 Paiements',
        '🏗️ Comptabilité de chantier',
        '📅 Planning'
    ];

    let projectData = [
        { id: 2, designation: 'Bâtiment', original: 146000, mutations: 1000, revised: 148000, contract: 70432.35, forecast: 14544.30, payment: 111502, probable: 146502, deviation: -25602 },
        { id: 21, designation: 'Gros oeuvre', original: 146000, mutations: 1000, revised: 148000, contract: 45432.35, forecast: 14544.30, payment: 146502, probable: 146502, deviation: -602 },
        { id: 32, designation: 'Préliminaires', original: 146000, mutations: 1000, revised: 148000, contract: 45432.35, forecast: 14544.30, payment: 146502, probable: 146502, deviation: -602 },

    ];
  
    let newCFC = {
        id: '',
        designation: '',
        original: 0,
        mutations: 0,
        revised: 0,
        contract: 0,
        forecast: 0,
        payment: 0,
        probable: 0,
        deviation: 0
    };
  
    function handleEditCell(id, field, value) {
        projectData = projectData.map(item =>
            item.id === id ? { ...item, [field]: value } : item
        );
    }
  
    function handleAddCFC() {
        projectData = [...projectData, newCFC];
        newCFC = {
            id: '',
            designation: '',
            original: 0,
            mutations: 0,
            revised: 0,
            contract: 0,
            forecast: 0,
            payment: 0,
            probable: 0,
            deviation: 0
            };
        isModalOpen = false;
    }
    function openLink(){
        console.log('open link');
    }

</script>
  
  <div style="display: flex; font-family: 'Helvetica', 'Arial', sans-serif'; height: 100vh; padding: 1rem;" class="">
    <Sidebar items={sidebarItems} />
    <MainContent>
        <div class="button-group">
            <Button label="Ajouter un CFC" onClick={() => isModalOpen = true} />
            <div>
            <Button label="Projets" onClick={() => activeTab = 'projects'} />
            <Button label="Accueil" onClick={() => activeTab = 'home'} />
            <Button label="Sélection" onClick={() => activeTab = 'selection'} />
            </div>
        </div>

        <div class="input-group">
            <Select
                options={[
                    { value: 'Tous les OUV', label: 'Tous les OUV' },
                    { value: 'Contrôle des coûts', label: 'Contrôle des coûts' }
                ]}
                onSelect={(e) => { activeTab = e.value;
                }}
            />
            <div>
            <Button label="📄" onClick={openLink}/>
            <Button label="💰" onClick={openLink}/>
            <Button label="📅" onClick={openLink}/>
            <Button label="🖨️" onClick={openLink}/>
            <Button label="⬇️" onClick={openLink}/>
            <Button label="⬆️" onClick={openLink}/>
            </div>
        </div>

        <div class="table-group" >

            <Table
                headers={[
                    'CFC',
                    'Désignation',
                    'Devis original',
                    'Mutations',
                    'Devis révisé',
                    'Contrat',
                    'Prévision',
                    'Paiement',
                    'Coût probable',
                    'Dév.rév.-C.prob.',
                    'Coût réel',
                    'Dév.rév.-C.réel',
                ]}
                rows={projectData}
                onEditCell={handleEditCell}
            />
        </div>
    </MainContent>
  
    <Modal {isModalOpen} onClose={() => isModalOpen = false}>
        <h2>Ajouter un nouveau CFC</h2>
        <Input
            type="text"
            placeholder="CFC"
            bind:value={newCFC.id}
            onInput={(e) => newCFC.id = e.target.value}

        />
        <Input
            type="text"
            placeholder="Désignation"
            bind:value={newCFC.designation}
            onInput={(e) => newCFC.designation = e.target.value}

        />
        <Input
            type="number"
            placeholder="Devis original"
            bind:value={newCFC.original}
            onInput={(e) => newCFC.original = e.target.value}

        />
        <Input
            type="number"
            placeholder="Mutations"
            bind:value={newCFC.mutations}
            onInput={(e) => newCFC.mutations = e.target.value}

        />

      <Button label="Ajouter" onClick={handleAddCFC} />
      <!-- <button on:click={handleAddCFC} style={buttonStyle}>Ajouter</button> -->
    </Modal>

  </div>
  <style>
    /* .container {
      display: flex;
      font-family: 'Arial, sans-serif';
      height: 100vh;
    } */
    .button-group {
      display: flex;
      justify-content: space-between;
      align-items: center;
      margin-bottom: 20px;
    }
    .input-group {
      display: flex;
      justify-content: space-between;
      align-items: center;
      margin-bottom: 20px;
    }
    .table-group {
        width: 100%;
        overflow-x: scroll;
    }
  </style>