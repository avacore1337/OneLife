<template>
  <div>
    <h3>Settings</h3>
    <br />
    Saved Ticks: {{ metaData.saved_ticks.toFixed(0) }}
    <br />
    <button v-on:click="toggle_use_saved_ticks" style="margin: 2px">
      {{ !metaData.use_saved_ticks ? "Use Saved Ticks" : "Don't Use Saved Ticks" }}
    </button>
    <br />
    <button v-on:click="toggle_auto_work" style="margin: 2px">
      {{ !metaData.options.auto_work ? "Auto Work" : "Don't Auto Work" }}
    </button>
    <br />
    <button v-on:click="toggle_auto_living" style="margin: 2px">
      {{ !metaData.options.auto_living ? "Auto Living" : "Don't Auto Living" }}
    </button>
    <br />
    <button v-on:click="toggle_auto_buy_item" style="margin: 2px">
      {{ !metaData.options.auto_buy_item ? "Auto Buy Item" : "Don't Auto Buy Item" }}
    </button>
    <br />
    <button v-on:click="toggle_auto_buy_blessing" style="margin: 2px">
      {{ !metaData.options.auto_buy_blessing ? "Auto Buy Blessing" : "Don't Auto Buy Blessing" }}
    </button>
    <br />
    <button v-on:click="toggle_auto_buy_tomb" style="margin: 2px">
      {{ !metaData.options.auto_buy_tomb ? "Auto Buy Tomb" : "Don't Auto Buy Tomb" }}
    </button>
    <br />
    <button v-on:click="toggle_auto_rebirth" style="margin: 2px">
      {{ !metaData.options.auto_rebirth ? "Auto Rebirth" : "Don't Auto Rebirth" }}
    </button>
    <br />
    <input v-model="end_early_criteria" size="10" />
    <br />
    <button v-on:click="wasm.set_auto_end_early(end_early_criteria)">Set End Early Criteria</button>
    <br />
    <button v-on:click="wasm.set_disable_tutorial(false)">Enable Tutorial</button>
    <br />
    <button v-on:click="toggle_show_recorded" style="margin: 2px">
      {{ !metaData.options.show_recorded ? "Show Recorded" : "Don't Show Recorded" }}
    </button>
    <br />
    <button v-on:click="toggle_pause" style="margin: 2px">
      {{ metaData.options.paused ? "Resume the game" : "Pause the game" }}
    </button>
    <br />
    <button v-on:click="tick">Tick</button>
    <br />
  </div>
</template>

<script>
import { downloadFile } from "../utility.js";
export default {
  props: ["metaData", "state", "world", "input", "wasm"],
  data() {
    return {
      presets: [],
      money: 1000000000.0,
      coins: 1000000.0,
      divine_favor: 10000.0,
      end_early_criteria: 0.0,
    };
  },
  mounted: function () {
    this.presets = this.wasm.get_preset_saves();
  },
  methods: {
    toggle_use_saved_ticks: function () {
      this.wasm.use_saved_ticks(!this.metaData.use_saved_ticks);
    },
    toggle_auto_work: function () {
      this.wasm.set_auto_work(!this.metaData.options.auto_work);
    },
    toggle_auto_living: function () {
      this.wasm.set_auto_living(!this.metaData.options.auto_living);
    },
    toggle_auto_buy_item: function () {
      this.wasm.set_auto_buy_item(!this.metaData.options.auto_buy_item);
    },
    toggle_auto_buy_blessing: function () {
      this.wasm.set_auto_buy_blessing(!this.metaData.options.auto_buy_blessing);
    },
    toggle_auto_buy_tomb: function () {
      this.wasm.set_auto_buy_tomb(!this.metaData.options.auto_buy_tomb);
    },
    toggle_auto_rebirth: function () {
      this.wasm.set_auto_rebirth(!this.metaData.options.auto_rebirth);
    },
    tick: function (work_name) {
      this.wasm.single_tick();
      this.$parent.update_dynamic_data();
    },
    toggle_show_recorded: function () {
      this.wasm.set_show_recorded(!this.metaData.options.show_recorded);
    },
    toggle_pause: function () {
      this.wasm.set_paused(!this.metaData.options.paused);
    },
  },
};
</script>

<style scoped></style>
