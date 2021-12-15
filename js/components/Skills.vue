<template>
  <div>
    Skills
    <div style="border: solid; margin: 2px; padding: 10px">
      <div v-for="(skill, index) in world.skills" v-if="state.skills[index].is_visible" :key="skill.name">
        <p>{{ skill.display_name }}: {{ state.skills[index].level }} xp rate: {{ state.skills[index].xp_rate }}</p>
        <b-progress
          class="notransition w-75"
          :value="state.skills[index].next_level_percentage.toFixed(2)"
          animated
        ></b-progress>
        <br />
      </div>
    </div>
  </div>
</template>

<script>
export default {
  props: ["state", "world", "input", "wasm"],
  methods: {},
};
</script>

<style scoped></style>
