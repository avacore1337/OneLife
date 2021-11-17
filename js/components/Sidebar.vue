<template>
  <div>
    <button v-on:click="save">Save</button>
    <button v-on:click="load">Load</button>
    <button v-on:click="hard_reset">Hard Reset</button>
    <button v-on:click="setNumberFormat">{{ nextNumberFormat(numberFormat) }}</button>
    <input type="checkbox" id="autosave" v-on:click="toggleAutoSave" :checked="metaData.autosave" />
    <label for="autosave">Autosave</label>

    <br />
    <BaseStats v-bind:state="state" v-bind:input="input" v-bind:world="world" v-bind:wasm="wasm" />

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
</template>

<script>
import BaseStats from "./BaseStats.vue";
export default {
  props: ["state", "world", "input", "wasm", "metaData"],
  components: { BaseStats },
  data() {
    return {
      numberFormat: "DEFAULT",
    };
  },
  methods: {
    toggleAutoSave: function () {
      this.wasm.set_autosave(!this.metaData.autosave);
    },
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
    save: function () {
      this.wasm.save();
    },
    load: function () {
      this.wasm.load();
    },
    hard_reset: function () {
      this.wasm.hard_reset();
    },
  },
};
</script>

<style scoped></style>