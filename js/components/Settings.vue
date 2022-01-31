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
    <button v-on:click="wasm.set_disable_tutorial(false)">Enable Tutorial</button>
    <br />
    <button v-on:click="toggle_show_recorded" style="margin: 2px">
      {{ !$parent.show_recorded ? "Show Recorded" : "Don't Show Recorded" }}
    </button>
    <button v-on:click="toggle_pause" style="margin: 2px">
      {{ $parent.$parent.paused ? "Resume the game" : "Pause the game" }}
    </button>
    <br />
    <br />
    <button v-on:click="export_save" style="margin: 2px">Export gamesave</button>
    <br />
    Import gamesave
    <p>TODO</p>
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
    toggle_show_recorded: function () {
      this.$parent.$parent.show_recorded = !this.$parent.$parent.show_recorded;
    },
    toggle_pause: function () {
      this.$parent.$parent.paused = !this.$parent.$parent.paused;
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

<style scoped></style>
