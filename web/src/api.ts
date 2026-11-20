import type { AccountSummary } from "./accounts";

export interface BridgeApi {
  listAccounts(): Promise<AccountSummary[]>;
}

export const fixtureApi: BridgeApi = {
  listAccounts: async () => [
    { id: "wa-1", network: "WhatsApp", label: "+55 71 99999-0000", state: "Connected", pendingReplies: 0 },
    { id: "wa-2", network: "WhatsApp", label: "+55 71 98888-1111", state: "Reconnecting", pendingReplies: 3 },
    { id: "tg-1", network: "Telegram", label: "@paulo", state: "Connected", pendingReplies: 0 },
    { id: "ig-1", network: "Instagram", label: "@byo.store", state: "Expired", pendingReplies: 5 },
    { id: "li-1", network: "LinkedIn", label: "Paulo C.", state: "Authenticating", pendingReplies: 0 },
  ],
};
