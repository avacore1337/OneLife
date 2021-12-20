<template>
  <Section title="Activity" v-if="world.activities !== undefined">
    <table>
      <tr
        v-for="[activity, activity_state] in visible_activities"
        v-on:click="set_activity(activity.name)"
        v-bind:class="{ disabled: !activity_state.is_unlocked }"
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
  computed: {
    visible_activities: function () {
      let self = this;
      return self.world.activities
        .map((w, i) => {
          return [w, self.state.activities[i]];
        })
        .filter(([w, s]) => {
          return s.is_visible;
        });
    },
  },
};
</script>

<style scoped></style>
