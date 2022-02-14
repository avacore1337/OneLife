<template>
  <Section title="Items">
    <span>
      Show bought Items
      <input
        id="show_bought"
        type="checkbox"
        :checked="metaData.options.show_bought_items"
        @click="toggle_show_bought"
      />
    </span>
    <div
      v-if="
        metaData.options.show_bought_items && state.boost_items.some((item) => item.is_purchased)
      "
    >
      <h4>Bought Items</h4>
      <table>
        <tr v-for="[item, item_state] in bought_items" :key="item.name">
          <td>
            <span>{{ item.display_name }}</span>
            <br />
            <icon-with-text :icon="item.icon" :text="item.effect_description" />
          </td>
        </tr>
      </table>
    </div>
    <br />

    <h4>Buyable Items</h4>
    <table>
      <tr
        v-for="[item, item_state] in visible_unbought_items"
        :key="item.name"
        :class="{ disabled: !item_state.is_unlocked }"
      >
        <td @click.shift.exact="wasm.queue_item(item.name)" @click="buy_item(item.name, $event)">
          <span>{{ item.display_name }} </span>
          <span style="float: right">
            <icon-with-text :icon="world.icons['Money']">
              <FormatNumber :value="item.purchasing_cost" />
            </icon-with-text>
          </span>
          <br />
          <icon-with-text :icon="item.icon" :text="item.effect_description" />
        </td>
      </tr>
    </table>
  </Section>
</template>

<script>
import Section from './Section.vue'
import { compare } from '../utility.js'
import FormatNumber from './FormatNumber.vue'

export default {
  components: { Section, FormatNumber },
  props: ['state', 'world', 'input', 'wasm', 'metaData', 'item_queue'],
  computed: {
    visible_unbought_items() {
      let self = this
      return self.world.boost_items
        .map((w, i) => {
          return [w, self.state.boost_items[i]]
        })
        .filter(([w, s]) => {
          return s.is_visible && !s.is_purchased
        })
        .sort(compare)
    },
    bought_items() {
      let self = this
      return self.world.boost_items
        .map((w, i) => {
          return [w, self.state.boost_items[i]]
        })
        .filter(([w, s]) => {
          return s.is_purchased
        })
        .sort(compare)
    },
  },
  methods: {
    buy_item(val, e) {
      if (!e.shiftKey) {
        this.wasm.buy_item(val)
      } else {
        e.preventDefault()
      }
    },

    toggle_show_bought() {
      this.wasm.set_show_bought_items(!this.metaData.options.show_bought_items)
    },
  },
}
</script>

<style scoped></style>
