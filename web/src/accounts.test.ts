import { describe, expect, it } from "vitest";
import { type AccountSummary, NETWORKS, groupByNetwork, initials } from "./accounts";

const account = (id: string, network: AccountSummary["network"], label: string): AccountSummary => ({
  id,
  network,
  label,
  state: "Connected",
  pendingReplies: 0,
});

describe("groupByNetwork", () => {
  it("lists every Network in a fixed order, Accounts sorted by label, empty Networks kept", () => {
    const groups = groupByNetwork([
      account("wa-2", "WhatsApp", "+55 71 98888"),
      account("tg-1", "Telegram", "@paulo"),
      account("wa-1", "WhatsApp", "+55 71 11111"),
    ]);

    expect(groups.map((g) => [g.network, g.accounts.map((a) => a.id)])).toEqual([
      ["WhatsApp", ["wa-1", "wa-2"]],
      ["Telegram", ["tg-1"]],
      ["Instagram", []],
      ["LinkedIn", []],
      ["X", []],
      ["Teams", []],
    ]);
    expect(groups).toHaveLength(NETWORKS.length);
  });
});

describe("initials", () => {
  it.each([
    ["+55 71 99999-0000", "55"],
    ["@byo.store", "BY"],
    ["Paulo C.", "PA"],
    ["João", "JO"],
    ["@@@", "?"],
  ])("%s → %s", (label, expected) => {
    expect(initials(label)).toBe(expected);
  });
});
