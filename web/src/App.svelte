<script lang="ts">
  import AccountSidebar from "./AccountSidebar.svelte";
  import type { AccountSummary } from "./accounts";
  import type { BridgeApi } from "./api";
  import { t } from "./i18n";

  let { api }: { api: BridgeApi } = $props();

  let accounts: AccountSummary[] = $state([]);
  let selectedId: string | null = $state(null);

  $effect(() => {
    api.listAccounts().then((loaded) => {
      accounts = loaded;
      selectedId ??= loaded[0]?.id ?? null;
    });
  });
</script>

<div class="frame">
  <header><div class="brand">{t("app-title")}</div></header>
  <div class="split">
    <AccountSidebar {accounts} {selectedId} onselect={(id) => (selectedId = id)} />
    <main></main>
  </div>
</div>

<style>
  .frame { height: 100vh; display: flex; flex-direction: column; }
  header { display: flex; align-items: center; padding: 14px 28px; border-bottom: 1px solid var(--line); background: var(--surface-sidebar); }
  .brand { font-weight: 700; font-size: 17px; letter-spacing: -0.03em; }
  .split { flex: 1; display: grid; grid-template-columns: 300px 1fr; min-height: 0; }
</style>
