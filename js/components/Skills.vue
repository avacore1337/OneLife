<template>
  <div>
    <h4 class="section-header">Skills</h4>
    <div class="section">
      <div
        v-for="[skill, skill_state] in visible_skills"
        :key="skill.name"
        v-b-tooltip.hover.right.html="tooltip(skill)"
        style="margin-bottom: 1rem"
      >
        <MyProgressBar :value="skill_state.next_level_percentage">
          <span style="display: flex; justify-content: space-between; width: 100%">
            <icon-with-text :icon="skill.icon" color="black">
              <span>{{ skill.display_name }}</span>
            </icon-with-text>
            <span> Level: {{ skill_state.level }} </span>
          </span>
        </MyProgressBar>
      </div>
    </div>
  </div>
</template>

<script>
import MyProgressBar from './MyProgressBar.vue'
import { mapState } from 'vuex'

export default {
  components: { MyProgressBar },
  computed: {
    ...mapState(['state']),
    visible_skills() {
      let self = this
      return self.$world.skills
        .map((w, i) => {
          return [w, self.state.skills[i]]
        })
        .filter(([w, s]) => {
          return s.is_visible
        })
    },
  },
  methods: {
    tooltip(stat) {
      return stat.description + '\n\n' + stat.effect_description
    },
  },
}
</script>

<style scoped></style>
