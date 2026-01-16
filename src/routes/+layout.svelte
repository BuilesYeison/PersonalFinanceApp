<script lang="ts">
  import "../app.css";
  import { Sun, Moon } from "lucide-svelte";
  import { Toast } from "@skeletonlabs/skeleton-svelte";
  import { onMount } from "svelte";
  import { toaster } from "../lib/toaster";
  import { invoke } from "@tauri-apps/api/core";
  import { workspace } from "$lib/state/workspace.svelte";
  import { goto } from "$app/navigation";
  import type { WorkspaceContextDto } from "../domain/dto/workspace_context.dto";

  let { children } = $props();

  let isDark = $state(false);

  onMount(async () => {
    isDark = document.documentElement.classList.contains("dark");
    try {
      const context: WorkspaceContextDto = await invoke(
        "get_workspace_context"
      );
      workspace.setContext(context as any);
      // Si todo carga bien, saltamos al home
      if (window.location.pathname === "/") {
        goto("/app/home");
      }
    } catch (e) {
      console.error(e);
      // Si falla, es que es la primera vez o no hay sesión,
      // nos quedamos en la landing.
    }
  });

  function toggleDarkMode() {
    isDark = !isDark;
    document.documentElement.classList.toggle("dark", isDark);
  }
</script>

<div class="fixed top-4 right-4 z-50">
  <button
    type="button"
    class="btn-icon preset-filled-surface-500 shadow-xl"
    onclick={toggleDarkMode}
    title="Cambiar modo"
  >
    {#if isDark}
      <Sun size={20} />
    {:else}
      <Moon size={20} />
    {/if}
  </button>
</div>

{@render children()}

<Toast.Group {toaster}>
  {#snippet children(toast)}
    <Toast {toast}>
      <Toast.Message>
        <Toast.Title>{toast.title}</Toast.Title>
        <Toast.Description>{toast.description}</Toast.Description>
      </Toast.Message>
      <Toast.CloseTrigger />
    </Toast>
  {/snippet}
</Toast.Group>
