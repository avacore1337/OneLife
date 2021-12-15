<template>
  <div>
    <Section title="Works">
      <h4>Labour</h4>
      <table>
        <tr
          v-for="{ work, index } in world.works
            .map((work, index) => ({ work, index }))
            .filter(({ work, index }) => state.works[index].is_visible && work.main_stat === 'Con')"
          v-on:click="state.works[index].is_unlocked && set_work(work.name)"
          v-bind:class="{ disabled: !state.works[index].is_unlocked }"
          :key="work.name"
        >
          <td>
            <p v-bind:class="{ selected: input.work === work.name }">{{ work.display_name }}</p>
          </td>
          <td>
            <p>
              Level: {{ state.works[index].level }} Reached level:
              {{ state.rebirth_stats.max_job_levels[index] }} Income
              {{ state.works[index].effective_income.toFixed(1) }}/s
              <ProgressBar :value="state.works[index].next_level_percentage" :decimalPoints="2" />
            </p>
          </td>
        </tr>
      </table>
      <table v-if="state.rebirth_stats.class_tier >= 2">
        <br />
        <h4>Soldiering</h4>
        <tr
          v-for="{ work, index } in world.works
            .map((work, index) => ({ work, index }))
            .filter(({ work, index }) => state.works[index].is_visible && work.main_stat === 'Str')"
          v-on:click="state.works[index].is_unlocked && set_work(work.name)"
          v-bind:class="{ disabled: !state.works[index].is_unlocked }"
          :key="work.name"
        >
          <td>
            <p v-bind:class="{ selected: input.work === work.name }">{{ work.display_name }}</p>
          </td>
          <td>
            <p>
              Level: {{ state.works[index].level }} Reached level:
              {{ state.rebirth_stats.max_job_levels[index] }} Income
              {{ state.works[index].effective_income.toFixed(1) }}/s
              <ProgressBar :value="state.works[index].next_level_percentage" :decimalPoints="2" />
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
  methods: {
    set_work: function (work_name) {
      this.wasm.set_work(work_name);
    },
  },
};
</script>

<style scoped></style>
