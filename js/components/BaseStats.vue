<template>
  <div>
    <h4 class="section-header">Base Stats</h4>
    <div class="section">
      <div
        v-for="[stat, stat_state] in visible_stats"
        :key="stat.name"
        v-b-tooltip.hover.right.html="tooltip(stat)"
        style="margin-bottom: 1rem"
      >
        <MyProgressBar :value="stat_state.next_level_percentage">
          <span style="display: flex; justify-content: space-between; width: 100%">
            <icon-with-text :icon="stat.icon" color="black">
              <span>{{ stat.display_name }}</span>
            </icon-with-text>
            <span> Level: {{ stat_state.level }} </span>
          </span>
        </MyProgressBar>
      </div>
    </div>
  </div>
</template>

<script>
import MyProgressBar from './MyProgressBar.vue'
import FormatNumber from './FormatNumber.vue'

export default {
  components: { MyProgressBar, FormatNumber },
  props: ['state', 'world'],
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
