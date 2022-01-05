<template>
  <Section title="Recorded Inputs">
    <table>
      <tr v-for="entry in entries" v-on:click="remove_recorded(entry.id)">
        <td>
          <p>tick:{{ entry.tick }} | {{ entry.name }}</p>
        </td>
      </tr>
    </table>
  </Section>
</template>

<script>
import Section from "./Section.vue";

export default {
  props: ["wasm", "recorded_inputs", "remove_recorded"],
  components: { Section },
  computed: {
    entries: function () {
      return Object.values(this.recorded_inputs);
    },
  },
};
</script>

<style scoped></style>
