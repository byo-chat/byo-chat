export const NETWORKS = ["WhatsApp", "Telegram", "Instagram", "LinkedIn", "X", "Teams"] as const;
export type Network = (typeof NETWORKS)[number];

export const ACCOUNT_STATES = [
  "New",
  "Authenticating",
  "Connected",
  "Reconnecting",
  "Disconnected",
  "Expired",
  "Paused",
] as const;
export type AccountState = (typeof ACCOUNT_STATES)[number];

export type Tone = "ok" | "warn" | "bad" | "idle";

export const TONE: Record<AccountState, Tone> = {
  New: "idle",
  Authenticating: "warn",
  Connected: "ok",
  Reconnecting: "warn",
  Disconnected: "bad",
  Expired: "bad",
  Paused: "idle",
};

export interface AccountSummary {
  id: string;
  network: Network;
  label: string;
  state: AccountState;
  pendingReplies: number;
}

export interface NetworkGroup {
  network: Network;
  accounts: AccountSummary[];
}

export function groupByNetwork(accounts: readonly AccountSummary[]): NetworkGroup[] {
  return NETWORKS.map((network) => ({
    network,
    accounts: accounts
      .filter((account) => account.network === network)
      .sort((a, b) => a.label.localeCompare(b.label)),
  }));
}

export function initials(label: string): string {
  const letters = label.replace(/[^\p{L}\p{N}]/gu, "").slice(0, 2).toUpperCase();
  return letters || "?";
}
