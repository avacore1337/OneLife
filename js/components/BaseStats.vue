<template>
  <div>
    Base Stats
    <div style="border: solid; margin: 2px; padding: 10px">
      <div v-for="(stat, index) in world.stats" v-if="state.stats[index].is_visible" :key="stat.name">
        <p>{{ stat.display_name }}: {{ state.stats[index].level }}</p>
        <b-progress
          class="notransition w-75"
          :value="state.stats[index].next_level_percentage.toFixed(2)"
          animated
        ></b-progress>
        <br />
      </div>
    </div>
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
