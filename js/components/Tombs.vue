<template>
  <Section2>
    <div v-if="state.tombs.some((tomb) => tomb.is_purchased)" style="margin-bottom: 2rem">
      <table>
        <tr class="header-row">
          <th style="flex-grow: 3">Current tomb</th>
          <th style="flex-grow: 1">Coin Gain</th>
        </tr>
        <tr class="info-tr">
          <td style="flex-grow: 3">{{ max_bought_tomb[0].display_name }}</td>
          <td style="flex-grow: 1">
            <icon-with-text :icon="$world.icons['Coin']">
              <FormatNumber :value="max_bought_tomb[1].effective_income" />
            </icon-with-text>
          </td>
        </tr>
      </table>
    </div>

    <table>
      <tr class="header-row">
        <th style="flex-grow: 2">Buyable tombs</th>
        <th style="flex-grow: 1">Coin Gain</th>
        <th style="flex-grow: 1">Cost</th>
      </tr>
      <tr
        v-for="[tomb, tomb_state] in visible_unbought_tombs"
        :key="tomb.name"
        :class="{ mydisabled: !tomb_state.is_unlocked }"
        @click="buyTomb(tomb.name)"
      >
        <td>
          {{ tomb.display_name }}
        </td>
        <td>
          <icon-with-text :icon="$world.icons['Coin']">
            <FormatNumber :value="tomb_state.effective_income" />
          </icon-with-text>
        </td>
        <td>
          <icon-with-text :icon="$world.icons['Money']">
            <FormatNumber :value="tomb.purchasing_cost" />
          </icon-with-text>
        </td>
      </tr>
    </table>
  </Section2>
</template>

<script>
import Section2 from './Section2.vue'
import FormatNumber from './FormatNumber.vue'

export default {
  components: { Section2, FormatNumber },
  props: ['metaData', 'state', 'input'],
  computed: {
    visible_unbought_tombs() {
      let self = this
      return self.$world.tombs
        .map((w, i) => {
          return [w, self.state.tombs[i]]
        })
        .filter(([w, s]) => {
          return s.is_visible && !s.is_purchased
        })
    },
    max_bought_tomb() {
      let bought_tombs = this.$world.tombs
        .map((w, i) => {
          return [w, this.state.tombs[i]]
        })
        .filter(([w, s]) => {
          return s.is_purchased
        })
      return bought_tombs[bought_tombs.length - 1]
    },
  },
  methods: {
    buyTomb(tomb_name) {
      this.$wasm.buy_tomb(tomb_name)
      this.$parent.update_dynamic_data()
    },
  },
}
</script>

<style scoped></style>
