import Vue from "vue/dist/vue.js";
import { BootstrapVue } from "bootstrap-vue";
import App from "./App.vue";

import "../css/styles.css";
import "bootstrap/dist/css/bootstrap.css";
import "bootstrap-vue/dist/bootstrap-vue.css";

Vue.use(BootstrapVue);

import("../pkg/index.js")
  .then(function (wasm) {
    wasm.load();
    new Vue({
      el: "#app",
      render: (h) =>
        h(App, {
          props: { wasm },
        }),
    });
  })
  .catch(console.error);
