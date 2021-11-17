<template>
  <div>
    Base stats
    <div style="border: solid; margin: 2px; padding: 10px">
      <div v-for="stat in state.base_stats" :key="stat.name">
        <p>{{ stat.name }}: {{ stat.level }}</p>
        <b-progress class="notransition w-75" :value="stat.next_level_percentage.toFixed(2)" animated></b-progress>
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
