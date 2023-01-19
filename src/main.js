const { invoke } = window.__TAURI__.tauri;

let greetInputEl;
let greetMsgEl;

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  greetMsgEl.textContent = await invoke("greet", { name: greetInputEl.value });
}

window.addEventListener("DOMContentLoaded", () => {
  greetInputEl = document.querySelector("#greet-input");
  greetMsgEl = document.querySelector("#greet-msg");
  document
    .querySelector("#greet-button")
    .addEventListener("click", () => greet());
});


let configMsgEl;

async function config() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  configMsgEl.innerHTML = JSON.stringify(await invoke("get_config"));
}

window.addEventListener("DOMContentLoaded", () => {
  configMsgEl = document.querySelector("#config-msg");
  document
    .querySelector("#get-config")
    .addEventListener("click", () => config());
});