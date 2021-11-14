<template>
  <div>
    Debug

    <br /><br />
    <button v-on:click="set_gamespeed(1)">Set GameSpeed 1</button>
    <br />
    <button v-on:click="set_gamespeed(100)">Set GameSpeed 100</button>
    <br />
    <button v-on:click="toggle_pause" style="margin: 2px">
      {{ $parent.paused ? "Resume the game" : "Pause the game" }}
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
</template>

<script>
export default {
  props: ["state", "presets", "input", "wasm"],
  methods: {
    print_debug: function () {
      this.wasm.print_debug();
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
    set_autosave: function (autosave) {
      this.wasm.set_autosave(autosave);
    },
    set_autosave: function (autosave) {
      this.wasm.set_autosave(autosave);
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
