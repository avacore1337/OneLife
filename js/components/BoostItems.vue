<template>
  <div style="border: solid; margin: 2px">
    Bought Items
    <ul>
      <li
        v-for="(item, index) in world.boost_items"
        v-if="state.items.boost_items[index].is_purchased"
        :key="item.name"
      >
        <p>{{ item.display_name }}</p>
      </li>
    </ul>
    Items
    <ul>
      <li
        v-for="(item, index) in world.boost_items"
        v-if="!state.items.boost_items[index].is_purchased"
        :key="item.name"
      >
        <button v-on:click="buyItem(item.name)" :disabled="!state.items.boost_items[index].is_unlocked">
          {{ item.display_name }}
        </button>
        {{ item.purchasing_cost }}
      </li>
    </ul>
  </div>
</template>

<script>
export default {
  props: ["state", "world", "input", "wasm"],
  /* computed: { */
  /*   ownedItems: function () { */
  /*     return this.state.items.boost_items.filter((item) => item.is_purchased); */
  /*   }, */
  /*   notOwnedItems: function () { */
  /*     return this.state.items.boost_items.filter((item) => !item.is_purchased); */
  /*   }, */
  /* }, */
  methods: {
    buyItem: function (item_name) {
      this.wasm.buy_item(item_name);
    },
  },
};
</script>

<style scoped></style>
