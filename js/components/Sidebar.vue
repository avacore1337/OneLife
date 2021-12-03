<template>
  <div style="border: solid; margin: 2px">
    <BaseStats v-bind:state="state" v-bind:input="input" v-bind:world="world" v-bind:wasm="wasm" />

    <br />
    Items
    <div style="border: solid; margin: 2px; padding: 10px">
      <p>Money: {{ printableNumbers(state.items.money) }} Income: {{ printableNumbers(state.items.income) }}</p>
    </div>

    <br />
    Life Stats
    <div style="border: solid; margin: 2px; padding: 10px">
      <p>Age: {{ prettyPrintDays(state.life_stats.age) }}</p>
      <p>Lifespan: {{ prettyPrintDays(state.life_stats.lifespan) }}</p>
      <p>Health: {{ state.life_stats.health.toFixed(2) }}</p>
      <p>Happiness: {{ state.life_stats.happiness }}</p>
      <p>Alive: {{ life_status() }}</p>
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
  methods: {
    life_status: function () {
      if (this.state.life_stats.dead) {
        return "No ;(";
      }
      if (this.state.life_stats.is_dying) {
        return "Soon dead...";
      }
      return "Yes :)";
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

      if (this.$parent.numberFormat === "SCIENTIFIC") {
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

<style scoped></style>
