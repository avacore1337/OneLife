<template>
  <Section title="Items">
    <span v-if="state.tombs.some((tomb) => tomb.is_purchased)">
      <h4>Bought Tombs</h4>
      <table>
        <tr v-for="[tomb, tomb_state] in bought_tombs" :key="tomb.name">
          <td>
            <span>{{ tomb.display_name }}</span>
            <br />
            <span>Coins: {{ tomb_state.effective_income }} </span>
          </td>
        </tr>
      </table>
    </span>

    <h4>Tombs</h4>
    <table>
      <tr
        v-for="[tomb, tomb_state] in visible_unbought_tombs"
        v-bind:class="{ disabled: !tomb_state.is_unlocked }"
        :key="tomb.name"
      >
        <td v-on:click="buyTomb(tomb.name)">
          <span>{{ tomb.display_name }} </span>
          <span style="float: right">Cost: {{ tomb.purchasing_cost }} money </span>
          <br />
          <span>Coins: {{ tomb_state.effective_income }} </span>
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
  computed: {
    visible_unbought_tombs: function () {
      let self = this;
      return self.world.tombs
        .map((w, i) => {
          return [w, self.state.tombs[i]];
        })
        .filter(([w, s]) => {
          return s.is_visible && !s.is_purchased;
        });
    },
    bought_tombs: function () {
      let self = this;
      return self.world.tombs
        .map((w, i) => {
          return [w, self.state.tombs[i]];
        })
        .filter(([w, s]) => {
          return s.is_purchased;
        });
    },
  },
  methods: {
    buyTomb: function (tomb_name) {
      this.wasm.buy_tomb(tomb_name);
      this.$parent.update_dynamic_data();
    },
  },
};
</script>

<style scoped></style>
