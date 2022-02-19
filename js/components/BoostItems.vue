<template>
  <Section2>
    <div
      v-if="
        metaData.options.show_bought_items && state.boost_items.some((item) => item.is_purchased)
      "
      style="margin-bottom: 2rem"
    >
      <table>
        <tr class="header-row">
          <th>Bought Items</th>
          <th>Effect</th>
        </tr>
        <tr
          v-for="[item, item_state] in bought_items"
          :key="item.name"
          v-b-tooltip.hover.left="item.description"
          class="info-tr"
        >
          <td>
            <span>{{ item.display_name }}</span>
          </td>
          <td>
            <icon-with-text :icon="item.icon" :text="item.effect_description" />
          </td>
        </tr>
      </table>
    </div>

    <table>
      <tr class="header-row">
        <th style="flex-grow: 2">Buyable Items</th>
        <th style="flex-grow: 2">Effect</th>
        <th style="flex-grow: 1">Cost</th>
      </tr>
      <tr
        v-for="[item, item_state] in visible_unbought_items"
        :key="item.name"
        v-b-tooltip.hover.left="item.description"
        :class="{
          mydisabled:
            !item_state.is_unlocked && !(shift && state.rebirth_stats.unlocks.can_queue_item),
        }"
        @click.shift.exact="$wasm.queue_item(item.name)"
        @click="buy_item(item.name, $event)"
      >
        <td style="flex-grow: 2">
          {{ item.display_name }}
        </td>
        <td style="flex-grow: 2">
          <icon-with-text :icon="item.icon" :text="item.effect_description" />
        </td>
        <td style="flex-grow: 1">
          <icon-with-text :icon="$world.icons['Money']">
            <FormatNumber :value="item.purchasing_cost" />
          </icon-with-text>
        </td>
      </tr>
    </table>
  </Section2>
</template>

<script>
import Section2 from './Section2.vue'
import { compare } from '../utility.js'
import FormatNumber from './FormatNumber.vue'

export default {
  components: { Section2, FormatNumber },
  props: ['state', 'input', 'metaData', 'item_queue'],
  data() {
    return {
      shift: false,
    }
  },
  computed: {
    visible_unbought_items() {
      let self = this
      return self.$world.boost_items
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
      return self.$world.boost_items
        .map((w, i) => {
          return [w, self.state.boost_items[i]]
        })
        .filter(([w, s]) => {
          return s.is_purchased
        })
        .sort(compare)
    },
  },
  mounted() {
    window.addEventListener('keydown', (event) => {
      if (event.key == 'Shift') {
        this.shift = true
      }
    })
    window.addEventListener('keyup', (event) => {
      if (event.key == 'Shift') {
        this.shift = false
      }
    })
  },
  methods: {
    buy_item(val, e) {
      if (!e.shiftKey) {
        this.$wasm.buy_item(val)
      } else {
        e.preventDefault()
      }
    },
  },
}
</script>

<style scoped></style>
