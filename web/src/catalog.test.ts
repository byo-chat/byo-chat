import { readFileSync, readdirSync } from "node:fs";
import { join } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";
import { ACCOUNT_STATES } from "./accounts";
import { bundleFor } from "./i18n";

const src = fileURLToPath(new URL(".", import.meta.url));
const english = readFileSync(join(src, "..", "locales", "en.ftl"), "utf8");

function catalogIds(source: string): string[] {
  return [...source.matchAll(/^([a-zA-Z][\w-]*)\s*=/gm)].map((m) => m[1] ?? "");
}

function usedIds(): string[] {
  const files = readdirSync(src).filter((f: string) => /\.(ts|svelte)$/.test(f) && !f.endsWith(".test.ts"));
  const literal = files.flatMap((f: string) =>
    [...readFileSync(join(src, f), "utf8").matchAll(/\bt\(\s*"([\w-]+)"/g)].map((m) => m[1] ?? ""),
  );
  const states = ACCOUNT_STATES.map((state) => `account-state-${state}`);
  return [...new Set([...literal, ...states])].sort();
}

describe("the English catalog", () => {
  it("parses without errors", () => {
    expect(() => bundleFor(english)).not.toThrow();
  });

  it("holds exactly the messages the code uses", () => {
    expect(catalogIds(english).sort()).toEqual(usedIds());
  });

  it("pluralises pending replies", () => {
    const bundle = bundleFor(english);
    const pattern = bundle.getMessage("pending-replies")?.value;
    expect(pattern).toBeDefined();
    if (pattern == null) return;
    expect([1, 3].map((count) => bundle.formatPattern(pattern, { count }))).toEqual([
      "1 reply waiting",
      "3 replies waiting",
    ]);
  });
});
