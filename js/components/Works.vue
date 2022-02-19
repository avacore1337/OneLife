<template>
  <div class="section">
    <WorkCategory name="Labor Work" :thework="visible_labor_work" :input="input" />
    <br />
    <WorkCategory
      v-if="state.rebirth_stats.tier >= 2"
      name="Soldiering"
      :thework="visible_soldier_work"
      :input="input"
    />
    <br />
    <WorkCategory
      v-if="state.rebirth_stats.unlocks.has_faith"
      name="Priesthood"
      :thework="visible_priest_work"
      :input="input"
    />
  </div>
</template>

<script>
import WorkCategory from './WorkCategory.vue'

export default {
  components: { WorkCategory },
  props: ['metaData', 'state', 'world', 'input'],
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
