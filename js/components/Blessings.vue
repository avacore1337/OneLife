<template>
  <div style="border: solid; margin: 2px">
    Bought Blessings
    <ul>
      <li v-for="[blessing, blessing_state] in bought_blessings" :key="blessing.name">
        <p>{{ blessing.display_name }}</p>
      </li>
    </ul>
    Blessings
    <ul>
      <li v-for="[blessing, blessing_state] in visible_unbought_blessings" :key="blessing.name">
        <button v-on:click="buyBlessing(blessing.name)" :disabled="!blessing_state.is_unlocked">
          {{ blessing.display_name }}
        </button>
        {{ blessing.purchasing_cost }}
      </li>
    </ul>
  </div>
</template>

<script>
export default {
  props: ["state", "world", "input", "wasm"],
  methods: {
    buyBlessing: function (blessing_name) {
      this.wasm.buy_blessing(blessing_name);
    },
  },
  computed: {
    bought_blessings: function () {
      let self = this;
      return self.world.blessings
        .map((w, i) => {
          return [w, self.state.blessings[i]];
        })
        .filter(([w, s]) => {
          return s.is_purchased;
        });
    },
    visible_unbought_blessings: function () {
      let self = this;
      return self.world.blessings
        .map((w, i) => {
          return [w, self.state.blessings[i]];
        })
        .filter(([w, s]) => {
          return s.is_visible && !s.is_purchased;
        });
    },
  },
};
</script>

<style scoped></style>
