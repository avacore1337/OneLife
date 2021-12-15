<template>
  <div>
    <Section title="Labour">
      <tr
        v-for="(work, index) in world.works.filter(
          (work, index) => state.works[index].is_visible && work.main_stat == 'Con'
        )"
        v-on:click="state.works[index].is_unlocked && set_work(work.name)"
        v-bind:class="{ disabled: !state.works[index].is_unlocked }"
        :key="work.name"
      >
        <td>
          <p v-bind:class="{ selected: input.work === work.name }">{{ work.display_name }}</p>
        </td>
        <td>
          <p>
            Level: {{ state.works[index].level }} Reached level: {{ state.rebirth_stats.max_job_levels[index] }} Income
            {{ state.works[index].effective_income.toFixed(1) }}/s
            <b-progress
              class="notransition w-75"
              :value="state.works[index].next_level_percentage.toFixed(2)"
              animated
            ></b-progress>
          </p>
        </td>
      </tr>
    </Section>
    <Section title="Soldiering" v-if="state.rebirth_stats.class_tier >= 2">
      <tr
        v-for="(work, index) in world.works.filter(
          (work, index) => state.works[index].is_visible && work.main_stat == 'Str'
        )"
        v-on:click="state.works[index].is_unlocked && set_work(work.name)"
        v-bind:class="{ disabled: !state.works[index].is_unlocked }"
        :key="work.name"
      >
        <td>
          <p v-bind:class="{ selected: input.work === work.name }">{{ work.display_name }}</p>
        </td>
        <td>
          <p>
            Level: {{ state.works[index].level }} Reached level: {{ state.rebirth_stats.max_job_levels[index] }} Income
            {{ state.works[index].effective_income.toFixed(1) }}/s
            <b-progress
              class="notransition w-75"
              :value="state.works[index].next_level_percentage.toFixed(2)"
              animated
            ></b-progress>
          </p>
        </td>
      </tr>
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
