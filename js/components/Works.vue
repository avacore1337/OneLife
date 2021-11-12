<template>
  <div style="border: solid; margin: 2px">
    Works
    <ul>
      <li
        v-for="(work, index) in world.works.filter(
          (work, index) => work.required_tier <= state.rebirth_stats.class_tier && state.works[index].is_visible
        )"
        :key="work.name"
      >
        <button v-on:click="!work.is_unlocked && set_work(work.name)" style="margin: 2px" :disabled="work.is_unlocked">
          <span v-if="work.name == input.work">{{ work.name }} &lt;-- </span>
          <span v-if="work.name != input.work">{{ work.name }}</span>
        </button>
        <p>
          Level: {{ state.works[index].current_level }}
          <b-progress
            class="notransition w-75"
            :value="state.works[index].next_level_percentage.toFixed(2)"
            animated
          ></b-progress>
        </p>
      </li>
    </ul>
  </div>
</template>

<script>
export default {
  props: ["state", "world", "input", "wasm"],
  methods: {
    set_work: function (work_name) {
      this.wasm.set_work(work_name);
    },
  },
};
</script>

<style scoped></style>
