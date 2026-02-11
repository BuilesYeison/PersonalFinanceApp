<!-- Home Page -->

<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { toaster } from "../../../lib/toaster";
  import type { DashboardOverallStatsDto } from "../../../domain/dto/dashboard_overall_stats.dto";
  import { onMount, onDestroy } from "svelte";
  import { useCurrencyFormatter } from "$lib/utils/formatter.svelte";
  import { XIcon, TrashIcon, SquarePenIcon } from "lucide-svelte";
  import { Dialog, Portal } from "@skeletonlabs/skeleton-svelte";
  import type { AccountInfoDto } from "../../../domain/dto/account_info.dto";
  import { workspace } from "$lib/state/workspace.svelte";
  import { handleCommandError } from "$lib/utils/error.handler";
  import {
    Chart,
    DoughnutController,
    ArcElement,
    Tooltip,
    Legend,
  } from "chart.js";
  import type { RecordDto } from "../../../domain/dto/record.dto";
  import type { PaginationDto } from "../../../domain/dto/pagination.dto";

  Chart.register(DoughnutController, ArcElement, Tooltip, Legend);

  let canvas: HTMLCanvasElement;
  let chart: Chart | null = null;

  let labels: string[] = ["Red", "Blue", "Yellow"];
  let values: number[] = [300, 50, 100];

  function renderChart() {
    chart?.destroy();

    chart = new Chart(canvas, {
      type: "doughnut",
      data: {
        labels,
        datasets: [
          {
            label: "My First Dataset",
            data: values,
            backgroundColor: [
              "rgb(255, 99, 132)",
              "rgb(54, 162, 235)",
              "rgb(255, 205, 86)",
            ],
            hoverOffset: 4,
          },
        ],
      },
      options: {
        animation: false,
        responsive: true,
        plugins: {
          legend: {
            position: "bottom",
          },
        },
      },
    });
  }

  onMount(renderChart);

  $effect(() => {
    if (labels.length && values.length) {
      renderChart();
    }
  });

  onDestroy(() => {
    chart?.destroy();
  });

  const formatter = useCurrencyFormatter();

  let dashboardOverallStats: DashboardOverallStatsDto = $state({
    total_balance: 0,
    total_expense: 0,
    total_income: 0,
  });

  let accounts: AccountInfoDto[] = $state([]);
  let lastRecords : PaginationDto<RecordDto> = $state({
    items:[],
    total_items:0,
    total_pages:0,
    current_page:0, 
    size:0,   
  })

  let isDialogOpen = $state(false);
  let newAccount: AccountInfoDto = $state({
    id: "",
    name: "",
    balance: 0,
    account_type: null,
    currency: workspace.currency,
    initial_balance: null,
    credit_limit: null,
  });

  onMount(async () => {
    await getDashboardOverallStats();
    await getAccounts();
    await getRecords();
  });

  async function getRecords(): Promise<void> {
    try {
      const result: PaginationDto<RecordDto> = await invoke(
        "get_paginated_records",
        { page: 1, size: 30 }
      );
      lastRecords = result
    } catch (error: any) {
      handleCommandError(error);
    }
  }

  async function getAccounts(): Promise<void> {
    try {
      const result: AccountInfoDto[] = await invoke("get_accounts", {});

      if (!result) {
        throw Error("Couldn't get accounts");
      }
      accounts = result;
    } catch (error: any) {
      handleCommandError(error);
    }
  }

  async function getDashboardOverallStats(): Promise<void> {
    try {
      const result: DashboardOverallStatsDto = await invoke(
        "get_overall_stats",
        {}
      );

      if (!result) {
        throw Error("Couldn't get overall stats");
      }
      dashboardOverallStats = result;
    } catch (error: any) {
      handleCommandError(error);
    }
  }

  async function handleCreateAccount() {
    try {
      newAccount.balance = newAccount.initial_balance ?? 0;
      await invoke("create_account", { newAccount });
      accounts.push(newAccount);
      isDialogOpen = false;
      newAccount = {
        id: "",
        name: "",
        balance: 0,
        account_type: null,
        currency: workspace.currency,
        initial_balance: null,
        credit_limit: null,
      };

      await getDashboardOverallStats();
      toaster.success({
        title: "Exito",
        description: "Cuenta creada exitosamente",
      });
    } catch (error: any) {
      handleCommandError(error);
    }
  }

  async function handleDeleteAccount(accountId: string) {
    try {
      let result = await invoke("delete_account", { accountId });
      if (result) {
        await Promise.all([getAccounts(), getDashboardOverallStats()]);
        toaster.success({
          title: "Exito",
          description: "Cuenta eliminada exitosamente",
        });
      } else {
        toaster.error({
          title: "Error",
          description:
            "No se pudo eliminar la cuenta porque tiene registros asociados.",
        });
      }
    } catch (error: any) {
      handleCommandError(error);
    }
  }
</script>

<main class="w-full h-auto">
  <section id="summary-cards-section" class="grid grid-cols-4 gap-4">
    <div class="card w-full max-w-md preset-filled-surface-100-900 p-4">
      <h3>Balance:</h3>
      <p class="font-semibold w-max">
        {formatter.format(dashboardOverallStats.total_balance)}
      </p>
    </div>
    <div class="card w-full max-w-md preset-filled-surface-100-900 p-4">
      <h3>Ingresos:</h3>
      <p class="font-semibold w-max">
        {formatter.format(dashboardOverallStats.total_income)}
      </p>
    </div>
    <div class="card w-full max-w-md preset-filled-surface-100-900 p-4">
      <h3>Gastos:</h3>
      <p class="font-semibold w-max">
        {formatter.format(dashboardOverallStats.total_expense)}
      </p>
    </div>
  </section>
  <section id="accounts-section" class="mt-2">
    <h4 class="">Cuentas:</h4>
    <div class="border-1 border-dashed w-full grid grid-cols-4 gap-3 p-2">
      {#each accounts as account}
        <div class="card p-4 preset-filled-success-500 relative group">
          <h5>{account.name}</h5>
          <h5 class="font-semibold w-max">
            {formatter.format(account.balance)}
          </h5>
          <Dialog>
            <Dialog.Trigger
              class="btn absolute top-0 right-0 opacity-0 group-hover:opacity-100 transition-opacity"
              ><TrashIcon size={17} /></Dialog.Trigger
            >
            <Portal>
              <Dialog.Backdrop
                class="fixed inset-0 z-50 bg-surface-50-950/50"
              />
              <Dialog.Positioner
                class="fixed inset-0 z-50 flex justify-center items-center p-4"
              >
                <Dialog.Content
                  class="card bg-surface-100-900 w-full max-w-xl p-4 space-y-4 shadow-xl"
                >
                  <header class="flex justify-between items-center">
                    <Dialog.Title class="text-lg font-bold"
                      >¿Estás seguro que deseas eliminar esta cuenta?</Dialog.Title
                    >
                    <Dialog.CloseTrigger class="btn-icon hover:preset-tonal">
                      <XIcon class="size-4" />
                    </Dialog.CloseTrigger>
                  </header>
                  <div>
                    Solo se eliminará si no tiene movimientos registrados, de lo
                    contrario solo se desactivará la cuenta.
                  </div>
                  <footer class="flex justify-end gap-2">
                    <Dialog.CloseTrigger class="btn preset-tonal"
                      >Cancelar</Dialog.CloseTrigger
                    >
                    <Dialog.CloseTrigger
                      ><button
                        type="button"
                        class="btn preset-tonal-error"
                        onclick={() => handleDeleteAccount(account.id)}
                        >Eliminar</button
                      ></Dialog.CloseTrigger
                    >
                  </footer>
                </Dialog.Content>
              </Dialog.Positioner>
            </Portal>
          </Dialog>
          <Dialog>
            <Dialog.Trigger
              class="btn absolute top-0 right-7 opacity-0 group-hover:opacity-100 transition-opacity"
              ><SquarePenIcon size={17} /></Dialog.Trigger
            >
            <Portal>
              <Dialog.Backdrop
                class="fixed inset-0 z-50 bg-surface-50-950/50"
              />
              <Dialog.Positioner
                class="fixed inset-0 z-50 flex justify-center items-center p-4"
              >
                <Dialog.Content
                  class="card bg-surface-100-900 w-full max-w-xl p-4 space-y-4 shadow-xl"
                >
                  <header class="flex justify-between items-center">
                    <Dialog.Title class="text-lg font-bold"
                      >Editar cuenta</Dialog.Title
                    >
                    <Dialog.CloseTrigger class="btn-icon hover:preset-tonal">
                      <XIcon class="size-4" />
                    </Dialog.CloseTrigger>
                  </header>
                  <div>
                    Solo se eliminará si no tiene movimientos registrados, de lo
                    contrario solo se desactivará la cuenta.
                  </div>
                  <footer class="flex justify-end gap-2">
                    <Dialog.CloseTrigger class="btn preset-tonal"
                      >Cancelar</Dialog.CloseTrigger
                    >
                    <Dialog.CloseTrigger
                      ><button type="button" class="btn preset-filled"
                        >Guardar</button
                      ></Dialog.CloseTrigger
                    >
                  </footer>
                </Dialog.Content>
              </Dialog.Positioner>
            </Portal>
          </Dialog>
        </div>
      {/each}
      <Dialog>
        <Dialog.Trigger class="btn preset-outlined-surface-500"
          >+ Añadir cuenta</Dialog.Trigger
        >
        <Portal>
          <Dialog.Backdrop class="fixed inset-0 z-50 bg-surface-50-950/50" />
          <Dialog.Positioner
            class="fixed inset-0 z-50 flex justify-center items-center p-4"
          >
            <Dialog.Content
              class="card bg-surface-100-900 w-full max-w-xl p-4 space-y-4 shadow-xl"
            >
              <header class="flex justify-between items-center">
                <Dialog.Title class="text-lg font-bold"
                  >Añadir nueva cuenta</Dialog.Title
                >
                <Dialog.CloseTrigger class="btn-icon hover:preset-tonal">
                  <XIcon class="size-4" />
                </Dialog.CloseTrigger>
              </header>
              <div class="flex flex-col gap-4">
                <label class="label">
                  <span>Nombre de la cuenta</span>
                  <input
                    class="input"
                    type="text"
                    placeholder="Ej: Billetera"
                    bind:value={newAccount.name}
                  />
                </label>

                <label class="label">
                  <span>Tipo de cuenta</span>
                  <select class="select" bind:value={newAccount.account_type}>
                    <option value="cash">Efectivo</option>
                    <option value="debit">Débito</option>
                    <option value="credit">Crédito</option>
                  </select>
                </label>

                <label class="label">
                  <span>Balance inicial</span>
                  <input
                    class="input"
                    type="number"
                    placeholder="0.00"
                    bind:value={newAccount.initial_balance}
                  />
                </label>
                {#if newAccount.account_type === "credit"}
                  <label class="label">
                    <span>Límite de crédito</span>
                    <input
                      class="input"
                      type="number"
                      placeholder="0.00"
                      bind:value={newAccount.credit_limit}
                    />
                  </label>
                {/if}
              </div>
              <footer class="flex justify-end gap-2">
                <Dialog.CloseTrigger class="btn preset-tonal"
                  >Cancelar</Dialog.CloseTrigger
                >
                <Dialog.CloseTrigger
                  ><button
                    type="button"
                    class="btn preset-filled"
                    onclick={handleCreateAccount}>Guardar</button
                  ></Dialog.CloseTrigger
                >
              </footer>
            </Dialog.Content>
          </Dialog.Positioner>
        </Portal>
      </Dialog>
    </div>
  </section>
  <div class="details-container mt-2 grid grid-cols-2 gap-4 w-full h-full">
    <section class="expenses-chart-section w-full">
      <h4>Estructura de gastos:</h4>
      <div class="border-1 border-dashed flex flex-col justify-center h-full">
        <p>
          Total gastos ultimos 30 días: <br />{formatter.format(
            dashboardOverallStats.total_expense
          )}
        </p>
        <div class="canvas w-70 h-auto self-center">
          <canvas bind:this={canvas}></canvas>
        </div>
      </div>
    </section>
    <section class="records-section">
      <h4>Registros:</h4>
      <div
        class="border-1 border-dashed flex flex-col items-center h-full relative overflow-hidden"
      >
      {#each lastRecords.items as record}
      <div
          class="card rounded-xl w-full max-w-md preset-filled-surface-100-900 py-2 px-7 m-2 relative"
        >
          <p>{record.description}</p>
          <p class="text-xs">{record.account?.name}</p>
          <p class="font-semibold absolute top-4 right-5">
            - {formatter.format(record.amount)}
          </p>
        </div>
      {/each}
        
        <button
          class="btn preset-outlined-surface-500 absolute bottom-2 right-2"
          >+ ver más...</button
        >
      </div>
    </section>
  </div>
</main>

<style lang="postcss">
  @reference "tailwindcss";
</style>
