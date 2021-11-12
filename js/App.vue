<template>
  <div>
    <div style="width: 300px; float: left">
      <button v-on:click="save">Save</button>
      <button v-on:click="load">Load</button>
      <button v-on:click="hard_reset">Hard Reset</button>
      <button v-on:click="setNumberFormat">{{ nextNumberFormat(numberFormat) }}</button>

      <br />
      Base stats
      <div style="border: solid; margin: 2px; padding: 10px">
        <p>Strength: {{ state.base_stats.str }}</p>
        <p>Intelligence: {{ state.base_stats.int }}</p>
        <p>Charisma: {{ state.base_stats.cha }}</p>
        <p>Constitution: {{ state.base_stats.con }}</p>
        <p>Dexterity: {{ state.base_stats.dex }}</p>
        <p>Faith: {{ state.base_stats.faith }}</p>
      </div>

      <br />
      Items
      <div style="border: solid; margin: 2px; padding: 10px">
        <p>Money: {{ printableNumbers(state.items.money) }}</p>
      </div>

      <br />
      Life Stats
      <div style="border: solid; margin: 2px; padding: 10px">
        <p>Age: {{ prettyPrintDays(state.life_stats.age) }}</p>
        <p>Lifespan: {{ prettyPrintDays(state.life_stats.lifespan) }}</p>
        <p>Health: {{ state.life_stats.health }}</p>
        <p>Happiness: {{ state.life_stats.happiness }}</p>
        <p>Alive: {{ state.life_stats.dead ? "No ;(" : "Yes :)" }}</p>
      </div>

      <br />
      Rebirth Stats
      <div style="border: solid; margin: 2px; padding: 10px">
        <p>Life number {{ state.rebirth_stats.rebirth_count + 1 }}</p>
        <p>Class: {{ state.rebirth_stats.class_tier }}</p>
        <p>Coins: {{ state.rebirth_stats.coins }}</p>
        <p>Karma: {{ state.rebirth_stats.karma }}</p>
        <p>Time multiplier: {{ state.rebirth_stats.time_factor }}</p>
      </div>
    </div>

    <div style="margin-left: 420px">
      <div style="float: left">
        <div style="border: solid; width: 400px; float: left">
          <Works v-bind:state="state" v-bind:input="input" v-bind:world="world" v-bind:wasm="wasm" />

          <div style="border: solid; margin: 2px">
            Housing
            <ul>
              <li v-for="housing in world.housing" :key="housing.name">
                <button v-on:click="set_housing(housing.name)" style="margin: 2px">
                  <span v-if="housing.name == input.housing">{{ housing.name }} &lt;-- </span>
                  <span v-if="housing.name != input.housing">{{ housing.name }}</span>
                </button>
              </li>
            </ul>
          </div>
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

import Vue from "vue/dist/vue.js";
import { BootstrapVue } from "bootstrap-vue";
import { downloadFile } from "./utility.js";

import "bootstrap/dist/css/bootstrap.css";
import "bootstrap-vue/dist/bootstrap-vue.css";

Vue.use(BootstrapVue);

export default {
  props: ["wasm"],
  components: { Works },
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
      numberFormat: "DEFAULT",
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
    printableNumbers: function (num) {
      if (num === undefined) {
        return null;
      }

      if (num < 100) {
        return num.toFixed(1);
      }
      if (num < 10000) {
        return Math.floor(num).toString();
      }

      if (this.numberFormat === "SCIENTIFIC") {
        let exponent = 1;
        while (num >= 10) {
          num /= 10;
          exponent++;
        }

        return `${num.toFixed(1)}e${exponent}`;
      }

      const ending = ["K", "M", "B", "T", "Qa", "Qi", "He", "Se", "Oc", "No", "De"];
      let index = -1;
      while (num >= 10000 && index < ending.length - 1) {
        num /= 1000;
        index++;
      }

      return `${num.toFixed(1)}${ending[index]}`;
    },
    nextNumberFormat: function (numberFormat) {
      return {
        DEFAULT: "Scientific notation",
        SCIENTIFIC: "Natural numbers",
      }[numberFormat];
    },
    setNumberFormat: function () {
      this.numberFormat = {
        DEFAULT: "SCIENTIFIC",
        SCIENTIFIC: "DEFAULT",
      }[this.numberFormat];
    },
    save: function () {
      this.wasm.save();
    },
    print_debug: function () {
      this.wasm.print_debug();
    },
    hard_reset: function () {
      this.wasm.hard_reset();
    },
    load: function () {
      this.wasm.load();
    },
    set_gamespeed: function (game_speed) {
      this.wasm.set_gamespeed(game_speed);
    },
    tick: function (work_name) {
      this.wasm.single_tick();

      this.state = this.wasm.get_state();
      this.input = this.wasm.get_input();
    },
    set_housing: function (housing_name) {
      this.wasm.set_housing(housing_name);
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
    prettyPrintDays: function (total_days) {
      const years = Math.floor(total_days / 365);
      const days = total_days % 365;

      if (years === 0) {
        return `${days} days`;
      } else if (days === 0) {
        return `${years} years`;
      }
      return `${years} years and ${days.toFixed(0)} days`;
    },
    prettyPrint: function (value) {
      if (typeof value !== "number") {
        return value;
      }

      return this.printableNumbers(value);
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
