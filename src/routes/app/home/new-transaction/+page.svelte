<script lang="ts">
  import { goto } from "$app/navigation";
  import { page } from "$app/stores";
  import { ArrowLeft, Plus, Minus, X, Divide, Delete, Equal } from "lucide-svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { handleCommandError } from "$lib/utils/error.handler";
  import { toaster } from "$lib/toaster";
  import { useCurrencyFormatter } from "$lib/utils/formatter.svelte";
  import { DatePicker, parseDate, Portal } from "@skeletonlabs/skeleton-svelte";
  import type { AccountInfoDto } from "../../../../domain/dto/account_info.dto";
  import type { CategoryDto } from "../../../../domain/dto/category.dto";
  import type { CreateRecordDto } from "../../../../domain/dto/create_record.dto";

  const formatter = useCurrencyFormatter();

  let transactionType = $state<"expense" | "income" | "transfer">(
    ($page.url.searchParams.get("type") as "expense" | "income" | "transfer") || "expense"
  );
  let tabIndex = $state(transactionType === "expense" ? 0 : transactionType === "transfer" ? 1 : 2);

  let expression = $state("0");
  let displayValue = $derived(formatter.format(parseFloat(expression) || 0));

  let description = $state("");
  let selectedDate = $state([parseDate(new Date().toISOString().split("T")[0])!]);
  let selectedAccountId = $state("");
  let selectedCategoryId = $state("");
  let selectedToAccountId = $state("");

  let accounts = $state<AccountInfoDto[]>([]);
  let categories = $state<CategoryDto[]>([]);

  let filteredCategories = $derived(
    categories.filter((c) => {
      if (transactionType === "expense") return c.type === "expense";
      if (transactionType === "income") return c.type === "income";
      return true;
    })
  );

  async function loadAccounts() {
    try {
      const result: AccountInfoDto[] = await invoke("get_accounts", {});
      accounts = result;
      if (result.length > 0 && !selectedAccountId) {
        selectedAccountId = result[0].id;
      }
    } catch (error: any) {
      handleCommandError(error);
    }
  }

  async function loadCategories() {
    try {
      const result: CategoryDto[] = await invoke("get_categories", {});
      categories = result;
    } catch (error: any) {
      handleCommandError(error);
    }
  }

  $effect(() => {
    loadAccounts();
    loadCategories();
  });

  function handleTypeChange(index: number) {
    tabIndex = index;
    transactionType = index === 0 ? "expense" : index === 1 ? "transfer" : "income";
  }

  function appendDigit(digit: string) {
    if (expression === "0" && digit !== ".") {
      expression = digit;
    } else {
      expression += digit;
    }
  }

  function appendOperator(op: string) {
    const lastChar = expression.slice(-1);
    if (["+", "-", "*", "/"].includes(lastChar)) {
      expression = expression.slice(0, -1) + op;
    } else {
      expression += op;
    }
  }

  function clearExpression() {
    expression = "0";
  }

  function backspace() {
    if (expression.length === 1 || expression === "-0") {
      expression = "0";
    } else {
      expression = expression.slice(0, -1);
    }
  }

  function toggleSign() {
    if (expression.startsWith("-")) {
      expression = expression.slice(1);
    } else {
      expression = "-" + expression;
    }
  }

  function evaluate() {
    try {
      const result = Function(`"use strict"; return (${expression})`)();
      expression = String(result);
    } catch {
      expression = "Error";
      setTimeout(() => (expression = "0"), 1000);
    }
  }

  function handleDateChange(e: any) {
    if (e.value && e.value.length > 0) {
      selectedDate = e.value;
    }
  }

  async function handleSave() {
    if (!selectedAccountId) {
      toaster.error({ title: "Error", description: "Selecciona una cuenta" });
      return;
    }

    if (transactionType === "transfer" && !selectedToAccountId) {
      toaster.error({ title: "Error", description: "Selecciona una cuenta destino" });
      return;
    }

    const amount = parseFloat(expression);
    if (isNaN(amount) || amount <= 0) {
      toaster.error({ title: "Error", description: "Ingresa un monto válido" });
      return;
    }

    const dateValue = selectedDate[0];
    const timestamp = dateValue && "toDate" in dateValue ? dateValue.toDate("UTC").getTime() : Date.now();

    const record: CreateRecordDto = {
      type: transactionType,
      amount,
      account_id: selectedAccountId,
      to_account_id: transactionType === "transfer" ? selectedToAccountId : undefined,
      category_id: selectedCategoryId || undefined,
      description: description || undefined,
      timestamp,
    };

    try {
      await invoke("create_record", { record });
      toaster.success({ title: "Éxito", description: "Registro creado exitosamente" });
      goto("/app/home");
    } catch (error: any) {
      handleCommandError(error);
    }
  }

  const calculatorButtons = [
    ["7", "8", "9", "/"],
    ["4", "5", "6", "*"],
    ["1", "2", "3", "-"],
    ["0", ".", "=", "+"],
  ];
</script>

<main class="w-full min-h-screen flex justify-center px-4 py-4">
  <div class="w-full max-w-md">
    <div class="flex items-center gap-3 mb-4">
      <button class="btn-icon preset-tonal" onclick={() => goto("/app/home")}>
        <ArrowLeft size={20} />
      </button>
      <h3>Nueva transacción</h3>
    </div>

    <div class="card p-4 space-y-4">
      <div class="grid grid-cols-3 gap-2">
        <button
          class={tabIndex === 0 ? "btn preset-filled-primary-500" : "btn preset-outlined-surface-500"}
          onclick={() => handleTypeChange(0)}
        >
          Gasto
        </button>
        <button
          class={tabIndex === 1 ? "btn preset-filled-primary-500" : "btn preset-outlined-surface-500"}
          onclick={() => handleTypeChange(1)}
        >
          Transferencia
        </button>
        <button
          class={tabIndex === 2 ? "btn preset-filled-primary-500" : "btn preset-outlined-surface-500"}
          onclick={() => handleTypeChange(2)}
        >
          Ingreso
        </button>
      </div>

      <div class="text-right py-4 border-b border-surface-200-800 dark:border-surface-700-300">
        <p class="text-4xl font-semibold tabular-nums text-surface-900-50">{displayValue}</p>
      </div>

      <div class="space-y-2">
        <div class="grid grid-cols-3 gap-2">
          <button class="btn preset-tonal-error py-3 font-semibold" onclick={clearExpression}>C</button>
          <button class="btn preset-tonal py-3" onclick={backspace}>
            <Delete size={18} />
          </button>
          <button class="btn preset-tonal py-3" onclick={toggleSign}>
            +/-
          </button>
        </div>

        <div class="grid grid-cols-4 gap-2">
          {#each calculatorButtons.flat() as btn}
            {#if btn === "="}
              <button class="btn preset-filled-primary-500 py-3" onclick={evaluate}>
                <Equal size={20} />
              </button>
            {:else if ["+", "-", "*", "/"].includes(btn)}
              <button class="btn preset-tonal-primary-500 py-3" onclick={() => appendOperator(btn)}>
                {#if btn === "+"}<Plus size={20} />
                {:else if btn === "-"}<Minus size={20} />
                {:else if btn === "*"}<X size={20} />
                {:else}<Divide size={20} />
                {/if}
              </button>
            {:else}
              <button class="btn preset-outlined-surface-500 py-3 text-lg font-medium" onclick={() => appendDigit(btn)}>
                {btn}
              </button>
            {/if}
          {/each}
        </div>
      </div>

      <div class="space-y-3 pt-2">
        <label class="label">
          <span>Descripción</span>
          <input class="input" type="text" placeholder="Ej: Compra en supermercado" bind:value={description} />
        </label>

        <label class="label">
          <span>Fecha</span>
          <DatePicker value={selectedDate} onValueChange={handleDateChange}>
            <DatePicker.Control>
              <DatePicker.Input placeholder="mm/dd/yyyy" class="input" />
              <DatePicker.Trigger class="btn preset-tonal">
                <svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                  <rect width="18" height="18" x="3" y="4" rx="2" ry="2"></rect>
                  <line x1="16" x2="16" y1="2" y2="6"></line>
                  <line x1="8" x2="8" y1="2" y2="6"></line>
                  <line x1="3" x2="21" y1="10" y2="10"></line>
                </svg>
              </DatePicker.Trigger>
            </DatePicker.Control>
            <Portal>
              <DatePicker.Positioner>
                <DatePicker.Content class="card shadow-xl border border-surface-200-800 dark:border-surface-700-300">
                  <DatePicker.View view="day">
                    <DatePicker.Context>
                      {#snippet children(datePicker)}
                        <DatePicker.ViewControl class="mb-2">
                          <DatePicker.PrevTrigger class="btn-icon preset-tonal" />
                          <DatePicker.ViewTrigger class="btn preset-tonal">
                            <DatePicker.RangeText />
                          </DatePicker.ViewTrigger>
                          <DatePicker.NextTrigger class="btn-icon preset-tonal" />
                        </DatePicker.ViewControl>
                        <DatePicker.Table>
                          <DatePicker.TableHead>
                            <DatePicker.TableRow>
                              {#each datePicker().weekDays as weekDay, id (id)}
                                <DatePicker.TableHeader class="text-xs text-surface-600-400">{weekDay.short}</DatePicker.TableHeader>
                              {/each}
                            </DatePicker.TableRow>
                          </DatePicker.TableHead>
                          <DatePicker.TableBody>
                            {#each datePicker().weeks as week, id (id)}
                              <DatePicker.TableRow>
                                {#each week as day, id (id)}
                                  <DatePicker.TableCell value={day}>
                                    <DatePicker.TableCellTrigger class="w-8 h-8 text-sm rounded-full hover:preset-tonal">
                                      {day.day}
                                    </DatePicker.TableCellTrigger>
                                  </DatePicker.TableCell>
                                {/each}
                              </DatePicker.TableRow>
                            {/each}
                          </DatePicker.TableBody>
                        </DatePicker.Table>
                      {/snippet}
                    </DatePicker.Context>
                  </DatePicker.View>
                  <DatePicker.View view="month">
                    <DatePicker.Context>
                      {#snippet children(datePicker)}
                        <DatePicker.ViewControl class="mb-2">
                          <DatePicker.PrevTrigger class="btn-icon preset-tonal" />
                          <DatePicker.ViewTrigger class="btn preset-tonal">
                            <DatePicker.RangeText />
                          </DatePicker.ViewTrigger>
                          <DatePicker.NextTrigger class="btn-icon preset-tonal" />
                        </DatePicker.ViewControl>
                        <DatePicker.Table>
                          <DatePicker.TableBody>
                            {#each datePicker().getMonthsGrid({ columns: 4, format: "short" }) as months, id (id)}
                              <DatePicker.TableRow>
                                {#each months as month, id (id)}
                                  <DatePicker.TableCell value={month.value}>
                                    <DatePicker.TableCellTrigger class="btn preset-tonal text-sm py-2 w-full">
                                      {month.label}
                                    </DatePicker.TableCellTrigger>
                                  </DatePicker.TableCell>
                                {/each}
                              </DatePicker.TableRow>
                            {/each}
                          </DatePicker.TableBody>
                        </DatePicker.Table>
                      {/snippet}
                    </DatePicker.Context>
                  </DatePicker.View>
                  <DatePicker.View view="year">
                    <DatePicker.Context>
                      {#snippet children(datePicker)}
                        <DatePicker.ViewControl class="mb-2">
                          <DatePicker.PrevTrigger class="btn-icon preset-tonal" />
                          <DatePicker.ViewTrigger class="btn preset-tonal">
                            <DatePicker.RangeText />
                          </DatePicker.ViewTrigger>
                          <DatePicker.NextTrigger class="btn-icon preset-tonal" />
                        </DatePicker.ViewControl>
                        <DatePicker.Table>
                          <DatePicker.TableBody>
                            {#each datePicker().getYearsGrid({ columns: 4 }) as years, id (id)}
                              <DatePicker.TableRow>
                                {#each years as year, id (id)}
                                  <DatePicker.TableCell value={year.value}>
                                    <DatePicker.TableCellTrigger class="btn preset-tonal text-sm py-2 w-full">
                                      {year.label}
                                    </DatePicker.TableCellTrigger>
                                  </DatePicker.TableCell>
                                {/each}
                              </DatePicker.TableRow>
                            {/each}
                          </DatePicker.TableBody>
                        </DatePicker.Table>
                      {/snippet}
                    </DatePicker.Context>
                  </DatePicker.View>
                </DatePicker.Content>
              </DatePicker.Positioner>
            </Portal>
          </DatePicker>
        </label>

        <label class="label">
          <span>Cuenta</span>
          <select class="select" bind:value={selectedAccountId}>
            <option value="">Seleccionar cuenta</option>
            {#each accounts as account}
              <option value={account.id}>{account.name}</option>
            {/each}
          </select>
        </label>

        {#if transactionType !== "transfer"}
          <label class="label">
            <span>Categoría</span>
            <select class="select" bind:value={selectedCategoryId}>
              <option value="">Seleccionar categoría</option>
              {#each filteredCategories as category}
                <option value={category.id}>{category.name}</option>
              {/each}
            </select>
          </label>
        {/if}

        {#if transactionType === "transfer"}
          <label class="label">
            <span>Cuenta destino</span>
            <select class="select" bind:value={selectedToAccountId}>
              <option value="">Seleccionar cuenta destino</option>
              {#each accounts.filter((a) => a.id !== selectedAccountId) as account}
                <option value={account.id}>{account.name}</option>
              {/each}
            </select>
          </label>
        {/if}
      </div>

      <button class="btn preset-filled-primary-500 w-full mt-4" onclick={handleSave}>
        Guardar transacción
      </button>
    </div>
  </div>
</main>

<style lang="postcss">
  @reference "tailwindcss";
</style>
