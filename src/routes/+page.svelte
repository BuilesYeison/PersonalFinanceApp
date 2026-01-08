<!--Landing Page-->

<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { CircleDollarSign, ArrowLeft, FolderSearch } from "lucide-svelte";
  import { open } from "@tauri-apps/plugin-dialog";
  import { AppErrors } from "../domain/errors.enum";
  import { toaster } from "../lib/toaster";
  import { event } from "@tauri-apps/api";

  let view = $state("welcome"); //welcome, create or open

  // --- Estados del Formulario ---
  let projectName = $state("");
  let projectPath = $state("");

  async function selectFolder() {
    // Abre el diálogo nativo del Sistema Operativo
    const selected = await open({
      multiple: false,
      directory: true,
    });

    if (selected) projectPath = selected as string;
  }

  async function handleCreate(event: Event) {
    event.preventDefault();
    try {
      const result = await invoke("init_workspace", {
        basePath: projectPath,
        name: projectName,
      });

      toaster.success({
        title: "Bien",
        description: result,
      });
    } catch (error: any) {
      console.error(error);
      if (error?.type == AppErrors.WorkspaceExists) {
        toaster.error({
          title: "Error",
          description: `La carpeta financiera '${error.message}' ya existe`,
        });
      } else if (error?.type == AppErrors.ConfigError) {
        toaster.error({
          title: "Error",
          description: `Error al crear archivos de configuración, por favor comuniquese con un desarrollador`,
        });
      } else {
        toaster.error({
          title: "Error",
          description: `Ocurrió un error no controlado, por favor comuniquese con un desarrollador.`,
        });
      }
    }
  }

  async function handleOpen(event: Event) {
    console.log("Abriendo:", { projectName, projectPath });
    event.preventDefault();

    try {
      const result = await invoke("open_workspace", {
        fullPath: projectPath,
      });

      toaster.success({
        title: "Bien",
        description: result,
      });
    } catch (error: any) {
      console.error(error);
      if (error?.type == AppErrors.ConfigError) {
        toaster.error({
          title: "Error",
          description: `Error al crear archivos de configuración, por favor comuniquese con un desarrollador`,
        });
      } else {
        toaster.error({
          title: "Error",
          description: `Ocurrió un error no controlado, por favor comuniquese con un desarrollador.`,
        });
      }
    }
  }
</script>

<main
  class="flex flex-col items-center justify-center min-h-screen w-full p-4 text-center"
>
  <div class="mb-8 text-primary-500">
    <CircleDollarSign
      class="preset-tonal-surface"
      size={120}
      strokeWidth={1.5}
    />
  </div>

  {#if view === "welcome"}
    <div class="mb-5">
      <p>Crea una nueva carpeta financiera o abre una existente</p>
    </div>
    <div class="w-fit max-w-xs">
      <div class="welcome-set flex flex-col flex gap-4">
        <button
          type="button"
          class="btn preset-filled"
          onclick={() => (view = "create")}>Crear nuevo</button
        >
        <button
          type="button"
          class="btn preset-outlined-surface-500"
          onclick={() => (view = "open")}>Abrir existente</button
        >
      </div>
    </div>
    <div class="mt-15">
      <p class="italic">
        Tus datos se guardan localmente y tú decides cómo sincronizarlos
      </p>
    </div>
  {:else if view === "create"}
    <div class="w-fit max-w-md card">
      <div class="mb-4">
        <button
          class="anchor text-sm flex items-center justify-center gap-1 opacity-70 hover:opacity-100 transition-opacity"
          onclick={() => (view = "welcome")}
        >
          <ArrowLeft size={14} />
          Volver
        </button>
      </div>
      <h3 class="font-bold mb-4">Crear carpeta financiera</h3>

      <div class="flex flex-col space-y-3 text-left">
        <label class="label">
          <span class="ml-1">Nombre del proyecto</span>
          <input
            class="input"
            type="text"
            placeholder="Ej: Mi App Increíble"
            bind:value={projectName}
          />
        </label>

        <label class="label">
          <span class="ml-1">Ubicación</span>
          <div class="input-group grid-cols-[1fr_auto] overflow-hidden">
            <input
              class="bg-transparent input"
              type="text"
              readonly
              placeholder="Haz clic en la lupa..."
              bind:value={projectPath}
            />
            <button class="variant-filled-surface p-2" onclick={selectFolder}>
              <FolderSearch size={20} />
            </button>
          </div>
        </label>

        <button
          type="button"
          class="btn preset-filled"
          onclick={handleCreate}
          disabled={!projectName || !projectPath}
        >
          Crear
        </button>
      </div>
    </div>
  {:else if view === "open"}
    <div class="w-fit max-w-md card">
      <div class="mb-4">
        <button
          class="anchor text-sm flex items-center justify-center gap-1 opacity-70 hover:opacity-100 transition-opacity"
          onclick={() => (view = "welcome")}
        >
          <ArrowLeft size={14} />
          Volver
        </button>
      </div>
      <h3 class="font-bold mb-4">Abrir carpeta financiera</h3>

      <div class="flex flex-col space-y-3 text-left">
        <label class="label">
          <span class="ml-1">Ubicación</span>
          <div class="input-group grid-cols-[1fr_auto] overflow-hidden">
            <input
              class="bg-transparent input"
              type="text"
              readonly
              placeholder="Haz clic en la lupa..."
              bind:value={projectPath}
            />
            <button class="variant-filled-surface p-2" onclick={selectFolder}>
              <FolderSearch size={20} />
            </button>
          </div>
        </label>

        <button type="button" class="btn preset-filled" onclick={handleOpen}>
          Abrir
        </button>
      </div>
    </div>
  {:else}
    <p>oops</p>
  {/if}
</main>

<style lang="postcss">
  @reference "tailwindcss";
</style>
