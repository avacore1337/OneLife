<template>
  <Section :title="title">
    <button @click="clear_recorded">Clear Recorded</button>
    <br />
    <table>
      <tr v-for="entry in entries" :key="entry" @click="remove_recorded(entry.id)">
        <td>
          <p>tick:{{ entry.tick }} | {{ entry.name }}</p>
        </td>
      </tr>
    </table>
  </Section>
</template>

<script>
import Section from './Section.vue'

export default {
  components: { Section },
  props: ['wasm', 'recorded_inputs', 'remove_recorded', 'clear_recorded', 'title'],
  computed: {
    entries() {
      return Object.values(this.recorded_inputs)
    },
  },
}
</script>

<style scoped></style>
