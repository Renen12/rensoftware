<script setup>

import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event"
listen("getapps_err", async () => {
  alert("An error has occured while getting the available apps.");
});
listen("failed_install", async () => {
  alert("An error has occured while installing the selected app.");
});
function refresh_apps() {
  invoke("get_apps").then((vector) => {
    vector.forEach((app, index) => {
      if (app.replace(" ", "") !== "") {
        const randomid = `app-element-${index}-${Date.now()}`;

        const button = document.createElement("button");
        button.type = "button";
        button.id = randomid;
        button.innerHTML = app.split("|")[0];
        button.internal_appid = app.split("|")[1];

        document.body.appendChild(button);
        document.body.appendChild(document.createElement("br"));

        button.onclick = async () => {

          await invoke("install_app", { application: button.internal_appid });
          alert("Software installation has finished.")
        };
      }
    });

  });
}
refresh_apps();
</script>

<template>
  <h1>
    Home
  </h1>
</template>