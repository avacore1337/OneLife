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
    <button v-on:click="toggle_auto_buy_tomb" style="margin: 2px">
      {{ !metaData.options.auto_buy_tomb ? "Auto Buy Tomb" : "Don't Auto Buy Tomb" }}
    </button>
    <br />
    <button v-on:click="wasm.set_disable_tutorial(false)">Enable Tutorial</button>
    <br />
    <button v-on:click="toggle_show_recorded" style="margin: 2px">
      {{ !$parent.show_recorded ? "Show Recorded" : "Don't Show Recorded" }}
    </button>
    <button v-on:click="toggle_pause" style="margin: 2px">
      {{ $parent.paused ? "Resume the game" : "Pause the game" }}
    </button>
    <br />
    <button v-on:click="tick">Tick</button>
    <br />
    <button v-on:click="wasm.test">Test</button>
    <br />
    <br />
    <button v-on:click="export_save" style="margin: 2px">Export gamesave</button>
    <br />
    Import gamesave
    <p>TODO</p>

    <br /><br />
    <h3>Debug</h3>
    <button v-on:click="wasm.set_gamespeed(1)">Set GameSpeed 1</button>
    <br />
    <button v-on:click="wasm.set_gamespeed(10)">Set GameSpeed 10</button>
    <br />
    <button v-on:click="wasm.set_gamespeed(100)">Set GameSpeed 100</button>
    <br />
    <button v-on:click="wasm.set_gamespeed(1000)">Set GameSpeed 1000</button>
    <br />
    <br />
    <button v-on:click="print_frontend_debug_state">Print Frontend Debug State</button>
    <br />
    <button v-on:click="wasm.print_debug_intermediate">Print Debug Intermediate</button>
    <br />
    <button v-on:click="wasm.print_debug_state">Print Debug State</button>
    <br />
    <button v-on:click="wasm.print_debug_meta">Print Debug Meta</button>
    <br />
    <br />
    <button v-on:click="wasm.grow_old">Grow Old</button>
    <br />
    <input v-model="money" size="10" />
    <br />
    <button v-on:click="wasm.give_money(money)">Give Money</button>
    <br />
    <input v-model="coins" size="10" />
    <br />
    <button v-on:click="wasm.give_coins(coins)">Give Coins</button>
    <br />
    <br />
    Presets
    <div v-for="(value, name) in presets" :key="name">
      <button v-on:click="wasm.set_preset_saves(name)" style="margin: 2px">{{ name }}</button>
    </div>
  </div>
</template>

<script>
export default {
  props: ["metaData", "state", "input", "wasm"],
  data() {
    return {
      presets: {},
      money: 1000000000.0,
      coins: 1000000.0,
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
    toggle_auto_buy_tomb: function () {
      this.wasm.set_auto_buy_tomb(!this.metaData.options.auto_buy_tomb);
    },
    print_frontend_debug_state: function () {
      console.log(this.state);
    },
    tick: function (work_name) {
      this.wasm.single_tick();

      this.$parent.state = this.wasm.get_state();
      this.$parent.input = this.wasm.get_input();
    },
    toggle_show_recorded: function () {
      this.$parent.show_recorded = !this.$parent.show_recorded;
    },
    toggle_pause: function () {
      this.$parent.paused = !this.$parent.paused;
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
