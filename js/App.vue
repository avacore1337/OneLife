<template>
  <div class="app" v-if="loaded">
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
    <div class="my-container">
      <div class="left-sidebar">
        <Sidebar
          v-bind:state="state"
          v-bind:input="input"
          v-bind:world="world"
          v-bind:wasm="wasm"
          v-bind:metaData="metaData"
        />
      </div>

      <div class="main">
        <Main
          v-bind:item_queue="item_queue"
          v-bind:metaData="metaData"
          v-bind:state="state"
          v-bind:input="input"
          v-bind:world="world"
          v-bind:wasm="wasm"
          v-bind:previous_recorded_inputs="previous_recorded_inputs"
          v-bind:recorded_inputs="recorded_inputs"
        />
      </div>

      <div style="">
        <div style="margin-left: 20px; border: 5px solid white; padding: 10px">
          <SidebarRight
            v-bind:world="world"
            v-bind:metaData="metaData"
            v-bind:state="state"
            v-bind:input="input"
            v-bind:wasm="wasm"
          />
        </div>
        <div style="margin-left: 20px; border: 5px solid white; padding: 10px" v-if="world.settings.display_debug">
          <Debug
            v-bind:world="world"
            v-bind:metaData="metaData"
            v-bind:state="state"
            v-bind:input="input"
            v-bind:wasm="wasm"
          />
        </div>
      </div>
    </div>
  </div>
</template>

<script>
import Main from "./components/Main.vue";
import Debug from "./components/Debug.vue";
import SidebarRight from "./components/SidebarRight.vue";
import Sidebar from "./components/Sidebar.vue";
import Topbar from "./components/Topbar.vue";

import Vue from "vue/dist/vue.js";
import { BootstrapVue } from "bootstrap-vue";
import Icon from "vue-awesome/components/Icon";
import MyIcon from "./components/MyIcon.vue";

import "vue-awesome/icons";
import "bootstrap/dist/css/bootstrap.css";
import "bootstrap-vue/dist/bootstrap-vue.css";

Vue.use(BootstrapVue);
Vue.config.performance = true;
Vue.component("v-icon", Icon);
Vue.component("my-icon", MyIcon);

export default {
  props: ["wasm"],
  components: {
    Main,
    Debug,
    Sidebar,
    SidebarRight,
    Topbar,
  },
  data() {
    return {
      loaded: false,
      world: {},
      state: {},
      input: {},
      recorded_inputs: {},
      previous_recorded_inputs: {},
      item_queue: [],
      metaData: {},
      numberFormat: "DEFAULT",
      modalText: "",
      updateCount: 0,
      updateRate: 3, // if made into 1, then progress bar sometimes freaks out and eats the cpu...
    };
  },
  mounted: function () {
    this.world = Object.freeze(this.wasm.get_world());
    this.state = this.wasm.get_state();
    this.input = this.wasm.get_input();
    this.recorded_inputs = this.wasm.get_recorded_inputs();
    this.previous_recorded_inputs = this.wasm.get_previous_recorded_inputs();
    this.metaData = this.wasm.get_meta_data();
    this.item_queue = this.wasm.get_world_item_queue();
    this.loaded = true;

    let self = this;
    setInterval(function () {
      if (self.metaData.options.paused || self.state.life_stats.is_dying || self.state.life_stats.dead) {
        self.wasm.paused();
        self.update_dynamic_data();
        self.updateModal();
        return;
      }

      self.wasm.tick();

      self.updateCount += 1;
      if (self.updateCount % self.updateRate === 0) {
        self.update_dynamic_data();
        self.updateModal();
      }
    }, 1000 / 30);
  },
  methods: {
    recurse_update(o, o2) {
      for (var key in o2) {
        if (Array.isArray(o2[key])) {
          if (o[key].length != o2[key].length) {
            console.log(key, o[key], o2[key]);
            o[key] = o2[key];
            continue;
          }
        }
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
      this.recurse_update(this.state, this.wasm.get_state());
      this.recurse_update(this.input, this.wasm.get_input());
      this.recurse_update(this.metaData, this.wasm.get_meta_data());
      if (this.metaData.options.show_recorded) {
        let recorded = this.wasm.get_recorded_inputs();
        if (this.recorded_inputs.length != recorded.length) {
          this.recorded_inputs = recorded;
        } else {
          this.recurse_update(this.recorded_inputs, recorded);
        }
        let previous_recorded = this.wasm.get_previous_recorded_inputs();
        if (this.previous_recorded_inputs.length != previous_recorded.length) {
          this.previous_recorded_inputs = previous_recorded;
        } else {
          this.recurse_update(this.previous_recorded_inputs, previous_recorded);
        }
      }
      let queue = this.wasm.get_world_item_queue();
      if (this.item_queue.length != queue.length) {
        this.item_queue = queue;
      } else {
        this.recurse_update(this.item_queue, queue);
      }
    },
    updateModal() {
      let modal = this.$refs["the-modal"];
      if (this.metaData.info.show_tutorial && modal.isHidden) {
        console.log("update modal");
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
  overflow-x: hidden;
}

span.the-modal {
  color: black;
}

.my-container {
  display: flex;
  /* max-width: 1920; */
  margin: 0 auto;
  gap: 10px;
  padding: 10px;
}

.left-sidebar {
  width: 280px;
}

.right-sidebar {
  width: 280px;
}

.main {
  flex-grow: 1;
}

div.app {
  /* width: 100vw; */
  /* height: 100vh; */
}
</style>
