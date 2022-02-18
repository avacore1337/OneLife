<template>
  <div>
    Skills
    <div style="border: solid; margin: 2px; padding: 10px">
      <div
        v-for="[skill, skill_state] in visible_skills"
        :key="skill.name"
        v-b-tooltip.hover.right="'tooltip todo'"
      >
        <icon-with-text :icon="skill.icon">
          <span>{{ skill.display_name }}: {{ skill_state.level }} </span>
          <span> xp rate: {{ Math.round(skill_state.xp_rate) }} </span>
        </icon-with-text>
        <ProgressBar :value="skill_state.next_level_percentage" :decimal-points="2"> </ProgressBar>
        <br />
      </div>
    </div>
  </div>
</template>

<script>
import ProgressBar from './ProgressBar.vue'

export default {
  components: { ProgressBar },
  props: ['state', 'world', 'input', 'wasm'],
  computed: {
    visible_skills() {
      let self = this
      return self.world.skills
        .map((w, i) => {
          return [w, self.state.skills[i]]
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
