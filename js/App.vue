<template>
  <div class="app">
    <b-modal ref="the-modal" hide-footer hide-header title="Using Component Methods">
      <span class="the-modal">
        <div class="d-block text-center">
          <h3>{{ this.modalText }}</h3>
        </div>
        <b-button class="float-end" block @click="hideModal">Next</b-button>
        <b-button class="float-end" variant="danger" block @click="disable_tutorial">Disable Tutorial</b-button>
      </span>
    </b-modal>
    <div>
      <Topbar
        v-bind:state="state"
        v-bind:input="input"
        v-bind:world="world"
        v-bind:wasm="wasm"
        v-bind:metaData="metaData"
      />
    </div>
    <div>
      <div style="width: 20%; float: left">
        <Sidebar
          v-bind:state="state"
          v-bind:input="input"
          v-bind:world="world"
          v-bind:wasm="wasm"
          v-bind:metaData="metaData"
        />
      </div>

      <div style="margin-left: 2%; float: left; width: 30%">
        <Works v-bind:state="state" v-bind:input="input" v-bind:world="world" v-bind:wasm="wasm" />
        <Housing v-bind:state="state" v-bind:input="input" v-bind:world="world" v-bind:wasm="wasm" />
      </div>

      <div style="margin-left: 2%; float: left; width: 30%">
        <Activities v-bind:state="state" v-bind:input="input" v-bind:world="world" v-bind:wasm="wasm" />
        <BoostItems v-bind:state="state" v-bind:input="input" v-bind:world="world" v-bind:wasm="wasm" />
        <Tombs v-bind:state="state" v-bind:input="input" v-bind:world="world" v-bind:wasm="wasm" />
        <Blessings
          v-if="state.rebirth_stats.unlocks.has_faith"
          v-bind:state="state"
          v-bind:input="input"
          v-bind:world="world"
          v-bind:wasm="wasm"
        />
      </div>

      <div style="margin-left: 2%; float: left">
        <div
          style="margin-left: 20px; border: 5px solid white; width: 300px; display: inline; float: left; padding: 10px"
          v-if="state.life_stats.dead || state.life_stats.is_dying || state.rebirth_stats.rebirth_count > 0"
        >
          <Death v-bind:state="state" v-bind:input="input" v-bind:world="world" v-bind:wasm="wasm" />
          <RebirthUpgrades
            v-if="state.rebirth_stats.class_tier > 0"
            v-bind:state="state"
            v-bind:input="input"
            v-bind:world="world"
            v-bind:wasm="wasm"
          />
        </div>

        <div
          style="margin-left: 20px; border: 5px solid white; width: 200px; display: inline; float: left; padding: 10px"
          v-if="world.settings.display_debug"
        >
          <Debug v-bind:metaData="metaData" v-bind:state="state" v-bind:input="input" v-bind:wasm="wasm" />
        </div>
      </div>
    </div>
  </div>
</template>

<script>
import Works from "./components/Works.vue";
import Tombs from "./components/Tombs.vue";
import Blessings from "./components/Blessings.vue";
import Housing from "./components/Housing.vue";
import Activities from "./components/Activities.vue";
import Death from "./components/Death.vue";
import Debug from "./components/Debug.vue";
import Sidebar from "./components/Sidebar.vue";
import Topbar from "./components/Topbar.vue";
import BoostItems from "./components/BoostItems.vue";
import RebirthUpgrades from "./components/RebirthUpgrades.vue";

import Vue from "vue/dist/vue.js";
import { BootstrapVue } from "bootstrap-vue";

import "bootstrap/dist/css/bootstrap.css";
import "bootstrap-vue/dist/bootstrap-vue.css";

Vue.use(BootstrapVue);
Vue.config.performance = true;

export default {
  props: ["wasm"],
  components: {
    Works,
    Housing,
    Activities,
    Debug,
    BoostItems,
    Death,
    Sidebar,
    Topbar,
    Blessings,
    Tombs,
    RebirthUpgrades,
  },
  data() {
    return {
      world: {
        works: [],
        housing: [],
        tiers: [],
        stats: [],
        boost_items: [],
        rebirth_upgrades: [],
        tutorial_texts: [],
        settings: {},
      },
      state: {
        stats: {},
        skills: {},
        housing: [],
        items: {
          boost_items: [],
        },
        life_stats: { health: 0.0, health_rate: 0.0 },
        rebirth_stats: {
          rebirth_upgrades: [],
          unlocks: {},
        },
      },
      input: {},
      metaData: { info: {}, saved_ticks: 0.0 },
      paused: false,
      numberFormat: "DEFAULT",
      modalText: "",
    };
  },
  mounted: function () {
    this.world = Object.freeze(this.wasm.get_world());
    this.state = this.wasm.get_state();
    this.input = this.wasm.get_input();
    this.metaData = this.wasm.get_meta_data();

    let self = this;
    setInterval(function () {
      if (self.paused || self.state.life_stats.is_dying || self.state.life_stats.dead) {
        self.wasm.paused();
        self.update_dynamic_data();
        self.updateModal();
        return;
      }

      self.wasm.tick();
      self.update_dynamic_data();

      self.updateModal();
    }, 1000 / 30);
  },
  methods: {
    recurse_update(o, o2) {
      for (var key in o2) {
        if (typeof o2[key] == "object") {
          this.recurse_update(o[key], o2[key]);
          continue;
        }
        if (o[key] != o2[key]) {
          /* console.log(typeof o[key]); */
          /* console.log(key, o[key], o2[key]); */
          o[key] = o2[key];
        }
      }
    },
    update_dynamic_data() {
      // 20 fps
      /* this.state= this.wasm.get_state(); */
      /* this.input= this.wasm.get_input(); */
      /* this.metaData= this.wasm.get_meta_data(); */
      // 30 fps
      this.recurse_update(this.state, this.wasm.get_state());
      this.recurse_update(this.input, this.wasm.get_input());
      this.recurse_update(this.metaData, this.wasm.get_meta_data());
    },
    updateModal() {
      let modal = this.$refs["the-modal"];
      if (this.metaData.info.show_tutorial && modal.isHidden) {
        this.modalText = this.world.tutorial_texts[this.metaData.info.tutorial_step];
        modal.show();
      }
    },
    showModal() {
      this.$refs["the-modal"].show();
    },
    hideModal() {
      this.wasm.next_info_step();
      this.$refs["the-modal"].hide();
    },
    toggleModal() {},
    disable_tutorial: function () {
      this.wasm.set_disable_tutorial(true);
      this.wasm.next_info_step();
      this.$refs["the-modal"].hide();
    },
  },
};
</script>

<style>
body {
  background-color: #232c3a;
  color: white;
}

span.the-modal {
  color: black;
}

div.app {
  width: 100vw;
  height: 100vh;
}
</style>
