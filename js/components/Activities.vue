<template>
  <Section title="Activity" v-if="world.activities !== undefined">
    <table>
      <tr
        v-for="{ activity, index } in world.activities
          .map((activity, index) => ({ activity, index }))
          .filter(({ activity, index }) => state.activities[index].is_visible)"
        v-on:click="set_activity(activity.name)"
        v-bind:class="{ disabled: !state.activities[index].is_unlocked }"
        :key="activity.name"
      >
        <td>
          <p v-bind:class="{ selected: activity.name == input.activity }">{{ activity.display_name }}</p>
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
    set_activity: function (activity_name) {
      this.wasm.set_activity(activity_name);
    },
  },
};
</script>

<style scoped></style>
