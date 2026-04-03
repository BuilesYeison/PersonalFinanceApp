<script lang="ts">
  import { useCurrencyFormatter } from "$lib/utils/formatter.svelte";
  import type { DashboardOverallStatsDto } from "../../../../domain/dto/dashboard_overall_stats.dto";

  let { dashboardOverallStats }: { dashboardOverallStats: DashboardOverallStatsDto } = $props();
  const formatter = useCurrencyFormatter();
</script>

<section id="summary-cards-section" class="grid grid-cols-1 sm:grid-cols-3 gap-4">
  <div class="card w-full preset-filled-surface-100-900 p-4 min-w-0">
    <h3>Balance:</h3>
    <p class="font-semibold break-all">
      {formatter.format(dashboardOverallStats.total_balance)}
    </p>
  </div>
  <div class="card w-full preset-filled-surface-100-900 p-4 min-w-0">
    <h3>Ingresos:</h3>
    <p class="font-semibold break-all">
      {formatter.format(dashboardOverallStats.total_income)}
    </p>
  </div>
  <div class="card w-full preset-filled-surface-100-900 p-4 min-w-0">
    <h3>Gastos:</h3>
    <p class="font-semibold break-all">
      {formatter.format(dashboardOverallStats.total_expense)}
    </p>
  </div>
</section>
