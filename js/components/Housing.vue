<template>
  <Section title="Housing">
    <tr
      v-for="housing in world.housing.filter((_, index) => state.housing[index].is_visible)"
      v-on:click="set_housing(housing.name)"
      :key="housing.name"
    >
      <td>
        <p>
          <span v-bind:class="{ selected: input.housing === housing.name }">{{ housing.display_name }} </span>
          <br />
          Cost: {{ housing.upkeep }}/s
        </p>
      </td>
    </tr>
  </Section>
</template>

<script>
import Section from "./Section.vue";

export default {
  props: ["state", "world", "input", "wasm"],
  components: { Section },
  methods: {
    set_housing: function (housing_name) {
      this.wasm.set_housing(housing_name);
    },
  },
};
</script>

<style scoped></style>
