<template>
  <div style="border: solid; margin: 2px">
    <span>
      Show bought upgrades
      <input
        type="checkbox"
        id="show_bought"
        v-on:click="toggle_show_bought"
        :checked="metaData.options.show_bought_upgrades"
      />
    </span>
    <br />
    <span
      v-if="
        metaData.options.show_bought_upgrades &&
        state.rebirth_stats.rebirth_upgrades.some((upgrade) => upgrade.is_purchased)
      "
    >
      Bought Rebirth Upgrades
      <ul>
        <li v-for="[upgrade, upgrade_state] in bought_upgrades" :key="upgrade.name">
          {{ upgrade.display_name }}
        </li>
      </ul>
    </span>
    Rebirth Upgrades
    <ul>
      <li v-for="[upgrade, upgrade_state] in visible_unbought_upgrades" :key="upgrade.name">
        <button
          v-on:click="buy_rebirth_upgrade(upgrade.name)"
          style="margin: 2px"
          :disabled="!upgrade_state.is_unlocked"
        >
          {{ upgrade.display_name }}
          {{ upgrade.purchasing_cost }}
        </button>
      </li>
    </ul>
  </div>
</template>

<script>
export default {
  props: ["state", "world", "input", "wasm", "metaData"],
  methods: {
    buy_rebirth_upgrade: function (rebirth_upgrade_name) {
      this.wasm.buy_rebirth_upgrade(rebirth_upgrade_name);
    },
    toggle_show_bought: function () {
      this.wasm.set_show_bought_upgrades(!this.metaData.options.show_bought_upgrades);
    },
  },
  computed: {
    visible_unbought_upgrades: function () {
      let self = this;
      return self.world.rebirth_upgrades
        .map((w, i) => {
          return [w, self.state.rebirth_stats.rebirth_upgrades[i]];
        })
        .filter(([w, s]) => {
          return s.is_visible && !s.is_purchased;
        });
    },
    bought_upgrades: function () {
      let self = this;
      return self.world.rebirth_upgrades
        .map((w, i) => {
          return [w, self.state.rebirth_stats.rebirth_upgrades[i]];
        })
        .filter(([w, s]) => {
          return s.is_purchased;
        });
    },
  },
};
</script>

<style scoped></style>
