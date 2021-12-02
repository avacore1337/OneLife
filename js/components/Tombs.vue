<template>
  <div style="border: solid; margin: 2px">
    Tombs
    <ul>
      <li v-for="(tomb, index) in world.tombs" :key="tomb.name">
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
    },
  },
};
</script>

<style scoped></style>
