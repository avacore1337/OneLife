<template>
  <div>
    <Section title="Works">
      <b-button
        v-if="state.rebirth_stats.unlocks.can_auto_work"
        size="sm"
        @click="toggle_auto_work"
      >
        {{ !metaData.options.auto_work ? 'Auto Work' : "Don't Auto Work" }}
      </b-button>
      <h4>Labor</h4>
      <table>
        <tr
          v-for="[work, work_state] in visible_labor_work"
          :key="work.name"
          :class="{ disabled: !work_state.is_unlocked }"
          @click="work_state.is_unlocked && wasm.set_work(work.name)"
        >
          <td>
            <p :class="{ selected: input.work === work.name }">{{ work.display_name }}</p>
          </td>
          <td>
            <p>
              Level: {{ work_state.level }} Reached level: {{ work_state.max_job_levels }} Income
              {{ work_state.effective_income.toFixed(1) }}/s
              <ProgressBar :value="work_state.next_level_percentage" :decimal-points="2" />
              <!--- XP required for next level: {{ work_state.next_level_required }} --->
            </p>
          </td>
        </tr>
      </table>
      <table v-if="state.rebirth_stats.tier >= 2">
        <br />
        <h4>Soldiering</h4>
        <tr
          v-for="[work, work_state] in visible_soldier_work"
          :key="work.name"
          :class="{ disabled: !work_state.is_unlocked }"
          @click="work_state.is_unlocked && wasm.set_work(work.name)"
        >
          <td>
            <p :class="{ selected: input.work === work.name }">{{ work.display_name }}</p>
          </td>
          <td>
            <p>
              Level: {{ work_state.level }} Reached level: {{ work_state.max_job_levels }} Income
              {{ work_state.effective_income.toFixed(1) }}/s
              <ProgressBar :value="work_state.next_level_percentage" :decimal-points="2" />
            </p>
          </td>
        </tr>
      </table>
      <table v-if="state.rebirth_stats.unlocks.has_faith">
        <br />
        <h4>Priesthood</h4>
        <tr
          v-for="[work, work_state] in visible_priest_work"
          :key="work.name"
          :class="{ disabled: !work_state.is_unlocked }"
          @click="work_state.is_unlocked && wasm.set_work(work.name)"
        >
          <td>
            <p :class="{ selected: input.work === work.name }">{{ work.display_name }}</p>
          </td>
          <td>
            <p>
              Level: {{ work_state.level }} Reached level: {{ work_state.max_job_levels }} Income
              {{ work_state.effective_income.toFixed(1) }}/s
              <ProgressBar :value="work_state.next_level_percentage" :decimal-points="2" />
            </p>
          </td>
        </tr>
      </table>
    </Section>
  </div>
</template>

<script>
import ProgressBar from './ProgressBar.vue'
import Section from './Section.vue'

export default {
  components: { ProgressBar, Section },
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
  methods: {
    toggle_auto_work() {
      this.wasm.set_auto_work(!this.metaData.options.auto_work)
    },
  },
}
</script>

<style scoped></style>
