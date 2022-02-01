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
      {{ !metaData.options.show_recorded ? "Show Recorded" : "Don't Show Recorded" }}
    </button>
    <br />
    <button v-on:click="toggle_pause" style="margin: 2px">
      {{ $parent.$parent.paused ? "Resume the game" : "Pause the game" }}
    </button>
    <br />
    <br />
    <button v-on:click="download_save" style="margin: 2px">Download Save</button>
    <br />
    <b-form-textarea
      id="textarea"
      v-model="save_text"
      placeholder="Paste save here"
      rows="6"
      max-rows="6"
    ></b-form-textarea>
    <button v-on:click="import_save" style="margin: 2px">Import Save</button>
    <button v-on:click="export_save" style="margin: 2px">Export Save</button>
    <br />
    <button v-on:click="wasm.save">Save</button>
    <br />
    <button v-on:click="wasm.load">Load</button>
    <br />
    <button v-on:click="wasm.hard_reset">Hard Reset</button>
    <br />
    <input type="checkbox" id="autosave" v-on:click="toggleAutoSave" :checked="metaData.autosave" />
    <label for="autosave">Autosave</label>
    <br />
    <button v-on:click="setNumberFormat">{{ nextNumberFormat($parent.$parent.numberFormat) }}</button>
  </div>
</template>

<script>
import { downloadFile } from "../utility.js";
export default {
  props: ["metaData", "state", "world", "input", "wasm"],
  data() {
    return {
      save_text: "",
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
      this.wasm.set_show_recorded(!this.metaData.options.show_recorded);
    },
    toggle_pause: function () {
      this.$parent.$parent.paused = !this.$parent.$parent.paused;
    },
    download_save: function () {
      // TODO: This should be exported by the backend
      downloadFile(`gamesave_${Date.now()}.txt`, this.wasm.export_save());
    },
    import_save_file: function (event) {
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
    import_save: function () {
      this.wasm.import_save(this.save_text);
    },
    export_save: function () {
      this.save_text = this.wasm.export_save();
    },
    toggleAutoSave: function () {
      this.wasm.set_autosave(!this.metaData.autosave);
    },
    nextNumberFormat: function (numberFormat) {
      return {
        DEFAULT: "Scientific notation",
        SCIENTIFIC: "Natural numbers",
      }[numberFormat];
    },
    setNumberFormat: function () {
      this.$parent.numberFormat = {
        DEFAULT: "SCIENTIFIC",
        SCIENTIFIC: "DEFAULT",
      }[this.$parent.numberFormat];
    },
  },
};
</script>

<style scoped></style>
