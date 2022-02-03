<template>
  <div>
    Death Section
    <br />
    <button
      v-on:click="wasm.die"
      v-if="
        state.rebirth_stats.unlocks.can_end_early &&
        !state.life_stats.is_dying &&
        !state.life_stats.dead &&
        state.life_stats.current_tick >= 5000
      "
      style="margin: 2px"
    >
      End It Early
    </button>
    <button v-on:click="wasm.die" v-if="state.life_stats.is_dying && !state.life_stats.dead" style="margin: 2px">
      Go To The Other Side
    </button>
    <button v-on:click="rebirth" v-if="state.life_stats.dead" style="margin: 2px">Rebirth</button>
    <button v-on:click="wasm.do_rebirth_replay" v-if="state.life_stats.dead" style="margin: 2px">Rebirth Replay</button>
    <div style="border: solid; margin: 2px">
      <ul>
        <li v-for="tier in world.tiers.filter((tier) => tier.level > state.rebirth_stats.tier)" :key="tier.name">
          <button v-on:click="wasm.buy_tier(tier.level)" style="margin: 2px" :disabled="!wasm.can_buy_tier(tier.level)">
            T{{ tier.level }} {{ tier.display_name }}: Cost {{ tier.purchasing_cost }}
          </button>
        </li>
      </ul>
    </div>
  </div>
</template>

<script>
export default {
  props: ["state", "world", "input", "wasm"],
  methods: {
    rebirth: function () {
      console.log("rebirth tag sent");
      this.$gtag.event("rebirth", { method: "normal" });
      this.wasm.do_rebirth();
    },
  },
};
</script>

<style scoped></style>
