<template>
  <Section title="Items">
    <span>
      Show bought Items
      <input
        type="checkbox"
        id="show_bought"
        v-on:click="toggle_show_bought"
        :checked="metaData.options.show_bought_items"
      />
    </span>
    <span v-if="metaData.options.show_bought_items && state.boost_items.some((item) => item.is_purchased)">
      <h4>Bought Items</h4>
      <table>
        <tr v-for="[item, item_state] in bought_items" :key="item.name" style="height: 2rem">
          <td>
            <span>{{ item.display_name }}</span>
            <br />
            <span>{{ item.effect_description }} </span>
          </td>
        </tr>
      </table>
    </span>
    <br />

    <h4>Buyable Items</h4>
    <table>
      <tr
        v-for="[item, item_state] in visible_unbought_items"
        v-bind:class="{ disabled: !item_state.is_unlocked }"
        :key="item.name"
      >
        <td v-on:click="wasm.buy_item(item.name)">
          <span>{{ item.display_name }} </span>
          <span style="float: right">Cost: {{ item.purchasing_cost }} money </span>
          <br />
          <span>{{ item.effect_description }} </span>
        </td>
      </tr>
    </table>
  </Section>
</template>

<script>
import Section from "./Section.vue";

export default {
  props: ["state", "world", "input", "wasm", "metaData"],
  components: { Section },
  computed: {
    visible_unbought_items: function () {
      let self = this;
      return self.world.boost_items
        .map((w, i) => {
          return [w, self.state.boost_items[i]];
        })
        .filter(([w, s]) => {
          return s.is_visible && !s.is_purchased;
        });
    },
    bought_items: function () {
      let self = this;
      return self.world.boost_items
        .map((w, i) => {
          return [w, self.state.boost_items[i]];
        })
        .filter(([w, s]) => {
          return s.is_purchased;
        });
    },
  },
  methods: {
    toggle_show_bought: function () {
      this.wasm.set_show_bought_items(!this.metaData.options.show_bought_items);
    },
  },
};
</script>

<style scoped></style>
