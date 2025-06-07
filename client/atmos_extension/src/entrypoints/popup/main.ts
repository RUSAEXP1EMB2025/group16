import { mount } from "svelte";
import App from "./App.svelte";
import "./app.css";

const app = mount(App, {
  //biome-ignore lint:
  target: document.getElementById("app")!
});

export default app;
