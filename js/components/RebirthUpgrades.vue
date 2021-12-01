<template>
  <div style="border: solid; margin: 2px">
    Rebirth Upgrades
    <ul>
      <li
        v-for="(rebirth_upgrade, index) in world.rebirth_upgrades.filter(
          (rebirth_upgrade, index) => state.rebirth_stats.rebirth_upgrades[index].is_visible
        )"
        v-on:click="
          state.rebirth_stats.rebirth_upgrades[index].is_unlocked && set_rebirth_upgrade(rebirth_upgrade.name)
        "
        v-bind:class="{ disabled: !state.rebirth_stats.rebirth_upgrades[index].is_unlocked }"
        :key="rebirth_upgrade.name"
      >
        <button v-on:click="buy_rebirth_upgrade(rebirth_upgrade.name)" style="margin: 2px">
          <span v-if="rebirth_upgrade.name == input.rebirth_upgrade">{{ rebirth_upgrade.display_name }} &lt;-- </span>
          <span v-if="rebirth_upgrade.name != input.rebirth_upgrade">{{ rebirth_upgrade.display_name }}</span>
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
