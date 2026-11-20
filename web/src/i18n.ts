import { FluentBundle, FluentResource, type FluentVariable } from "@fluent/bundle";
import english from "../locales/en.ftl?raw";

export type MessageId =
  | "app-title"
  | "sidebar-heading"
  | "sidebar-connect"
  | "sidebar-no-account"
  | `account-state-${string}`
  | "pending-replies";

export function bundleFor(source: string, locale = "en"): FluentBundle {
  const bundle = new FluentBundle(locale, { useIsolating: false });
  const errors = bundle.addResource(new FluentResource(source));
  if (errors.length > 0) {
    throw new Error(`invalid ${locale} catalog: ${errors.map((e) => e.message).join("; ")}`);
  }
  return bundle;
}

const bundle = bundleFor(english);

export function t(id: MessageId, args?: Record<string, FluentVariable>): string {
  const message = bundle.getMessage(id);
  if (message?.value == null) {
    throw new Error(`missing message ${id}`);
  }
  return bundle.formatPattern(message.value, args);
}
