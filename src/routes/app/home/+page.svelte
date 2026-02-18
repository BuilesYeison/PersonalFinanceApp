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

<main class="w-full h-auto">
  <SummaryCards {dashboardOverallStats}></SummaryCards>
  <AccountsCardsManagement
    accountsHaveChanged={() => getDashboardOverallStats()}
  ></AccountsCardsManagement>
  <div class="details-container mt-2 grid grid-cols-2 gap-4 w-full h-full">
    <section class="expenses-chart-section w-full">
      <h4>Estructura de gastos:</h4>
      <div class="border-1 border-dashed flex flex-col justify-center h-full">
        <p>
          Total gastos ultimos 30 días: <br />{formatter.format(
            dashboardOverallStats.total_expense,
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
