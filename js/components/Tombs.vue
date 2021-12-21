<template>
  <Section title="Items">
    <span v-if="state.tombs.some((tomb) => tomb.is_purchased)">
      <h4>Bought Tombs</h4>
      <table>
        <tr
          v-for="(tomb, index) in world.tombs"
          v-if="state.tombs[index].is_visible && state.tombs[index].is_purchased"
          :key="tomb.name"
        >
          <td>
            <span>{{ tomb.display_name }}</span>
            <br />
            <span>Coins: {{ tomb.coin_gain }} </span>
          </td>
        </tr>
      </table>
    </span>

    <h4>Tombs</h4>
    <table>
      <tr
        v-for="(tomb, index) in world.tombs"
        v-if="state.tombs[index].is_visible && !state.tombs[index].is_purchased"
        v-bind:class="{ disabled: !state.tombs[index].is_unlocked }"
        :key="tomb.name"
      >
        <td v-on:click="buyTomb(tomb.name)">
          <span>{{ tomb.display_name }} </span>
          <span style="float: right">Cost: {{ tomb.purchasing_cost }} money </span>
          <br />
          <span>Coins: {{ state.tombs[index].effective_income }} </span>
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
    buyTomb: function (tomb_name) {
      this.wasm.buy_tomb(tomb_name);
      this.$parent.update_dynamic_data();
    },
  },
};
</script>

<style scoped></style>
