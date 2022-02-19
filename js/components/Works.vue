<template>
  <div class="section">
    <WorkCategory name="Labor Work" :thework="visible_labor_work" />
    <br />
    <WorkCategory
      v-if="state.rebirth_stats.tier >= 2"
      name="Soldiering"
      :thework="visible_soldier_work"
    />
    <br />
    <WorkCategory
      v-if="state.rebirth_stats.unlocks.has_faith"
      name="Priesthood"
      :thework="visible_priest_work"
    />
  </div>
</template>

<script>
import WorkCategory from './WorkCategory.vue'
import { mapState } from 'vuex'

export default {
  components: { WorkCategory },
  computed: {
    ...mapState(['state', 'meta', 'input']),
    visible_labor_work() {
      let self = this
      return self.$world.works
        .map((w, i) => {
          return [w, self.state.works[i]]
        })
        .filter(([w, s]) => {
          return s.is_visible && w.work_type === 'Labor'
        })
    },
    visible_soldier_work() {
      let self = this
      return self.$world.works
        .map((w, i) => {
          return [w, self.state.works[i]]
        })
        .filter(([w, s]) => {
          return s.is_visible && w.work_type === 'Soldier'
        })
    },
    visible_priest_work() {
      let self = this
      return self.$world.works
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
