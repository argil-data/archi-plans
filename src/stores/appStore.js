
import { writable } from 'svelte/store';
import { invoke } from '@tauri-apps/api/tauri';

function createAppStore() {
    const { subscribe, set, update } = writable({
        projects: [],
        tasks: [],
        team: [],
        loading: false,
        error: null
    });

    return {
        subscribe,
        fetchAll: async () => {
            update(state => ({ ...state, loading: true, error: null }));
            try {
                const [projects, tasks, team] = await Promise.all([
                    invoke('fetch_projects'),
                    invoke('fetch_tasks'),
                    invoke('fetch_team')
                ]);
                set({ projects, tasks, team, loading: false, error: null });
            } catch (error) {
                console.error('Erreur lors de la récupération des données:', error);
                update(state => ({ ...state, loading: false, error: "Une erreur s'est produite lors du chargement des données." }));
            }
        },
        // Vous pouvez ajouter d'autres méthodes ici si nécessaire
    };
}

export const appStore = createAppStore();