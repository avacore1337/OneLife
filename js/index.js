import App from "./App.vue";

import MyIcon from "./components/MyIcon.vue";

import Vue from "vue/dist/vue.js";
import { BootstrapVue } from "bootstrap-vue";
import Vuex from "vuex";
import VueGtag from "vue-gtag";
import Icon from "vue-awesome/components/Icon";

import "vue-awesome/icons";
import "../css/styles.css";
import "bootstrap/dist/css/bootstrap.css";
import "bootstrap-vue/dist/bootstrap-vue.css";

Vue.use(VueGtag, {
  config: { id: "G-JW5LS1NXT6" },
});
Vue.use(Vuex);
Vue.use(BootstrapVue);

Vue.config.performance = true;
Vue.component("VIcon", Icon);
Vue.component("MyIcon", MyIcon);

const store = new Vuex.Store({
  state: {
    count: 0,
    numberFormat: "DEFAULT",
  },
  mutations: {
    toggleNumberFormat(state) {
      state.numberFormat = {
        DEFAULT: "SCIENTIFIC",
        SCIENTIFIC: "DEFAULT",
      }[state.numberFormat];
    },
  },
  getters: {
    getNextNumberFormat(state) {
      return {
        DEFAULT: "Scientific notation",
        SCIENTIFIC: "Natural numbers",
      }[state.numberFormat];
    },
  },
});

import("../pkg/index.js")
  .then(function (wasm) {
    wasm.load();
    new Vue({
      el: "#app",
      store,
      render: (h) =>
        h(App, {
          props: { wasm },
        }),
    });
  })
  .catch(console.error);
