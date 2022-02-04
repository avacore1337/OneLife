<template>
  <div style="border: solid; margin: 2px">
    Blessings
    <b-button v-if="state.rebirth_stats.unlocks.can_auto_buy_blessing" size="sm" @click="toggle_auto_buy_blessing">
      {{ !metaData.options.auto_buy_blessing ? "Auto Buy Blessing" : "Don't Auto Buy Blessing" }}
    </b-button>
    <ul>
      <li v-for="[blessing, blessing_state] in visible_blessings" :key="blessing.name">
        <my-icon :icon="blessing.icon" />
        <button :disabled="!blessing_state.is_unlocked" @click="wasm.buy_blessing(blessing.name)">
          {{ blessing.display_name }}
        </button>

        <span style="float: right">Cost: {{ blessing_state.next_level_cost }} Divine Favor </span>

        <br />
        Level: {{ blessing_state.level }}, Effect: {{ blessing_state.effect_description }}
      </li>
    </ul>
  </div>
</template>

<script>
export default {
  props: ["metaData", "state", "world", "input", "wasm"],
  computed: {
    visible_blessings() {
      let self = this;
      return self.world.blessings
        .map((w, i) => {
          return [w, self.state.blessings[i]];
        })
        .filter(([w, s]) => {
          return s.is_visible;
        });
    },
  },
  methods: {
    toggle_auto_buy_blessing() {
      this.wasm.set_auto_buy_blessing(!this.metaData.options.auto_buy_blessing);
    },
  },
};
</script>

<style scoped></style>
