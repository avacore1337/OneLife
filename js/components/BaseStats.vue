<template>
  <div>
    Base Stats
    <div style="border: solid; margin: 2px; padding: 10px">
      <div
        v-for="[stat, stat_state] in visible_stats"
        :key="stat.name"
        v-b-tooltip.hover.right.html="tooltip(stat)"
      >
        <icon-with-text :icon="stat.icon">
          <span>{{ stat.display_name }}: {{ stat_state.level }} </span>
          <span> xp rate: <FormatNumber :value="stat_state.xp_rate" /> </span>
        </icon-with-text>
        <ProgressBar :value="stat_state.next_level_percentage" :decimal-points="2"></ProgressBar>
        <br />
      </div>
    </div>
  </div>
</template>

<script>
import ProgressBar from './ProgressBar.vue'
import FormatNumber from './FormatNumber.vue'

export default {
  components: { ProgressBar, FormatNumber },
  props: ['state', 'world', 'input', 'wasm'],
  computed: {
    visible_stats() {
      let self = this
      return self.world.stats
        .map((w, i) => {
          return [w, self.state.stats[i]]
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
