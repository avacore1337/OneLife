<template>
  <div style="border: solid; margin: 2px">
    Bought Rebirth Upgrades
    <ul>
      <li
        v-for="(rebirth_upgrade, index) in world.rebirth_upgrades"
        v-if="
          state.rebirth_stats.rebirth_upgrades[index].is_visible &&
          state.rebirth_stats.rebirth_upgrades[index].is_purchased
        "
        :key="rebirth_upgrade.name"
      >
        {{ rebirth_upgrade.display_name }}
      </li>
    </ul>
    Rebirth Upgrades
    <ul>
      <li
        v-for="(rebirth_upgrade, index) in world.rebirth_upgrades"
        v-if="
          state.rebirth_stats.rebirth_upgrades[index].is_visible &&
          !state.rebirth_stats.rebirth_upgrades[index].is_purchased
        "
        :key="rebirth_upgrade.name"
      >
        <button
          v-on:click="buy_rebirth_upgrade(rebirth_upgrade.name)"
          style="margin: 2px"
          :disabled="!state.rebirth_stats.rebirth_upgrades[index].is_unlocked"
        >
          {{ rebirth_upgrade.display_name }}
          {{ rebirth_upgrade.purchasing_cost }}
        </button>
      </li>
    </ul>
  </div>
</template>

<script>
export default {
  props: ["state", "world", "input", "wasm"],
  methods: {
    buy_rebirth_upgrade: function (rebirth_upgrade_name) {
      this.wasm.buy_rebirth_upgrade(rebirth_upgrade_name);
    },
  },
};
</script>

<style scoped></style>
