<template>
  <div>
    Death Section
    <br />
    <button
      v-on:click="die"
      v-if="state.rebirth_stats.unlocks.can_end_early && !state.life_stats.is_dying && !state.life_stats.dead"
      style="margin: 2px"
    >
      End It Early
    </button>
    <button v-on:click="die" v-if="state.life_stats.is_dying && !state.life_stats.dead" style="margin: 2px">
      Go To The Other Side
    </button>
    <button v-on:click="rebirth" v-if="state.life_stats.dead" style="margin: 2px">Rebirth</button>
    <button v-on:click="rebirth_replay" v-if="state.life_stats.dead" style="margin: 2px">Rebirth Replay</button>
    <div style="border: solid; margin: 2px">
      <ul>
        <li v-for="tier in world.tiers.filter((tier) => tier.level > state.rebirth_stats.class_tier)" :key="tier.name">
          <button v-on:click="buy_tier(tier.level)" style="margin: 2px" :disabled="!can_buy_tier(tier)">
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
    can_buy_tier: function (tier) {
      return this.wasm.can_buy_tier(tier.level);
    },
    buy_tier: function (index) {
      this.wasm.buy_tier(index);
    },
    rebirth: function () {
      this.wasm.do_rebirth();
    },
    rebirth_replay: function () {
      this.wasm.do_rebirth_replay();
    },
    die: function () {
      this.wasm.die();
    },
  },
};
</script>

<style scoped></style>
