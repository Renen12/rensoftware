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
          document.querySelectorAll("button").forEach((btn) => {
            btn.disabled = true
          });
          document.getElementById("home").href = "javascript:void(0)";
          document.getElementById("updates").href = "javascript:void(0)";
          await invoke("install_app", { application: button.internal_appid });
          document.querySelectorAll("button").forEach((btn) => {
            btn.disabled = false
          });
          document.getElementById("home").href = "/index.html";
          document.getElementById("updates").href = "/updates.html";
          alert("Software installation has finished.")
        };
      }
    });

  });
}
refresh_apps();
document.getElementById("go").onclick = async () => {
  proceedWithSearch();
}
document.getElementById("search").onkeydown = async (key) => {
  if (key.key == "Enter") {
    proceedWithSearch();
  }
}
async function proceedWithSearch() {
  let searchTerm = document.getElementById("search").value;
  invoke("get_apps").then((apps) => {
    // Remove previous app elements and breaks
    document.querySelectorAll("button").forEach((btn) => {
      if (btn.id != "go") {
        btn.remove();
      }
    });
    document.querySelectorAll("br").forEach((breakelement) => {
      if (breakelement.id != "firstbreak") {
        breakelement.remove();

      }
    });
    apps.forEach((app) => {
      if (app.split("|")[0].toLowerCase().includes(searchTerm.toLowerCase()) == false) {
        return
      }

      const button = document.createElement("button");
      button.type = "button";
      button.innerHTML = app.split("|")[0];
      button.internal_appid = app.split("|")[1];
      document.body.appendChild(button)
      document.body.appendChild(document.createElement("br"));
      button.onclick = async () => {
        document.querySelectorAll("button").forEach((btn) => {
          btn.disabled = true
        });
        document.getElementById("home").href = "javascript:void(0)";
        document.getElementById("updates").href = "javascript:void(0)";
        await invoke("install_app", { application: button.internal_appid });
        document.querySelectorAll("button").forEach((btn) => {
          btn.disabled = false
        });
        document.getElementById("home").href = "/index.html";
        document.getElementById("updates").href = "/updates.html";
        alert("Software installation has finished.")
      };
    });
  });
}
</script>

<template>
</template>