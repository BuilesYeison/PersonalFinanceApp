<!-- Home Page -->

<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import type { DashboardOverallStatsDto } from "../../../domain/dto/dashboard_overall_stats.dto";
  import { onMount, onDestroy } from "svelte";
  import { useCurrencyFormatter } from "$lib/utils/formatter.svelte";
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
  import SummaryCards from "./components/summary-cards.svelte";
  import AccountsCardsManagement from "./components/accounts-cards-management.svelte";
  import { Dialog, Portal } from "@skeletonlabs/skeleton-svelte";
  import { goto } from "$app/navigation";
  import { Plus, ArrowUpRight, ArrowDownLeft, ArrowLeftRight } from "lucide-svelte";

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

  let lastRecords: PaginationDto<RecordDto> = $state({
    items: [],
    total_items: 0,
    total_pages: 0,
    current_page: 0,
    size: 0,
  });

  let isFabMenuOpen = $state(false);

  function openNewTransaction(type: "expense" | "income" | "transfer") {
    isFabMenuOpen = false;
    goto(`/app/home/new-transaction?type=${type}`);
  }

  onMount(async () => {
    await getDashboardOverallStats();
    await getRecords();
  });

  async function getRecords(): Promise<void> {
    try {
      const result: PaginationDto<RecordDto> = await invoke(
        "get_paginated_records",
        { page: 1, size: 30 },
      );
      lastRecords = result;
    } catch (error: any) {
      handleCommandError(error);
    }
  }

  async function getDashboardOverallStats(): Promise<void> {
    try {
      const result: DashboardOverallStatsDto = await invoke(
        "get_overall_stats",
        {},
      );

      console.log("llamando stats")

      if (!result) {
        throw Error("Couldn't get overall stats");
      }
      dashboardOverallStats = {...result};
    } catch (error: any) {
      handleCommandError(error);
    }
  }
</script>

<main class="w-full min-h-screen flex justify-center items-center px-4 py-2">
  <div class="w-full max-w-7xl">
    <SummaryCards {dashboardOverallStats}></SummaryCards>
    <AccountsCardsManagement
      accountsHaveChanged={() => getDashboardOverallStats()}
    ></AccountsCardsManagement>
    <div class="details-container mt-2 grid grid-cols-1 md:grid-cols-2 gap-4 w-full">
      <section class="expenses-chart-section w-full">
        <h4>Estructura de gastos:</h4>
        <div class="border-1 border-dashed flex flex-col justify-center h-full p-4">
          <p class="text-center">
            Total gastos ultimos 30 días: <br />{formatter.format(
              dashboardOverallStats.total_expense,
            )}
          </p>
          <div class="canvas w-full max-w-xs h-auto self-center mt-4">
            <canvas bind:this={canvas}></canvas>
          </div>
        </div>
      </section>
      <section class="records-section">
        <h4>Registros:</h4>
        <div
          class="border-1 border-dashed flex flex-col items-center h-full relative overflow-hidden p-2"
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

    <Dialog>
      <Dialog.Trigger class="fixed bottom-6 right-6 btn-icon preset-filled-primary-500 shadow-xl p-4 z-50">
        <Plus size={24} />
      </Dialog.Trigger>
      <Portal>
        <Dialog.Backdrop class="fixed inset-0 z-50 bg-surface-50-950/50" />
        <Dialog.Positioner class="fixed inset-0 z-50 flex justify-end items-end p-4">
          <Dialog.Content class="card bg-surface-100-900 p-2 space-y-2 shadow-xl w-48">
            <button
              class="btn preset-tonal w-full justify-start"
              onclick={() => openNewTransaction("expense")}
            >
              <ArrowDownLeft size={18} />
              Gasto
            </button>
            <button
              class="btn preset-tonal w-full justify-start"
              onclick={() => openNewTransaction("income")}
            >
              <ArrowUpRight size={18} />
              Ingreso
            </button>
            <button
              class="btn preset-tonal w-full justify-start"
              onclick={() => openNewTransaction("transfer")}
            >
              <ArrowLeftRight size={18} />
              Transferencia
            </button>
          </Dialog.Content>
        </Dialog.Positioner>
      </Portal>
    </Dialog>
  </div>
</main>

<style lang="postcss">
  @reference "tailwindcss";
</style>
