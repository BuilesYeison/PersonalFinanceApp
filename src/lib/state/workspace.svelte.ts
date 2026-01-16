// Usamos un objeto reactivo con $state
class WorkspaceState {
    name = $state("");
    path = $state("");
    currency = $state("");
    theme = $state("");
    isLoaded = $state(false);

    setContext(data: { name: string; path: string; currency: string; theme: string }) {
        this.name = data.name;
        this.path = data.path;
        this.currency = data.currency;
        this.theme = data.theme;
        this.isLoaded = true;
    }
}

// Exportamos una única instancia (Singleton)
export const workspace = new WorkspaceState();