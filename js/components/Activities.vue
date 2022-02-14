<template>
  <Section v-if="world.activities !== undefined" title="Activity">
    <table>
      <tr>
        <th>Name</th>
        <th class="is-centered">Base Amount</th>
        <th class="is-centered">Stat/Skill</th>
      </tr>
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

        <td style="width: 20%" class="is-centered">
          <span>
            {{ activity.base_gain_amount }}
          </span>
        </td>
        <td style="width: 30%" class="is-centered">
          <icon-with-text :icon="activity.icon" :text="activity.effect_description" />
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

.is-centered {
  text-align: center;
}

.center-items {
  display: flex;
  justify-content: center;
  align-items: center;
}

/* tr > td:not(:first-child) { */
/*   width: 30%; */
/* } */
</style>
