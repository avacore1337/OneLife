<template>
  <div>
    Death Section
    <br />
    <button
      v-if="
        state.rebirth_stats.unlocks.can_end_early &&
        !state.life_stats.is_dying &&
        !state.life_stats.dead &&
        state.life_stats.current_tick >= 5000
      "
      style="margin: 2px"
      @click="wasm.die"
    >
      End It Early
    </button>
    <button
      v-if="state.life_stats.is_dying && !state.life_stats.dead"
      style="margin: 2px"
      @click="wasm.die"
    >
      Go To The Other Side
    </button>
    <button v-if="state.life_stats.dead" style="margin: 2px" @click="rebirth">Rebirth</button>
    <button v-if="state.life_stats.dead" style="margin: 2px" @click="wasm.do_rebirth_replay">
      Rebirth Replay
    </button>
    <div style="border: solid; margin: 2px">
      <ul>
        <li
          v-for="tier in world.tiers.filter((tier) => tier.level > state.rebirth_stats.tier)"
          :key="tier.name"
        >
          <button
            style="margin: 2px"
            :disabled="!wasm.can_buy_tier(tier.level)"
            @click="wasm.buy_tier(tier.level)"
          >
            T{{ tier.level }} {{ tier.display_name }}: Cost
            <FormatNumber :value="tier.purchasing_cost" />
          </button>
        </li>
      </ul>
    </div>
  </div>
</template>

<script>
import FormatNumber from './FormatNumber.vue'
export default {
  components: { FormatNumber },
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
