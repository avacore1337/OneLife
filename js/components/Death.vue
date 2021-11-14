<template>
  <div>
    Death Section
    <button v-on:click="rebirth" style="margin: 2px">Rebirth</button>
    <div style="border: solid; margin: 2px">
      <ul>
        <li v-for="tier in world.tiers.filter((tier) => tier.level > state.rebirth_stats.class_tier)" :key="tier.name">
          <button v-on:click="buy_tier(tier.level)" style="margin: 2px" :disabled="!can_buy_tier(tier)">
            T{{ tier.level }} {{ tier.title }}: Cost {{ tier.purchasing_cost }}
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
  },
};
</script>

<style scoped></style>
