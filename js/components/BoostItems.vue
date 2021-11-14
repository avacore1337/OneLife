<template>
  <div style="border: solid; margin: 2px">
    Items
    <ul>
      <li v-for="item in world.boost_items" :key="item.name">
        <button v-on:click="buy_item(item.name)" style="margin: 2px" :disabled="!can_buy_item(item.name)">
          <span v-if="item.name == input.item">{{ item.name }} &lt;-- </span>
          <span v-if="item.name != input.item">{{ item.name }}</span>
        </button>
      </li>
    </ul>
  </div>
</template>

<script>
export default {
  props: ["state", "world", "input", "wasm"],
  methods: {
    buy_item: function (item_name) {
      this.wasm.buy_item(item_name);
    },
    can_buy_item: function (item) {
      return this.wasm.can_buy_item(item);
    },
  },
};
</script>

<style scoped></style>
