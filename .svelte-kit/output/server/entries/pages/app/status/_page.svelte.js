import { c as create_ssr_component, f as each } from "../../../../chunks/ssr.js";
import "@tauri-apps/api/tauri";
const _page_svelte_svelte_type_style_lang = "";
const css = {
  code: ".data.svelte-wx1ygn{word-wrap:break-word;white-space:pre-wrap;word-break:break-word;background-color:#c7c7c7;margin:1rem;padding:1rem}button.svelte-wx1ygn{border-radius:8px;border:1px solid transparent;padding:0.6em 1.2em;font-size:1em;font-weight:500;font-family:inherit;color:#0f0f0f;background-color:#ffffff;transition:border-color 0.25s;box-shadow:0 2px 2px rgba(0, 0, 0, 0.2)}button.svelte-wx1ygn{cursor:pointer}button.svelte-wx1ygn:hover{border-color:#396cd8}button.svelte-wx1ygn:active{border-color:#396cd8;background-color:#e8e8e8}button.svelte-wx1ygn{outline:none}@media(prefers-color-scheme: dark){:root{color:#f6f6f6;background-color:#2f2f2f}button.svelte-wx1ygn{color:#ffffff;background-color:#0f0f0f98}button.svelte-wx1ygn:active{background-color:#0f0f0f69}}",
  map: null
};
const Page = create_ssr_component(($$result, $$props, $$bindings, slots) => {
  let message = window.__ERROR_MESSAGE__;
  let lines = message.split("\n");
  $$result.css.add(css);
  return `<h1 data-svelte-h="svelte-1heaqv">Failed to load settings file</h1> ${message != null ? `<div class="data svelte-wx1ygn">${each(lines, (line) => {
    return `<!-- HTML_TAG_START -->${line}<!-- HTML_TAG_END --><br>`;
  })}</div>` : ``} <button class="svelte-wx1ygn" data-svelte-h="svelte-2h1i94">Reload</button>`;
});
export {
  Page as default
};
