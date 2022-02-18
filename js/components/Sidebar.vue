<template>
  <div style="border: solid; margin: 2px">
    <BaseStats :state="state" :input="input" :world="world" :wasm="wasm" />
    <Skills
      v-if="state.rebirth_stats.unlocks.has_skills"
      :state="state"
      :input="input"
      :world="world"
      :wasm="wasm"
    />

    <br />
    Current Work
    <div
      v-for="[work, work_state] in current_work"
      :key="work.name"
      style="border: solid; margin: 2px; padding: 10px"
      v-b-tooltip.hover.right="'tooltip todo'"
    >
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
        <icon-with-text :icon="world.icons['Money']" v-b-tooltip.hover.right="'tooltip todo'">
          Money: <FormatNumber :value="state.items.money" />
        </icon-with-text>
        <br />
        <span v-b-tooltip.hover.right="'tooltip todo'">
          Income: <FormatNumber :value="state.items.income" />/s
        </span>
      </p>
      <p v-if="state.rebirth_stats.unlocks.has_faith">
        <icon-with-text :icon="world.icons['DivineFavor']" v-b-tooltip.hover.right="'tooltip todo'">
          Divine Favor: <FormatNumber :value="state.items.divine_favor" />
        </icon-with-text>
        <br />
        Income: <FormatNumber :value="state.items.divine_favor_rate" />/s
      </p>
    </div>

    <br />
    Life Stats
    <div style="border: solid; margin: 2px; padding: 10px">
      <p v-b-tooltip.hover.right="'tooltip todo'">
        Age: <FormatDays :value="state.life_stats.age" />
      </p>
      <p v-b-tooltip.hover.right="'tooltip todo'">
        Lifespan: <FormatDays :value="state.life_stats.lifespan" />
      </p>
      <p>
        <icon-with-text :icon="world.icons['Health']" v-b-tooltip.hover.right="'tooltip todo'">
          Health: {{ state.life_stats.health.toFixed(2) }}
        </icon-with-text>
        Rate:
        {{ state.life_stats.health_rate.toPrecision(2) }}/s
      </p>
      <p>
        <icon-with-text
          :icon="world.icons['Happiness']"
          v-b-tooltip.hover.right="'Happiness boosts gained work XP'"
        >
          Happiness: {{ state.life_stats.happiness.toFixed(1) }}
        </icon-with-text>
      </p>
      <p v-b-tooltip.hover.right="'If you are currently alive, dead or dying'">
        Alive: {{ life_status() }}
      </p>
      <p v-b-tooltip.hover.right="'Happiness boosts gained work XP'">
        Tick: <FormatNumber :value="state.life_stats.current_tick" />
      </p>
    </div>

    <br />
    Rebirth Stats
    <div style="border: solid; margin: 2px; padding: 10px">
      <p>Life number: {{ state.rebirth_stats.rebirth_count + 1 }}</p>
      <p
        v-b-tooltip.hover.right="
          'What your current tier is. This is the main progression in the game and decides starting stats and what can/can not be bought'
        "
      >
        Tier: {{ state.rebirth_stats.tier }}
        {{ world.tiers[state.rebirth_stats.tier].display_name }}
      </p>
      <p>
        <icon-with-text
          :icon="world.icons['Coin']"
          v-b-tooltip.hover.right="'The main currency used for progressing between rebirths'"
        >
          Coins: <FormatNumber :value="state.rebirth_stats.coins" />
        </icon-with-text>
      </p>
      <p>
        <icon-with-text
          :icon="world.icons['Coin']"
          v-b-tooltip.hover.right="'How many coins you would get if you rebirthed right now'"
        >
          Coins Gain: <FormatNumber :value="state.rebirth_stats.coins_gain" />
        </icon-with-text>
      </p>
      <div v-if="false">
        <p>Karma: {{ state.rebirth_stats.karma }}</p>
        <p>Time multiplier: {{ state.rebirth_stats.time_factor }}</p>
      </div>
    </div>
  </div>
</template>

<script>
import BaseStats from './BaseStats.vue'
import Skills from './Skills.vue'
import ProgressBar from './ProgressBar.vue'
import FormatNumber from './FormatNumber.vue'
import FormatDays from './FormatDays.vue'
export default {
  components: { ProgressBar, BaseStats, Skills, FormatNumber, FormatDays },
  props: ['state', 'world', 'input', 'wasm', 'metaData'],

  computed: {
    current_work() {
      let self = this
      let the_work = []
      this.world.works.forEach((work, i) => {
        if (work.name == self.input.work) {
          the_work = [[work, self.state.works[i]]]
        }
      })
      return the_work
    },
  },
  methods: {
    life_status() {
      if (this.state.life_stats.dead) {
        return 'No ;('
      }
      if (this.state.life_stats.is_dying) {
        return 'Soon dead...'
      }
      return 'Yes :)'
    },
    prettyPrintDays(total_days) {
      const years = Math.floor(total_days / 365)
      const days = total_days % 365

      if (years === 0) {
        return `${days} days`
      } else if (days === 0) {
        return `${years} years`
      }
      return `${years} years and ${days.toFixed(0)} days`
    },
  },
}
</script>

<style scoped></style>
