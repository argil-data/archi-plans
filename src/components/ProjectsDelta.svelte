<script>
  import { onMount } from 'svelte';
  // import { Trash2, Edit, Plus, X, ChevronDown, ChevronRight } from 'lucide-svelte';
  // import { Dialog, DialogContent, DialogHeader, DialogTitle, DialogFooter } from "@/components/ui/dialog";
  // import { Button } from "@/components/ui/button";
  // import { Input } from "@/components/ui/input";

  import Dialog from './Dialog.svelte';
  import DialogContent from './DialogContent.svelte';
  import DialogHeader from './DialogHeader.svelte';
  import DialogTitle from './DialogTitle.svelte';
  import DialogFooter from './DialogFooter.svelte';
  import Button from './Button.svelte';
  import Input from './Input.svelte';
  import MenuItem from './MenuItem.svelte';

  let addresses = [
    { id: 1, name: 'Ingénieurs SA', type: 'Adresse', telephone: '026 663 01 01', mobile: '079 210 21 10', email: 'info@ingenieurs.ch' },
    { id: 2, name: 'Michel Bernard', type: 'Adresse', telephone: '026 663 01 01', mobile: '079 210 21 10', email: 'michel.bernard@ingenieurs.ch' },
  ];
  
  let selectedAddress = null;

  let isModalOpen = false;
  let editMode = false;
  let expandedMenus = {
    adresses: true,
    affaires: true,
  };

  // export let name;
  // export let type;
  // export let telephone;
  // export let mobile;
  // export let email;
  
  function handleNameInput(e) {
  // Handle input event
    selectedAddress = { ...selectedAddress, name: e.target.value };
  }

  function handleTypeInput(e) {
    // Handle input event
    selectedAddress = { ...selectedAddress, type: e.target.value };
  }

  function handleTelephoneInput(e) {
    // Handle input event
    selectedAddress = { ...selectedAddress, telephone: e.target.value };

  }

  function handleMobileInput(e) {
    // Handle input event
    selectedAddress = { ...selectedAddress, mobile: e.target.value };

  }

  function handleEmailInput(e) {
    // Handle input event
    selectedAddress = { ...selectedAddress, email: e.target.value };

  }

    function handleAddAddress() {
      selectedAddress = {};
      editMode = false;
      isModalOpen = true;
    }
  
    function handleEditAddress(address) {
      selectedAddress = address;
      editMode = true;
      isModalOpen = true;
    }
  
    function handleDeleteAddress(id) {
      addresses = addresses.filter(address => address.id !== id);
    }
  
    function handleSaveAddress() {
      if (editMode) {
        addresses = addresses.map(a => a.id === selectedAddress.id ? selectedAddress : a);
      } else {
        addresses = [...addresses, { ...selectedAddress, id: addresses.length + 1 }];
      }
      isModalOpen = false;
    }
  
    function toggleMenu(menu) {
      expandedMenus = { ...expandedMenus, [menu]: !expandedMenus[menu] };
    }
  
    // function MenuItem({ title, children, expanded, onToggle }) {
    //   return (
    //     <li class="mb-2">
    //       <div class="flex items-center cursor-pointer" on:click={onToggle}>
    //         {expanded ? <ChevronDown class="h-4 w-4 mr-1" /> : <ChevronRight class="h-4 w-4 mr-1" />}
    //         {title}
    //       </div>
    //       {expanded && children && (
    //         <ul class="ml-4 mt-1">
    //           {children.map((item, index) => (
    //             <li key={index} class="mb-1 text-sm">{item}</li>
    //           ))}
    //         </ul>
    //       )}
    //     </li>
    //   );
    // }
  </script>
  
  <div class="flex h-screen bg-gray-100">
    <aside class="w-64 bg-gray-800 text-white p-4">
      <h1 class="text-2xl font-bold mb-4">archiPLANS</h1>
      <nav>
        <ul>
          <MenuItem
            title="ADRESSES"
            expanded={expandedMenus.adresses}
            onToggle={() => toggleMenu('adresses')}
            children={['Liste des adresses', 'Mes favoris', 'Groupes d\'adresses', 'Propriétés', 'Chercher']}
          />
          <MenuItem
            title="AFFAIRES"
            expanded={expandedMenus.affaires}
            onToggle={() => toggleMenu('affaires')}
            children={['Gestion', 'Controlling', 'Mes affaires', 'Toutes les affaires']}
          />
          <li class="mb-2">COLLABORATEURS</li>
          <li class="mb-2">NOTES DE FRAIS</li>
          <li class="mb-2">HEURES</li>
          <li class="mb-2">TÂCHES</li>
          <li class="mb-2">FACTURES</li>
          <li class="mb-2">BÂTIMENT</li>
          <li class="mb-2">MANAGEMENT</li>
          <li class="mb-2">MODÈLES DOCUMENTS</li>
          <li class="mb-2">MODÈLES</li>
        </ul>
      </nav>
    </aside>
    <main class="flex-1 p-8 text-gray-800 dark:text-gray-100 dark:bg-gray-900">
      <h2 class="text-2xl font-bold mb-4">Adresses</h2>
      <Button label="Ajouter une adresse"  className="mb-4" onClick={handleAddAddress} />
        <!-- <Plus class="mr-2 h-4 w-4" />  -->
        <!-- Ajouter une adresse -->
      <table class="w-full ">
        <thead>
          <tr>
            <th class="text-left">Nom</th>
            <th class="text-left">Type</th>
            <th class="text-left">Téléphone</th>
            <th class="text-left">Mobile</th>
            <th class="text-left">Email</th>
            <th>Actions</th>
          </tr>
        </thead>
        <tbody>
          {#each addresses as address}
            <tr>
              <td>{address.name}</td>
              <td>{address.type}</td>
              <td>{address.telephone}</td>
              <td>{address.mobile}</td>
              <td>{address.email}</td>
              <td>
                <Button label="Edit" onClick={() => handleEditAddress(address)}>
                  <!-- <Edit class="h-4 w-4" /> -->
                   Edit
                </Button>
                <Button label="Delete" onClick={() => handleDeleteAddress(address.id)}>
                  <!-- <Trash2 class="h-4 w-4" /> --> 
                   Delete
                </Button>
              </td>
            </tr>
          {/each}
        </tbody>
      </table>
    </main>
    {#if isModalOpen}
      <Dialog open={isModalOpen} onOpenChange={(open) => isModalOpen = open}>
        <DialogContent>
          <DialogHeader>
            <DialogTitle>{editMode ? 'Modifier l\'adresse' : 'Ajouter une adresse'}</DialogTitle>
          </DialogHeader>
          <form on:submit|preventDefault={handleSaveAddress}>
            <Input
              placeholder="Nom"
              value={selectedAddress?.name}
              className="mb-2"
              onInput={handleNameInput}
            />
            <Input
              placeholder="Type"
              value={selectedAddress?.type}
              className="mb-2"
              onInput={handleTypeInput}
            />
            <Input
              placeholder="Téléphone"
              value={selectedAddress?.telephone}
              className="mb-2"
              onInput={handleTelephoneInput}
            />
            <Input
              placeholder="Mobile"
              value={selectedAddress?.mobile}
              className="mb-2"
              onInput={handleMobileInput}
            />
            <Input
              placeholder="Email"
              value={selectedAddress?.email}
              className="mb-2"
              onInput={handleEmailInput}
            />
            <DialogFooter>
              <Button className="" type="submit" onClick>{editMode ? 'Modifier' : 'Ajouter'}</Button>
            </DialogFooter>
          </form>
        </DialogContent>
      </Dialog>
    {/if}
  </div>
  
<style>    
      .flex {
    display: flex;
  }
  .h-screen {
    height: 100vh;
  }
  /* .bg-gray-100 {
    background-color: #f7fafc;
  } */
  .w-64 {
    width: 16rem;
  }
  .bg-gray-800 {
    background-color: #2d3748;
  }
  .text-white {
    color: #ffffff;
  }
  .p-4 {
    padding: 1rem;
  }
  .text-2xl {
    font-size: 1.5rem;
  }
  .font-bold {
    font-weight: bold;
  }
  .mb-4 {
    margin-bottom: 1rem;
  }
  .flex-1 {
    flex: 1;
  }
  .p-8 {
    padding: 2rem;
  }
  .mb-4 {
    margin-bottom: 1rem;
  }
  .w-full {
    width: 100%;
  }
  .text-left {
    text-align: left;
  }
  .mb-2 {
    margin-bottom: 0.5rem;
  }
  /* .ml-4 {
    margin-left: 1rem;
  } */
  /* .mt-1 {
    margin-top: 0.25rem;
  } */
  /* .mb-1 {
    margin-bottom: 0.25rem;
  } */
  /* .text-sm {
    font-size: 0.875rem;
  } */
  /* .h-4 {
    height: 1rem;
  } */
  /* .w-4 {
    width: 1rem;
  } */
  /* .mr-1 {
    margin-right: 0.25rem;
  } */
</style>
