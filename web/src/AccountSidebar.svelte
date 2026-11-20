<script lang="ts">
  import NetworkLogo from "./NetworkLogo.svelte";
  import { TONE, groupByNetwork, initials, type AccountSummary } from "./accounts";
  import { t } from "./i18n";

  let {
    accounts,
    selectedId,
    onselect,
  }: { accounts: AccountSummary[]; selectedId: string | null; onselect: (id: string) => void } = $props();

  const groups = $derived(groupByNetwork(accounts));
</script>

<aside>
  <div class="heading">{t("sidebar-heading")}</div>
  {#each groups as group (group.network)}
    <section>
      <div class="network">
        <NetworkLogo network={group.network} />
        <span>{group.network}</span>
        <button class="connect">{t("sidebar-connect")}</button>
      </div>
      {#each group.accounts as account (account.id)}
        <button class="account" class:on={account.id === selectedId} onclick={() => onselect(account.id)}>
          <span class="avatar">
            {initials(account.label)}
            <span class="badge"><NetworkLogo network={account.network} size={11} /></span>
          </span>
          <span class="who">
            <span class="name">{account.label}</span>
            <span class="state"><i class="dot {TONE[account.state]}"></i>{t(`account-state-${account.state}`)}</span>
          </span>
          {#if account.pendingReplies > 0}
            <span class="count" title={t("pending-replies", { count: account.pendingReplies })}>{account.pendingReplies}</span>
          {/if}
        </button>
      {/each}
      {#if group.accounts.length === 0}
        <div class="empty">{t("sidebar-no-account")}</div>
      {/if}
    </section>
  {/each}
</aside>

<style>
  aside {
    background: var(--surface-sidebar);
    border-right: 1px solid var(--line);
    overflow: auto;
    padding: 22px 14px;
    scrollbar-width: thin;
    scrollbar-color: var(--scrollbar) transparent;
  }
  .heading {
    font-size: 12px;
    font-weight: 600;
    color: var(--muted);
    text-transform: uppercase;
    letter-spacing: 0.08em;
    padding: 0 10px 14px;
  }
  section { margin-bottom: 14px; }
  .network { display: flex; align-items: center; gap: 10px; padding: 6px 10px; font-size: 13px; font-weight: 600; }
  .connect { margin-left: auto; font-size: 12px; color: var(--muted); font-weight: 500; }
  .connect:hover { color: var(--ink); }
  .account { width: 100%; display: flex; align-items: center; gap: 12px; padding: 8px 10px; border-radius: 8px; text-align: left; }
  .account:hover { background: var(--surface-hover); }
  .account.on { background: var(--surface-selected); box-shadow: 0 1px 2px rgb(28 27 34 / 6%), inset 3px 0 0 var(--accent); }
  .avatar {
    position: relative;
    width: 34px;
    height: 34px;
    border-radius: 50%;
    background: var(--surface-avatar);
    color: var(--ink-soft);
    display: grid;
    place-items: center;
    font-size: 12px;
    font-weight: 600;
    flex: none;
  }
  .badge {
    position: absolute;
    right: -3px;
    bottom: -3px;
    width: 18px;
    height: 18px;
    border-radius: 50%;
    background: var(--surface-panel);
    display: grid;
    place-items: center;
    box-shadow: 0 0 0 1px var(--line);
  }
  .who { flex: 1; display: flex; flex-direction: column; min-width: 0; }
  .name { font-size: 14px; font-weight: 500; }
  .state { display: inline-flex; align-items: center; gap: 6px; font-size: 12px; color: var(--muted); }
  .dot { width: 6px; height: 6px; border-radius: 50%; display: inline-block; }
  .dot.ok { background: var(--status-ok); }
  .dot.warn { background: var(--status-warn); }
  .dot.bad { background: var(--status-bad); }
  .dot.idle { background: var(--status-idle); }
  .count { font-size: 12px; color: var(--muted); font-variant-numeric: tabular-nums; }
  .empty { font-size: 12px; color: var(--muted); padding: 0 10px 4px 44px; }
</style>
