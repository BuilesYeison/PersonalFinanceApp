<script lang="ts">
  import { XIcon, TrashIcon, SquarePenIcon } from "lucide-svelte";
  import { Dialog, Portal } from "@skeletonlabs/skeleton-svelte";
  import type { AccountInfoDto } from "../../../../domain/dto/account_info.dto";
  import { useCurrencyFormatter } from "$lib/utils/formatter.svelte";
  import { handleCommandError } from "$lib/utils/error.handler";
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  import { workspace } from "$lib/state/workspace.svelte";
  import { toaster } from "$lib/toaster";

  let accounts: AccountInfoDto[] = $state([]);
  const formatter = useCurrencyFormatter();
  let isDialogOpen = $state(false);
  let editingAccount: AccountInfoDto | null = $state(null);
  let newAccount: AccountInfoDto = $state({
    id: "",
    name: "",
    balance: 0,
    account_type: null,
    currency: workspace.currency,
    initial_balance: null,
    credit_limit: null,
  });

  const { accountsHaveChanged } = $props();

  onMount(async () => {
    await getAccounts();
  });

  async function getAccounts(): Promise<void> {
    try {
      const result: AccountInfoDto[] = await invoke("get_accounts", {});
      console.log("cuentas",result)
      if (!result) {
        throw Error("Couldn't get accounts");
      }
      accounts = result;
      console.log("accountsss",accounts)
    } catch (error: any) {
      handleCommandError(error);
    }
  }

  async function handleCreateAccount() {
    try {
      newAccount.balance = newAccount.initial_balance ?? 0;
      const accountId: string = await invoke("create_account", { newAccount });
      newAccount.id = accountId;
      accounts = [...accounts, newAccount];
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

      accountsHaveChanged();

      toaster.success({
        title: "Exito",
        description: "Cuenta creada exitosamente",
      });
    } catch (error: any) {
      handleCommandError(error);
    }
  }

  function openEditDialog(account: AccountInfoDto) {
    editingAccount = { ...account };
  }

  async function handleEditAccount() {
    if (!editingAccount) return;
    try {
      editingAccount.balance = editingAccount.initial_balance ?? 0;
      await invoke("update_account", { account: editingAccount });
      accounts = accounts.map((a) => (a.id === editingAccount!.id ? editingAccount! : a));
      accountsHaveChanged();
      editingAccount = null;
      toaster.success({
        title: "Exito",
        description: "Cuenta editada exitosamente",
      });
    } catch (error: any) {
      console.log(error);
      handleCommandError(error);
    }
  }

  async function handleDeleteAccount(accountId: string) {
    try {
      if (!accountId) {
        toaster.error({
          title: "Error",
          description:
            "Error obteniendo la cuenta a eliminar. Intenta de nuevo más tarde.",
        });
        throw Error("Account ID is required for deletion");
      }
      let result = await invoke("delete_account", { accountId });
      if (result) {
        accounts = accounts.filter((a) => a.id !== accountId);
        accountsHaveChanged();
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

<section id="accounts-section" class="mt-2">
  <h4 class="">Cuentas:</h4>
  <div class="border-1 border-dashed w-full grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-3 p-2 items-stretch">
    {#each accounts as account (account.id)}
      {@const id = account.id}
      <div class="card p-4 preset-filled-success-500 relative group min-w-0 h-full flex flex-col">
        <h5 class="break-words">{account.name}</h5>
        <h5 class="font-semibold break-all">
          {formatter.format(account.balance)}
        </h5>
        <Dialog>
          <Dialog.Trigger
            class="btn absolute top-0 right-0 opacity-0 group-hover:opacity-100 transition-opacity"
            ><TrashIcon size={17} /></Dialog.Trigger
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
                    class="btn preset-tonal-error"
                    onclick={() => handleDeleteAccount(id)}
                  >
                    Eliminar
                  </Dialog.CloseTrigger>
                </footer>
              </Dialog.Content>
            </Dialog.Positioner>
          </Portal>
        </Dialog>
        <Dialog>
          <Dialog.Trigger
            class="btn absolute top-0 right-7 opacity-0 group-hover:opacity-100 transition-opacity"
            onclick={() => openEditDialog(account)}
            ><SquarePenIcon size={17} /></Dialog.Trigger
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
                    >Editar cuenta</Dialog.Title
                  >
                  <Dialog.CloseTrigger class="btn-icon hover:preset-tonal">
                    <XIcon class="size-4" />
                  </Dialog.CloseTrigger>
                </header>
                {#if editingAccount}
                <div class="flex flex-col gap-4">
                  <label class="label">
                    <span>Nombre de la cuenta</span>
                    <input
                      class="input"
                      type="text"
                      placeholder="Ej: Billetera"
                      bind:value={editingAccount.name}
                    />
                  </label>

                  <label class="label">
                    <span>Tipo de cuenta</span>
                    <select class="select" bind:value={editingAccount.account_type}>
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
                      bind:value={editingAccount.initial_balance}
                    />
                  </label>
                  {#if editingAccount.account_type === "credit"}
                    <label class="label">
                      <span>Límite de crédito</span>
                      <input
                        class="input"
                        type="number"
                        placeholder="0.00"
                        bind:value={editingAccount.credit_limit}
                      />
                    </label>
                  {/if}
                </div>
                {/if}
                <footer class="flex justify-end gap-2">
                  <Dialog.CloseTrigger class="btn preset-tonal"
                    >Cancelar</Dialog.CloseTrigger
                  >
                  <Dialog.CloseTrigger
                    ><button
                      onclick={handleEditAccount}
                      type="button"
                      class="btn preset-filled">Guardar</button
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
