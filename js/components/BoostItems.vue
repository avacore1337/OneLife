<template>
  <Section title="Items">
    <div>
      <h4>Item Queue</h4>
      <table>
        <tr v-for="item in item_queue" :key="item.name">
          <td v-on:click="wasm.dequeue_item(item.name)">
            <span>{{ item.display_name }} </span>
            <span style="float: right">Cost: {{ item.purchasing_cost }} money </span>
            <br />
            <span>{{ item.effect_description }} </span>
          </td>
        </tr>
      </table>
    </div>
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
        <td v-on:click.shift.exakt="wasm.queue_item(item.name)" v-on:click.exakt="buy_item(item.name, $event)">
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
import { compare } from "../utility.js";

export default {
  props: ["state", "world", "input", "wasm", "metaData", "item_queue"],
  components: { Section },
  computed: {
    visible_unbought_items: function () {
      let self = this;
      console.log("Item update");
      return self.world.boost_items
        .map((w, i) => {
          return [w, self.state.boost_items[i]];
        })
        .filter(([w, s]) => {
          return s.is_visible && !s.is_purchased;
        })
        .sort(compare);
    },
    bought_items: function () {
      let self = this;
      return self.world.boost_items
        .map((w, i) => {
          return [w, self.state.boost_items[i]];
        })
        .filter(([w, s]) => {
          return s.is_purchased;
        })
        .sort(compare);
    },
  },
  methods: {
    buy_item: function (val, e) {
      if (!e.shiftKey) {
        console.log("no shift");
        this.wasm.buy_item(val);
      }
    },

    toggle_show_bought: function () {
      this.wasm.set_show_bought_items(!this.metaData.options.show_bought_items);
    },
  },
};
</script>

<style scoped></style>
