<template>
  <div>
    <Section title="Works">
      <h4>Labor</h4>
      <table>
        <tr
          v-for="[work, work_state] in visible_labor_work"
          v-on:click="work_state.is_unlocked && wasm.set_work(work.name)"
          v-bind:class="{ disabled: !work_state.is_unlocked }"
          :key="work.name"
        >
          <td>
            <p v-bind:class="{ selected: input.work === work.name }">{{ work.display_name }}</p>
          </td>
          <td>
            <p>
              Level: {{ work_state.level }} Reached level: {{ work_state.max_job_levels }} Income
              {{ work_state.effective_income.toFixed(1) }}/s
              <ProgressBar :value="work_state.next_level_percentage" :decimalPoints="2" />
            </p>
          </td>
        </tr>
      </table>
      <table v-if="state.rebirth_stats.tier >= 2">
        <br />
        <h4>Soldiering</h4>
        <tr
          v-for="[work, work_state] in visible_solder_work"
          v-on:click="work_state.is_unlocked && set_work(work.name)"
          v-bind:class="{ disabled: !work_state.is_unlocked }"
          :key="work.name"
        >
          <td>
            <p v-bind:class="{ selected: input.work === work.name }">{{ work.display_name }}</p>
          </td>
          <td>
            <p>
              Level: {{ work_state.level }} Reached level: {{ work_state.max_job_levels }} Income
              {{ work_state.effective_income.toFixed(1) }}/s
              <ProgressBar :value="work_state.next_level_percentage" :decimalPoints="2" />
            </p>
          </td>
        </tr>
      </table>
    </Section>
  </div>
</template>

<script>
import ProgressBar from "./ProgressBar.vue";
import Section from "./Section.vue";

export default {
  props: ["state", "world", "input", "wasm"],
  components: { ProgressBar, Section },
  computed: {
    visible_labor_work: function () {
      let self = this;
      return self.world.works
        .map((w, i) => {
          return [w, self.state.works[i]];
        })
        .filter(([w, s]) => {
          return s.is_visible && w.work_type === "Labor";
        });
    },
    visible_solder_work: function () {
      let self = this;
      return self.world.works
        .map((w, i) => {
          return [w, self.state.works[i]];
        })
        .filter(([w, s]) => {
          return s.is_visible && w.work_type === "Soldier";
        });
    },
  },
  methods: {},
};
</script>

<style scoped></style>
