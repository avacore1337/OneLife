<template>
  <div class="app">
    <b-modal ref="the-modal" hide-footer hide-header title="Using Component Methods">
      <span class="the-modal">
        <div class="d-block text-center">
          <h3>The Modal ...</h3>
        </div>
        <b-button class="float-end" variant="danger" block @click="hideModal">Close</b-button>
      </span>
    </b-modal>

    <div style="width: 15%; float: left">
      <Sidebar
        v-bind:state="state"
        v-bind:input="input"
        v-bind:world="world"
        v-bind:wasm="wasm"
        v-bind:metaData="metaData"
      />
    </div>

    <div style="margin-left: 2%; float: left; width: 40%">
      <Works v-bind:state="state" v-bind:input="input" v-bind:world="world" v-bind:wasm="wasm" />
      <Housing v-bind:state="state" v-bind:input="input" v-bind:world="world" v-bind:wasm="wasm" />
      <Activities v-bind:state="state" v-bind:input="input" v-bind:world="world" v-bind:wasm="wasm" />
      <BoostItems v-bind:state="state" v-bind:input="input" v-bind:world="world" v-bind:wasm="wasm" />
      <Tombs v-bind:state="state" v-bind:input="input" v-bind:world="world" v-bind:wasm="wasm" />
    </div>

    <div style="margin-left: 20px; float: left">
      <div
        style="margin-left: 20px; border: 5px solid white; width: 300px; display: inline; float: left; padding: 10px"
      >
        <Death v-bind:state="state" v-bind:input="input" v-bind:world="world" v-bind:wasm="wasm" />
        <RebirthUpgrades v-bind:state="state" v-bind:input="input" v-bind:world="world" v-bind:wasm="wasm" />
      </div>

      <div
        style="margin-left: 20px; border: 5px solid white; width: 200px; display: inline; float: left; padding: 10px"
      >
        <Debug
          v-bind:metaData="metaData"
          v-bind:state="state"
          v-bind:input="input"
          v-bind:presets="presets"
          v-bind:wasm="wasm"
        />
      </div>
    </div>
  </div>
</template>

<script>
import Works from "./components/Works.vue";
import Tombs from "./components/Tombs.vue";
import Housing from "./components/Housing.vue";
import Activities from "./components/Activities.vue";
import Death from "./components/Death.vue";
import Debug from "./components/Debug.vue";
import Sidebar from "./components/Sidebar.vue";
import BoostItems from "./components/BoostItems.vue";
import RebirthUpgrades from "./components/RebirthUpgrades.vue";

import Vue from "vue/dist/vue.js";
import { BootstrapVue } from "bootstrap-vue";
import { downloadFile } from "./utility.js";

import "bootstrap/dist/css/bootstrap.css";
import "bootstrap-vue/dist/bootstrap-vue.css";

Vue.use(BootstrapVue);

export default {
  props: ["wasm"],
  components: { Works, Housing, Activities, Debug, BoostItems, Death, Sidebar, Tombs, RebirthUpgrades },
  data() {
    return {
      world: {
        works: [],
        tiers: [],
        boost_items: [],
        rebirth_upgrades: [],
      },
      state: {
        base_stats: {},
        items: {
          boost_items: [],
        },
        life_stats: {},
        rebirth_stats: {
          rebirth_upgrades: [],
        },
      },
      input: {},
      metaData: { info: {}, saved_ticks: 0.0 },
      presets: {},
      paused: false,
    };
  },
  mounted: function () {
    this.world = this.wasm.get_world();
    this.state = this.wasm.get_state();
    this.input = this.wasm.get_input();
    this.metaData = this.wasm.get_meta_data();
    this.presets = this.wasm.get_preset_saves();

    let self = this;
    setInterval(function () {
      if (self.paused) {
        self.wasm.paused();
        return;
      }

      self.wasm.tick();
      self.state = self.wasm.get_state();
      self.input = self.wasm.get_input();
      self.metaData = self.wasm.get_meta_data();

      // self.showModal();
    }, 1000 / 30);
  },
  methods: {
    showModal() {
      this.$refs["the-modal"].show();
    },
    hideModal() {
      this.$refs["the-modal"].hide();
    },
    toggleModal() {},
  },
};
</script>

<style>
body {
  background-color: #232c3a;
  color: white;
}

.notransition > div.progress-bar {
  -webkit-transition: none !important;
  -moz-transition: none !important;
  -o-transition: none !important;
  -ms-transition: none !important;
  transition: none !important;
}

span.the-modal {
  color: black;
}

div.app {
  width: 100vw;
  height: 100vh;
}
</style>
