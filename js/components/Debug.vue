<template>
  <div>
    <h3>Settings</h3>
    <br />
    <br />
    Saved Ticks: {{ metaData.saved_ticks.toFixed(0) }}
    <br />
    <button v-on:click="toggle_use_saved_ticks" style="margin: 2px">
      {{ !metaData.use_saved_ticks ? "Use Saved Ticks" : "Don't Use Saved Ticks" }}
    </button>
    <br />
    <button v-on:click="toggle_auto_work" style="margin: 2px">
      {{ !input.options.auto_work ? "Auto Work" : "Don't Auto Work" }}
    </button>
    <br />
    <button v-on:click="toggle_auto_living" style="margin: 2px">
      {{ !input.options.auto_living ? "Auto Living" : "Don't Auto Living" }}
    </button>
    <br />
    <button v-on:click="toggle_auto_buy_item" style="margin: 2px">
      {{ !input.options.auto_buy_item ? "Auto Buy Item" : "Don't Auto Buy Item" }}
    </button>
    <br />
    <button v-on:click="enable_tutorial">Enable Tutorial</button>
    <br />
    <button v-on:click="toggle_pause" style="margin: 2px">
      {{ $parent.paused ? "Resume the game" : "Pause the game" }}
    </button>
    <br />
    <button v-on:click="tick">Tick</button>
    <br />
    <br />
    <button v-on:click="export_save" style="margin: 2px">Export gamesave</button>
    <br />
    Import gamesave
    <p>TODO</p>

    <br /><br />
    <h3>Debug</h3>
    <button v-on:click="set_gamespeed(1)">Set GameSpeed 1</button>
    <br />
    <button v-on:click="set_gamespeed(10)">Set GameSpeed 10</button>
    <br />
    <button v-on:click="set_gamespeed(100)">Set GameSpeed 100</button>
    <br />
    <button v-on:click="set_gamespeed(1000)">Set GameSpeed 1000</button>
    <br />
    <br />
    <button v-on:click="print_debug_state">Print Debug State</button>
    <br />
    <button v-on:click="print_debug_meta">Print Debug Meta</button>
    <br />
    <br />
    Presets
    <div v-for="(value, name) in presets" :key="name">
      <button v-on:click="load_preset(name)" style="margin: 2px">{{ name }}</button>
    </div>
  </div>
</template>

<script>
export default {
  props: ["metaData", "state", "input", "wasm"],
  data() {
    return {
      presets: {},
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
      this.wasm.set_auto_work(!this.input.options.auto_work);
    },
    toggle_auto_living: function () {
      this.wasm.set_auto_living(!this.input.options.auto_living);
    },
    toggle_auto_buy_item: function () {
      this.wasm.set_auto_buy_item(!this.input.options.auto_buy_item);
    },
    print_debug_state: function () {
      this.wasm.print_debug_state();
    },
    print_debug_meta: function () {
      this.wasm.print_debug_meta();
    },
    set_gamespeed: function (game_speed) {
      this.wasm.set_gamespeed(game_speed);
    },
    tick: function (work_name) {
      this.wasm.single_tick();

      this.$parent.state = this.wasm.get_state();
      this.$parent.input = this.wasm.get_input();
    },
    load_preset: function (preset) {
      this.wasm.set_preset_saves(preset);
    },
    toggle_pause: function () {
      this.$parent.paused = !this.$parent.paused;
    },
    enable_tutorial: function () {
      this.wasm.set_disable_tutorial(false);
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
