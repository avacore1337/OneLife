<template>
  <div style="border: solid; margin: 2px">
    <BaseStats :state="state" :input="input" :world="world" :wasm="wasm" />
    <Skills v-if="state.rebirth_stats.unlocks.has_skills" :state="state" :input="input" :world="world" :wasm="wasm" />

    <br />
    Current Work
    <div v-for="[work, work_state] in current_work" style="border: solid; margin: 2px; padding: 10px">
      {{ work.display_name }}
      {{ work_state.level }}
      <br />
      {{ work_state.effective_income.toFixed(1) }}/s
      <ProgressBar :value="work_state.next_level_percentage" :decimal-points="2" />
    </div>
    <br />
    Currencies
    <div style="border: solid; margin: 2px; padding: 10px">
      <p>
        <my-icon :icon="world.icons['Money']" />
        Money: {{ printableNumbers(state.items.money) }} Income:
        {{ printableNumbers(state.items.income) }}
      </p>
      <p v-if="state.rebirth_stats.unlocks.has_faith">
        <my-icon :icon="world.icons['DivineFavor']" />
        Divine Favor: {{ printableNumbers(state.items.divine_favor) }} Rate:
        {{ printableNumbers(state.items.divine_favor_rate) }}
      </p>
    </div>

    <br />
    Life Stats
    <div style="border: solid; margin: 2px; padding: 10px">
      <p>Age: {{ prettyPrintDays(state.life_stats.age) }}</p>
      <p>Lifespan: {{ prettyPrintDays(state.life_stats.lifespan) }}</p>
      <p>
        <my-icon :icon="world.icons['Health']" />
        Health: {{ state.life_stats.health.toFixed(2) }} Rate: {{ state.life_stats.health_rate.toPrecision(2) }}/s
      </p>
      <p>
        <my-icon :icon="world.icons['Happiness']" />
        Happiness: {{ state.life_stats.happiness.toFixed(1) }}
      </p>
      <p>Alive: {{ life_status() }}</p>
      <p>Tick: {{ state.life_stats.current_tick }}</p>
    </div>

    <br />
    Rebirth Stats
    <div style="border: solid; margin: 2px; padding: 10px">
      <p>Life number: {{ state.rebirth_stats.rebirth_count + 1 }}</p>
      <p>
        Tier: {{ state.rebirth_stats.tier }}
        {{ world.tiers[state.rebirth_stats.tier].display_name }}
      </p>
      <p>
        <my-icon :icon="world.icons['Coin']" />
        Coins: {{ printableNumbers(state.rebirth_stats.coins) }}
      </p>
      <p>
        <my-icon :icon="world.icons['Coin']" />
        Coins Gain: {{ printableNumbers(state.rebirth_stats.coins_gain) }}
      </p>
      <div v-if="false">
        <p>Karma: {{ state.rebirth_stats.karma }}</p>
        <p>Time multiplier: {{ state.rebirth_stats.time_factor }}</p>
      </div>
    </div>
  </div>
</template>

<script>
import BaseStats from "./BaseStats.vue";
import Skills from "./Skills.vue";
import ProgressBar from "./ProgressBar.vue";
export default {
  components: { ProgressBar, BaseStats, Skills },
  props: ["state", "world", "input", "wasm", "metaData"],

  computed: {
    current_work: function () {
      let self = this;
      let the_work = [];
      this.world.works.forEach((work, i) => {
        if (work.name == self.input.work) {
          the_work = [[work, self.state.works[i]]];
        }
      });
      return the_work;
    },
  },
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
