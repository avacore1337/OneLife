<template>
  <div>
    Base stats
    <div style="border: solid; margin: 2px; padding: 10px">
      <p>Strength: {{ state.base_stats.str }}</p>
      <p>Intelligence: {{ state.base_stats.int }}</p>
      <p>Charisma: {{ state.base_stats.cha }}</p>
      <p>Constitution: {{ state.base_stats.con }}</p>
      <p>Dexterity: {{ state.base_stats.dex }}</p>
      <p>Faith: {{ state.base_stats.faith }}</p>
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
