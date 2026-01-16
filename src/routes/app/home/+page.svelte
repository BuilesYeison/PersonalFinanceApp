<!-- Home Page -->

<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { toaster } from "../../../lib/toaster";
  import { AppErrors } from "../../../domain/enums/errors.enum";
  import type { DashboardOverallStatsDto } from "../../../domain/dto/dashboard_overall_stats.dto";
  import { workspace } from "$lib/state/workspace.svelte";
  import { onMount } from "svelte";

  const locale = $derived(navigator.language);
  console.log(locale);

  const formatter = $derived(
    new Intl.NumberFormat(locale, {
      style: "currency",
      currency: workspace.currency,
      minimumFractionDigits: 2,
      maximumFractionDigits: 2,
    })
  );
  let dashboardOverallStats: DashboardOverallStatsDto = $state({
    total_balance: 0,
    total_expense: 0,
    total_income: 0,
  });

  onMount(async () => {
    dashboardOverallStats = await getDashboardOverallStats();
  });

  async function getDashboardOverallStats(): Promise<DashboardOverallStatsDto> {
    try {
      const result: DashboardOverallStatsDto = await invoke(
        "get_overall_stats",
        {
          workspaceName: workspace.name,
        }
      );

      if (!result) {
        throw Error("Couldn't get overall stats");
      }
      return result;
    } catch (error: any) {
      console.error(error);
      if (error?.type == AppErrors.DatabaseError) {
        toaster.error({
          title: "Error",
          description: `Ha ocurrido un error obteniendo la información de la base de datos`,
        });
      } else if (error?.type == AppErrors.ConfigError) {
        toaster.error({
          title: "Error",
          description: `Error al crear archivos de configuración, por favor comuniquese con un desarrollador`,
        });
      } else if (error?.type == AppErrors.IoError) {
        toaster.error({
          title: "Error",
          description: `Error al gestionar archivos`,
        });
      } else {
        toaster.error({
          title: "Error",
          description: `Ocurrió un error no controlado, por favor comuniquese con un desarrollador.`,
        });
      }
      return {} as DashboardOverallStatsDto;
    }
  }
</script>

<main class="w-full h-auto">
  <section id="summary-cards-section" class="grid grid-cols-4 gap-4">
    <div
      class="card w-fit max-w-md preset-filled-surface-100-900 p-4 text-center"
    >
      <h3>Balance:</h3>
      <p class="font-semibold w-max">
        {formatter.format(dashboardOverallStats.total_balance)}
      </p>
    </div>
    <div
      class="card w-fit max-w-md preset-filled-surface-100-900 p-4 text-center"
    >
      <h3>Ingresos:</h3>
      <p class="font-semibold w-max">
        {formatter.format(dashboardOverallStats.total_income)}
      </p>
    </div>
    <div
      class="card w-fit max-w-md preset-filled-surface-100-900 p-4 text-center"
    >
      <h3>Gastos:</h3>
      <p class="font-semibold w-max">
        {formatter.format(dashboardOverallStats.total_expense)}
      </p>
    </div>
  </section>
</main>

<style lang="postcss">
  @reference "tailwindcss";
</style>
