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
    <div v-if="state.rebirth_stats.unlocks.can_queue_item && false">
      <h4>Item Queue</h4>
      <table>
        <tr v-for="item in item_queue" :key="item.name">
          <td @click="wasm.dequeue_item(item.name)">
            <span>{{ item.display_name }} </span>
            <span style="float: right">
              <FormatNumber :value="item.purchasing_cost" />
              <my-icon :icon="world.icons['Money']" />
            </span>
            <br />
            <my-icon :icon="item.icon" />
            <span>{{ item.effect_description }} </span>
          </td>
        </tr>
      </table>
    </div>
    <span
      v-if="
        metaData.options.show_bought_items && state.boost_items.some((item) => item.is_purchased)
      "
    >
      <h4>Bought Items</h4>
      <table>
        <tr v-for="[item, item_state] in bought_items" :key="item.name" style="height: 2rem">
          <td>
            <span>{{ item.display_name }}</span>
            <br />
            <my-icon :icon="item.icon" />
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
        :key="item.name"
        :class="{ disabled: !item_state.is_unlocked }"
      >
        <td @click.shift.exact="wasm.queue_item(item.name)" @click="buy_item(item.name, $event)">
          <span>{{ item.display_name }} </span>
          <span style="float: right">
            <my-icon :icon="world.icons['Money']" />
            <FormatNumber :value="item.purchasing_cost" />
          </span>
          <br />
          <my-icon :icon="item.icon" />
          <span>{{ item.effect_description }} </span>
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
