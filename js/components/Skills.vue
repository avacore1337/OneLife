<template>
  <div>
    Skills
    <div style="border: solid; margin: 2px; padding: 10px">
      <div
        v-for="(skill, index) in world.skills
          .map((skill, index) => ({ skill, index }))
          .filter(({ skill, index }) => state.skills[index].is_visible)"
        :key="skill.name"
      >
        <p>
          {{ skill.skill.display_name }}: {{ state.skills[index].level }} xp rate: {{ state.skills[index].xp_rate }}
        </p>
        <ProgressBar :value="state.skills[index].next_level_percentage" :decimalPoints="2"> </ProgressBar>
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
};
</script>

<style scoped></style>
