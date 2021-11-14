<template>
  <div>
    <div style="width: 300px; float: left">
      <Sidebar v-bind:state="state" v-bind:input="input" v-bind:world="world" v-bind:wasm="wasm" />
    </div>

    <div style="margin-left: 420px">
      <div style="float: left">
        <div style="border: solid; width: 400px; float: left">
          <Works v-bind:state="state" v-bind:input="input" v-bind:world="world" v-bind:wasm="wasm" />
          <Housing v-bind:state="state" v-bind:input="input" v-bind:world="world" v-bind:wasm="wasm" />
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

import Vue from "vue/dist/vue.js";
import { BootstrapVue } from "bootstrap-vue";
import { downloadFile } from "./utility.js";

import "bootstrap/dist/css/bootstrap.css";
import "bootstrap-vue/dist/bootstrap-vue.css";

Vue.use(BootstrapVue);

export default {
  props: ["wasm"],
  components: { Works, Housing, Debug, Death, Sidebar },
  data() {
    return {
      world: {
        works: [],
        tiers: [],
      },
      state: {
        base_stats: {},
        items: {},
        life_stats: {},
        rebirth_stats: {},
      },
      input: {},
      presets: {},
      paused: false,
    };
  },
  mounted: function () {
    this.world = this.wasm.get_world();
    this.state = this.wasm.get_state();
    this.input = this.wasm.get_input();
    this.presets = this.wasm.get_preset_saves();

    let self = this;
    setInterval(function () {
      if (self.paused) {
        return;
      }

      self.wasm.tick();
      self.state = self.wasm.get_state();
      self.input = self.wasm.get_input();
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
