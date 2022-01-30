<template>
  <div>
    Base Stats
    <div style="border: solid; margin: 2px; padding: 10px">
      <div v-for="[stat, stat_state] in visible_stats" :key="stat.name">
        <my-icon :icon="stat.icon" />
        <span>{{ stat.display_name }}: {{ stat_state.level }} </span>
        <span> xp rate: {{ Math.round(stat_state.xp_rate) }} </span>
        <ProgressBar :value="stat_state.next_level_percentage" :decimalPoints="2"></ProgressBar>
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
  computed: {
    visible_stats: function () {
      let self = this;
      return self.world.stats
        .map((w, i) => {
          return [w, self.state.stats[i]];
        })
        .filter(([w, s]) => {
          return s.is_visible;
        });
    },
  },
};
</script>

<style scoped></style>
