<template>
  <div>
    Skills
    <div style="border: solid; margin: 2px; padding: 10px">
      <div v-for="[skill, skill_state] in visible_skills" :key="skill.name">
        <my-icon :icon="skill.icon" />
        <span>{{ skill.display_name }}: {{ skill_state.level }} </span>
        <span> xp rate: {{ Math.round(skill_state.xp_rate) }} </span>
        <ProgressBar :value="skill_state.next_level_percentage" :decimalPoints="2"> </ProgressBar>
        <br />
      </div>
    </div>
  </div>
</template>

<script>
import Section from "./Section.vue";
import ProgressBar from "./ProgressBar.vue";

export default {
  props: ["state", "world", "input", "wasm"],
  components: { Section, ProgressBar },
  methods: {},
  computed: {
    visible_skills: function () {
      let self = this;
      return self.world.skills
        .map((w, i) => {
          return [w, self.state.skills[i]];
        })
        .filter(([w, s]) => {
          return s.is_visible;
        });
    },
  },
};
</script>

<style scoped></style>
