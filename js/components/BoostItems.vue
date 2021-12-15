<template>
  <Section title="Bought Items">
    <span v-if="state.items.boost_items.some((item) => item.is_purchased)">
      <h4>Acquired benefits</h4>
      <table>
        <tr
          v-for="{ item } in world.boost_items
            .map((item, index) => ({ item, index }))
            .filter(
              ({ item, index }) =>
                state.items.boost_items[index].is_purchased && state.items.boost_items[index].is_visible
            )"
          :key="item.name"
          style="height: 2rem"
        >
          <td>
            <p>{{ item.display_name }}</p>
          </td>
        </tr>
      </table>
    </span>

    <h4>Items</h4>
    <table>
      <tr
        v-for="{ item, index } in world.boost_items
          .map((item, index) => ({ item, index }))
          .filter(
            ({ item, index }) =>
              !state.items.boost_items[index].is_purchased && state.items.boost_items[index].is_visible
          )"
        v-bind:class="{ disabled: !state.items.boost_items[index].is_unlocked }"
        :key="item.name"
      >
        <td v-on:click="buyItem(item.name)">
          <span>{{ item.display_name }} </span>
          <br />
          Cost: {{ item.purchasing_cost }} money
        </td>
      </tr>
    </table>
  </Section>
</template>

<script>
import Section from "./Section.vue";

export default {
  props: ["state", "world", "input", "wasm"],
  components: { Section },
  methods: {
    buyItem: function (item_name) {
      this.wasm.buy_item(item_name);
    },
  },
};
</script>

<style scoped></style>
