<template>
  <div style="border: solid; margin: 2px">
    Bought Tombs
    <ul>
      <li
        v-for="(tomb, index) in world.tombs"
        v-if="state.tombs[index].is_visible && state.tombs[index].is_purchased"
        :key="tomb.name"
      >
        {{ tomb.display_name }}
      </li>
    </ul>
    Tombs
    <ul>
      <li
        v-for="(tomb, index) in world.tombs"
        v-if="state.tombs[index].is_visible && !state.tombs[index].is_purchased"
        :key="tomb.name"
      >
        <button v-on:click="buyTomb(tomb.name)" style="margin: 2px" :disabled="!state.tombs[index].is_unlocked">
          {{ tomb.display_name }}
        </button>
        {{ tomb.purchasing_cost }}
      </li>
    </ul>
  </div>
</template>

<script>
export default {
  props: ["state", "world", "input", "wasm"],
  methods: {
    buyTomb: function (tomb_name) {
      this.wasm.buy_tomb(tomb_name);
      this.$parent.update_dynamic_data();
    },
  },
};
</script>

<style scoped></style>
