<template>
  <Section title="Tombs">
    <span v-if="state.tombs.some((tomb) => tomb.is_purchased)">
      <h4>Bought Tombs</h4>
      <table>
        <tr v-for="[tomb, tomb_state] in bought_tombs" :key="tomb.name">
          <td>
            <span>{{ tomb.display_name }}</span>
            <br />
            <span>Coins: <FormatNumber :value="tomb_state.effective_income" /> </span>
          </td>
        </tr>
      </table>
    </span>

    <h4>Buyable Tombs</h4>
    <table>
      <tr
        v-for="[tomb, tomb_state] in visible_unbought_tombs"
        :key="tomb.name"
        :class="{ disabled: !tomb_state.is_unlocked }"
      >
        <td @click="buyTomb(tomb.name)">
          <span>{{ tomb.display_name }} </span>
          <span style="float: right"
            >Cost: <FormatNumber :value="tomb.purchasing_cost" /> money
          </span>
          <br />
          <span>Coins: <FormatNumber :value="tomb_state.effective_income" /> </span>
        </td>
      </tr>
    </table>
  </Section>
</template>

<script>
import Section from './Section.vue'
import FormatNumber from './FormatNumber.vue'

export default {
  components: { Section, FormatNumber },
  props: ['metaData', 'state', 'world', 'input', 'wasm'],
  computed: {
    visible_unbought_tombs() {
      let self = this
      return self.world.tombs
        .map((w, i) => {
          return [w, self.state.tombs[i]]
        })
        .filter(([w, s]) => {
          return s.is_visible && !s.is_purchased
        })
    },
    bought_tombs() {
      let self = this
      return self.world.tombs
        .map((w, i) => {
          return [w, self.state.tombs[i]]
        })
        .filter(([w, s]) => {
          return s.is_purchased
        })
    },
  },
  methods: {
    buyTomb(tomb_name) {
      this.wasm.buy_tomb(tomb_name)
      this.$parent.update_dynamic_data()
    },
  },
}
</script>

<style scoped></style>
