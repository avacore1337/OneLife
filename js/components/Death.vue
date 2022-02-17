<template>
  <Section2>
    <h2 style="margin-bottom: 1rem">Death Section</h2>
    <div style="margin-bottom: 1rem">
      <b-button
        v-if="
          state.rebirth_stats.unlocks.can_end_early &&
          !state.life_stats.is_dying &&
          !state.life_stats.dead &&
          state.life_stats.current_tick >= 5000
        "
        @click="wasm.die"
      >
        End It Early
      </b-button>
      <b-button
        v-if="state.life_stats.is_dying && !state.life_stats.dead"
        style="margin: 2px"
        @click="wasm.die"
      >
        Go To The Other Side
      </b-button>
      <b-button v-if="state.life_stats.dead" @click="rebirth">Rebirth</b-button>
      <b-button v-if="state.life_stats.dead" @click="wasm.do_rebirth_replay">
        Rebirth Replay
      </b-button>
    </div>
    <table>
      <tr class="header-row">
        <th style="flex-grow: 2">Tier</th>
        <th style="flex-grow: 1">Cost</th>
      </tr>
      <tr
        v-for="tier in world.tiers.filter((tier) => tier.level > state.rebirth_stats.tier)"
        :key="tier.name"
        :class="{ disabled: !wasm.can_buy_tier(tier.level) }"
        @click="buy_tier(tier.level)"
      >
        <td>T{{ tier.level }} {{ tier.display_name }}</td>
        <td>
          <icon-with-text :icon="world.icons['Coin']">
            <FormatNumber :value="tier.purchasing_cost" />
          </icon-with-text>
        </td>
      </tr>
    </table>
  </Section2>
</template>

<script>
import Section2 from './Section2.vue'
import FormatNumber from './FormatNumber.vue'
export default {
  components: { Section2, FormatNumber },
  props: ['state', 'world', 'input', 'wasm'],
  methods: {
    rebirth() {
      console.log('rebirth tag sent')
      this.$gtag.event('rebirth', { method: 'normal' })
      this.wasm.do_rebirth()
    },
  },
}
</script>

<style scoped></style>
