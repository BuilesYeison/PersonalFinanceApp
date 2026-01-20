import { workspace } from "$lib/state/workspace.svelte";

export function useCurrencyFormatter(locale?: string) {
  const resolvedLocale = locale ?? navigator.language;

  const formatter = $derived(
    new Intl.NumberFormat(resolvedLocale, {
      style: "currency",
      currency: workspace.currency,
      minimumFractionDigits: 2,
      maximumFractionDigits: 2,
    })
  );

  return formatter;
}
