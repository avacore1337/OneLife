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
          Death Section
          <button v-on:click="rebirth" style="margin: 2px">Rebirth</button>
          <div style="border: solid; margin: 2px">
            <ul>
              <li
                v-for="tier in world.tiers.filter((tier) => tier.level > state.rebirth_stats.class_tier)"
                :key="tier.name"
              >
                <button v-on:click="buy_tier(tier.level)" style="margin: 2px" :disabled="!can_buy_tier(tier)">
                  T{{ tier.level }} {{ tier.title }}: Cost {{ tier.purchasing_cost }}
                </button>
              </li>
            </ul>
          </div>
        </div>
      </div>

      <div style="border: solid; margin-left: 840px; width: 400px; padding: 10px">
        Debug

        <br /><br />
        <button v-on:click="set_gamespeed(1)">Set GameSpeed 1</button>
        <br />
        <button v-on:click="set_gamespeed(100)">Set GameSpeed 100</button>
        <br />
        <button v-on:click="toggle_pause" style="margin: 2px">
          {{ paused ? "Resume the game" : "Pause the game" }}
        </button>
        <br />
        <button v-on:click="tick">Tick</button>
        <br />
        <button v-on:click="print_debug">Print Debug</button>

        <br /><br />
        <button v-on:click="export_save" style="margin: 2px">Export gamesave</button>

        <br />
        Import gamesave
        <p>TODO</p>

        <br /><br />
        Presets
        <div v-for="(value, name) in presets" :key="name">
          <button v-on:click="load_preset(name)" style="margin: 2px">{{ name }}</button>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
import Works from "./components/Works.vue";
import Housing from "./components/Housing.vue";
import Sidebar from "./components/Sidebar.vue";

import Vue from "vue/dist/vue.js";
import { BootstrapVue } from "bootstrap-vue";
import { downloadFile } from "./utility.js";

import "bootstrap/dist/css/bootstrap.css";
import "bootstrap-vue/dist/bootstrap-vue.css";

Vue.use(BootstrapVue);

export default {
  props: ["wasm"],
  components: { Works, Housing, Sidebar },
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
  methods: {
    print_debug: function () {
      this.wasm.print_debug();
    },
    set_gamespeed: function (game_speed) {
      this.wasm.set_gamespeed(game_speed);
    },
    tick: function (work_name) {
      this.wasm.single_tick();

      this.state = this.wasm.get_state();
      this.input = this.wasm.get_input();
    },
    buy_tier: function (index) {
      this.wasm.buy_tier(index);
    },
    load_preset: function (preset) {
      this.wasm.set_preset_saves(preset);
    },
    rebirth: function () {
      this.wasm.do_rebirth();
    },
    toggle_pause: function () {
      this.paused = !this.paused;
    },
    set_autosave: function (autosave) {
      this.wasm.set_autosave(autosave);
    },
    set_autosave: function (autosave) {
      this.wasm.set_autosave(autosave);
    },
    can_buy_tier: function (tier) {
      return this.wasm.can_buy_tier(tier.level);
    },
    export_save: function () {
      // TODO: This should be exported by the backend
      downloadFile(`gamesave_${Date.now()}.txt`, this.wasm.export_save());
    },
    import_save: function (event) {
      // TODO: This is only on the frontend atm, it doesn't actually save the changes
      var files = event.target.files;
      var f = files[0];
      var reader = new FileReader();
      var self = this;

      reader.onload = (function (theFile) {
        return function (e) {
          var data = e.target.result;
          console.log(data);
          this.wasm.import_save(data);
        };
      })(f);
      reader.readAsText(f);
    },
  },
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
