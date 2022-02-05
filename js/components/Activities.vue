<template>
  <Section v-if="world.activities !== undefined" title="Activity">
    <table>
      <tr
        v-for="[activity, activity_state] in visible_activities"
        :key="activity.name"
        :class="{ disabled: !activity_state.is_unlocked }"
        @click="wasm.set_activity(activity.name)"
      >
        <td>
          <span :class="{ selected: activity.name == input.activity }">
            {{ activity.display_name }}
          </span>
        </td>

        <td style="width: 50%">
          <span>
            <my-icon :icon="activity.icon" />
            {{ activity.effect_description }}
          </span>
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
  props: ['state', 'world', 'input', 'wasm'],
  computed: {
    visible_activities() {
      let self = this
      return self.world.activities
        .map((w, i) => {
          return [w, self.state.activities[i]]
        })
        .filter(([w, s]) => {
          return s.is_visible
        })
    },
  },
  methods: {},
}
</script>

<style scoped>
tr > td:first-child {
  width: 50%;
}

/* tr > td:not(:first-child) { */
/*   width: 30%; */
/* } */
</style>
