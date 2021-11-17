<template>
  <div style="border: solid; margin: 2px">
    Bought Items
    <ul>
      <li v-for="item in ownedItems" :key="item.name">
        <button v-on:click="buyItem(item.name)" style="margin: 2px" :disabled="!canBuyItem(item.name)">
          <span v-if="item.name === input.item">{{ item.name }} &lt;-- </span>
          <span v-if="item.name !== input.item">{{ item.name }}</span>
        </button>
      </li>
    </ul>
    Items
    <ul>
      <li v-for="item in notOwnedItems" :key="item.name">
        <button v-on:click="buyItem(item.name)" style="margin: 2px" :disabled="!canBuyItem(item.name)">
          <span v-if="item.name === input.item">{{ item.name }} &lt;-- </span>
          <span v-if="item.name !== input.item">{{ item.name }}</span>
        </button>
      </li>
    </ul>
  </div>
</template>

<script>
export default {
  props: ["state", "world", "input", "wasm"],
  computed: {
    ownedItems: function () {
      return this.state.items.boost_items.filter((item) => item.is_purchased);
    },
    notOwnedItems: function () {
      return this.state.items.boost_items.filter((item) => !item.is_purchased);
    },
  },
  methods: {
    buyItem: function (item_name) {
      this.wasm.buy_item(item_name);
    },
    canBuyItem: function (item) {
      return this.wasm.can_buy_item(item);
    },
  },
};
</script>

<style scoped></style>
