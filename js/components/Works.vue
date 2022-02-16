<template>
  <div class="section">
    <WorkCategory name="Labor Work" :thework="visible_labor_work" :input="input" :wasm="wasm" />
    <br />
    <WorkCategory
      v-if="state.rebirth_stats.tier >= 2"
      name="Soldiering"
      :thework="visible_soldier_work"
      :wasm="wasm"
      :input="input"
    />
    <br />
    <WorkCategory
      v-if="state.rebirth_stats.unlocks.has_faith"
      name="Soldiering"
      :thework="visible_priest_work"
      :input="input"
      :wasm="wasm"
    />
  </div>
</template>

<script>
import ProgressBar from './ProgressBar.vue'
import WorkCategory from './WorkCategory.vue'

export default {
  components: { ProgressBar, WorkCategory },
  props: ['metaData', 'state', 'world', 'input', 'wasm'],
  computed: {
    visible_labor_work() {
      let self = this
      return self.world.works
        .map((w, i) => {
          return [w, self.state.works[i]]
        })
        .filter(([w, s]) => {
          return s.is_visible && w.work_type === 'Labor'
        })
    },
    visible_soldier_work() {
      let self = this
      return self.world.works
        .map((w, i) => {
          return [w, self.state.works[i]]
        })
        .filter(([w, s]) => {
          return s.is_visible && w.work_type === 'Soldier'
        })
    },
    visible_priest_work() {
      let self = this
      return self.world.works
        .map((w, i) => {
          return [w, self.state.works[i]]
        })
        .filter(([w, s]) => {
          return s.is_visible && w.work_type === 'Priest'
        })
    },
  },
  methods: {},
}
</script>

<style scoped></style>
