<template>
  <Section2 v-if="$world.activities !== undefined">
    <table>
      <tr class="header-row">
        <th>Activity</th>
        <th>XP</th>
        <th>Stat/Skill</th>
      </tr>
      <tr
        v-for="[activity, activity_state] in visible_activities"
        :key="activity.name"
        :class="{ mydisabled: !activity_state.is_unlocked }"
        @click="$wasm.set_activity(activity.name)"
      >
        <td>
          <span :class="{ selected: activity.name == input.activity }">
            {{ activity.display_name }}
          </span>
        </td>

        <td style="width: 20%">
          <span>
            {{ activity.base_gain_amount }}
          </span>
        </td>
        <td style="width: 30%">
          <icon-with-text :icon="activity.icon" :text="activity.effect_description" />
        </td>
      </tr>
    </table>
  </Section2>
</template>

<script>
import Section2 from './Section2.vue'
import FormatNumber from './FormatNumber.vue'

export default {
  components: { Section2, FormatNumber },
  props: ['state', 'input'],
  computed: {
    visible_activities() {
      let self = this
      return self.$world.activities
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

<style scoped></style>
