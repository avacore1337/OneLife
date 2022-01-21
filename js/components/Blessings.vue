<template>
  <div style="border: solid; margin: 2px">
    Blessings
    <ul>
      <li v-for="[blessing, blessing_state] in visible_blessings" :key="blessing.name">
        <button v-on:click="wasm.buy_blessing(blessing.name)" :disabled="!blessing_state.is_unlocked">
          {{ blessing.display_name }}
        </button>

        <span style="float: right">Cost: {{ blessing_state.next_level_cost }} Divine Favor </span>

        <br />
        Level: {{ blessing_state.level }}
      </li>
    </ul>
  </div>
</template>

<script>
export default {
  props: ["state", "world", "input", "wasm"],
  methods: {},
  computed: {
    visible_blessings: function () {
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
};
</script>

<style scoped></style>
