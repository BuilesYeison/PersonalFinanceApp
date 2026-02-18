<script lang="ts">
  import { useCurrencyFormatter } from "$lib/utils/formatter.svelte";
  import type { DashboardOverallStatsDto } from "../../../../domain/dto/dashboard_overall_stats.dto";

  let { dashboardOverallStats }: { dashboardOverallStats: DashboardOverallStatsDto } = $props();
  const formatter = useCurrencyFormatter();
</script>

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
