<template>
  <div>
    <div style="width: 300px; float: left">
      <Sidebar
        v-bind:state="state"
        v-bind:input="input"
        v-bind:world="world"
        v-bind:wasm="wasm"
        v-bind:metaData="metaData"
      />
    </div>

    <div style="margin-left: 420px">
      <div style="float: left">
        <div style="border: solid; width: 400px; float: left">
          <Works v-bind:state="state" v-bind:input="input" v-bind:world="world" v-bind:wasm="wasm" />
          <Housing v-bind:state="state" v-bind:input="input" v-bind:world="world" v-bind:wasm="wasm" />
          <BoostItems v-bind:state="state" v-bind:input="input" v-bind:world="world" v-bind:wasm="wasm" />
        </div>
        <!-- <div style="border:solid;margin-left: 420px; width: 400px;" v-if="state.life_stats.dead"> -->
        <div style="border: solid; margin-left: 420px; width: 400px">
          <Death v-bind:state="state" v-bind:input="input" v-bind:world="world" v-bind:wasm="wasm" />
        </div>
      </div>

      <div style="border: solid; margin-left: 840px; width: 400px; padding: 10px">
        <Debug v-bind:state="state" v-bind:input="input" v-bind:presets="presets" v-bind:wasm="wasm" />
      </div>
    </div>
  </div>
</template>

<script>
import Works from "./components/Works.vue";
import Housing from "./components/Housing.vue";
import Death from "./components/Death.vue";
import Debug from "./components/Debug.vue";
import Sidebar from "./components/Sidebar.vue";
import BoostItems from "./components/BoostItems.vue";

import Vue from "vue/dist/vue.js";
import { BootstrapVue } from "bootstrap-vue";
import { downloadFile } from "./utility.js";

import "bootstrap/dist/css/bootstrap.css";
import "bootstrap-vue/dist/bootstrap-vue.css";

Vue.use(BootstrapVue);

export default {
  props: ["wasm"],
  components: { Works, Housing, Debug, BoostItems, Death, Sidebar },
  data() {
    return {
      world: {
        works: [],
        tiers: [],
        boost_items: [],
      },
      state: {
        base_stats: {},
        items: {
          boost_items: [],
        },
        life_stats: {},
        rebirth_stats: {},
      },
      input: {},
      metaData: {},
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
        return;
      }

      self.wasm.tick();
      self.state = self.wasm.get_state();
      self.input = self.wasm.get_input();
      self.metaData = self.wasm.get_meta_data();
    }, 1000 / 30);
  },
  methods: {},
};
</script>

<style>
.notransition > div.progress-bar {
  -webkit-transition: none !important;
  -moz-transition: none !important;
  -o-transition: none !important;
  -ms-transition: none !important;
  transition: none !important;
}
</style>
