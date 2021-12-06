<template>
  <div style="border: solid; margin: 2px">
    Bought Blessings
    <ul>
      <li v-for="(blessing, index) in world.blessings" v-if="state.blessings[index].is_purchased" :key="blessing.name">
        <p>{{ blessing.display_name }}</p>
      </li>
    </ul>
    Blessings
    <ul>
      <li
        v-for="(blessing, index) in world.blessings"
        v-if="!state.blessings[index].is_purchased && state.blessings[index].is_visible"
        :key="blessing.name"
      >
        <button v-on:click="buyBlessing(blessing.name)" :disabled="!state.blessings[index].is_unlocked">
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
};
</script>

<style scoped></style>
