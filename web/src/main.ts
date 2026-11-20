import { mount } from "svelte";
import App from "./App.svelte";
import { fixtureApi } from "./api";
import "./tokens.css";

const target = document.getElementById("app");
if (target === null) {
  throw new Error("missing #app element");
}

mount(App, { target, props: { api: fixtureApi } });
