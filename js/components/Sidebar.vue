<template>
  <div style="margin-top: 1rem">
    <BaseStats />
    <Skills v-if="state.rebirth_stats.unlocks.has_skills" />

    <h4 class="section-header">Current Work</h4>
    <div v-for="[work, work_state] in current_work" :key="work.name" class="section">
      <MyProgressBar :value="work_state.next_level_percentage">
        <span style="display: flex; justify-content: space-between; width: 100%">
          <span>{{ work.display_name }} </span>
          <span> Level {{ work_state.level }} </span>
        </span>
      </MyProgressBar>
    </div>

    <h4 class="section-header">Currencies</h4>
    <div class="section column-flex">
      <icon-with-text v-b-tooltip.hover.right="money_tooltip" :icon="$world.icons['Money']">
        Money: <FormatNumber :value="state.items.money" />
      </icon-with-text>
      <span v-b-tooltip.hover.right="money_income_tooltip">
        Income: <FormatNumber :value="state.items.income" />/s
      </span>
      <icon-with-text
        v-if="state.rebirth_stats.unlocks.has_faith"
        v-b-tooltip.hover.right="divine_tooltip"
        :icon="$world.icons['DivineFavor']"
      >
        Divine Favor: <FormatNumber :value="state.items.divine_favor" />
      </icon-with-text>
      <span
        v-if="state.rebirth_stats.unlocks.has_faith"
        v-b-tooltip.hover.right="divine_income_tooltip"
      >
        Income: <FormatNumber :value="state.items.divine_favor_rate" />/s
      </span>
    </div>

    <h4 class="section-header">Life Stats</h4>
    <div class="section column-flex">
      <span v-b-tooltip.hover.right="age_tooltip"
        >Age: <FormatDays :value="state.life_stats.age"
      /></span>
      <span v-b-tooltip.hover.right="lifespan_tooltip">
        Lifespan: <FormatDays :value="state.life_stats.lifespan" />
      </span>
      <icon-with-text v-b-tooltip.hover.right="health_tooltip" :icon="$world.icons['Health']">
        Health: {{ state.life_stats.health.toFixed(2) }}
      </icon-with-text>
      <span v-b-tooltip.hover.right="health_rate_tooltip">
        Health Change:
        {{ state.life_stats.health_rate.toPrecision(2) }}/s
      </span>
      <icon-with-text v-b-tooltip.hover.right="happiness_tooltip" :icon="$world.icons['Happiness']">
        Happiness: {{ state.life_stats.happiness.toFixed(1) }}
      </icon-with-text>
      <span v-b-tooltip.hover.right="life_status_tooltip">Alive: {{ life_status() }}</span>
      <span v-b-tooltip.hover.right="current_tick_tooltip">
        Tick: <FormatNumber :value="state.life_stats.current_tick" />
      </span>
    </div>

    <h4 class="section-header">Rebirth Stats</h4>
    <div class="section column-flex">
      <span>Life number: {{ state.rebirth_stats.rebirth_count + 1 }}</span>
      <span v-b-tooltip.hover.right="tier_tooltip">
        Tier: {{ state.rebirth_stats.tier }}
        {{ $world.tiers[state.rebirth_stats.tier].display_name }}
      </span>
      <icon-with-text v-b-tooltip.hover.right="coin_tooltip" :icon="$world.icons['Coin']">
        Coins: <FormatNumber :value="state.rebirth_stats.coins" />
      </icon-with-text>
      <icon-with-text v-b-tooltip.hover.right="coin_income_tooltip" :icon="$world.icons['Coin']">
        Coins Gain: <FormatNumber :value="state.rebirth_stats.coins_gain" />
      </icon-with-text>
      <div v-if="false">
        <span>Karma: {{ state.rebirth_stats.karma }}</span>
        <span>Time multiplier: {{ state.rebirth_stats.time_factor }}</span>
      </div>
    </div>
  </div>
</template>

<script>
import BaseStats from './BaseStats.vue'
import MyProgressBar from './MyProgressBar.vue'
import Skills from './Skills.vue'
import FormatNumber from './FormatNumber.vue'
import FormatDays from './FormatDays.vue'
import { mapState } from 'vuex'

export default {
  components: { BaseStats, Skills, FormatNumber, FormatDays, MyProgressBar },
  data() {
    return {
      money_tooltip: 'The amount of money you have to spend on housing/items/tombs',
      money_income_tooltip: 'Your current effective income per second (net income - housing costs)',
      divine_tooltip: 'The amount of divine favor you have to spend up blessings',
      divine_income_tooltip: 'Divine income gained per second',
      age_tooltip: 'Your current age, you will die when this value is equal to your Lifespan',
      lifespan_tooltip:
        'Your current maximum age, a value between 0 and 140 depending on your health',
      health_tooltip: 'Your current health value, a value between -1 and 1',
      health_rate_tooltip:
        'How much your health is changing per second. \nDepends on both health modifier and current health',
      coin_tooltip: 'The main currency used for progressing between rebirths',
      coin_income_tooltip: 'How many coins you would get if you rebirthed right now',
      happiness_tooltip: 'Happiness boosts gained work XP',
      current_tick_tooltip: 'The amount of ticks since this rebirth started',
      life_status_tooltip: 'If you are currently alive, dead or dying',
      tier_tooltip:
        'What your current tier is. This is the main progression in the game and decides starting stats and what can/can not be bought',
    }
  },

  computed: {
    ...mapState(['state', 'meta', 'input']),
    current_work() {
      let self = this
      let the_work = []
      this.$world.works.forEach((work, i) => {
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

<style scoped>
.column-flex {
  display: flex;
  flex-direction: column;
  /* align-items: start; */
  gap: 5px;
}
</style>