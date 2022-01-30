<template>
  <Section title="Activity" v-if="world.activities !== undefined">
    <table>
      <tr
        v-for="[activity, activity_state] in visible_activities"
        v-on:click="wasm.set_activity(activity.name)"
        v-bind:class="{ disabled: !activity_state.is_unlocked }"
        :key="activity.name"
      >
        <td>
          <span v-bind:class="{ selected: activity.name == input.activity }">
            {{ activity.display_name }}
          </span>
          <span style="float: right">
            <my-icon :icon="activity.icon" />
            {{ activity.effect_description }}
          </span>
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
