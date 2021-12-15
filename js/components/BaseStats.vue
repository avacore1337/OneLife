<template>
  <div>
    Base Stats
    <div style="border: solid; margin: 2px; padding: 10px">
      <div v-for="(stat, index) in world.stats.filter((_, index) => state.stats[index].is_visible)" :key="stat.name">
        <p>{{ stat.display_name }}: {{ state.stats[index].level }} xp rate: {{ state.stats[index].xp_rate }}</p>
        <ProgressBar :value="stat.next_level_percentage" :decimalPoints="2"></ProgressBar>
        <br />
      </div>
    </div>
  </div>
</template>

<script>
import ProgressBar from "./ProgressBar.vue";

export default {
  props: ["state", "world", "input", "wasm"],
  components: { ProgressBar },
  methods: {},
};
</script>

<style scoped></style>
