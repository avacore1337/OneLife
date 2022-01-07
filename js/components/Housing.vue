<template>
  <Section title="Housing">
    <table>
      <tr
        v-for="[housing, housing_state] in visible_housing"
        v-on:click="wasm.set_housing(housing.name)"
        v-bind:class="{ disabled: !housing_state.is_unlocked }"
        :key="housing.name"
      >
        <td>
          <p>
            <span v-bind:class="{ selected: input.housing === housing.name }">{{ housing.display_name }} </span>
            <span v-if="!housing_state.is_unlocked" style="float: right"
              >Required Money: {{ housing.required_money }}
            </span>
            <br />
            Cost: {{ housing.upkeep }}/s
          </p>
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
  methods: {},
  computed: {
    visible_housing: function () {
      let self = this;
      return self.world.housing
        .map((w, i) => {
          return [w, self.state.housing[i]];
        })
        .filter(([w, s]) => {
          return s.is_visible;
        });
    },
  },
};
</script>

<style scoped></style>
