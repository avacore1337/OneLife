<template>
  <div>
    <button v-on:click="wasm.save">Save</button>
    <button v-on:click="wasm.load">Load</button>
    <button v-on:click="wasm.hard_reset">Hard Reset</button>
    <input type="checkbox" id="autosave" v-on:click="toggleAutoSave" :checked="metaData.autosave" />
    <label for="autosave">Autosave</label>
    <button v-on:click="setNumberFormat">{{ nextNumberFormat($parent.numberFormat) }}</button>

    <h1 style="text-align: center">One Life</h1>
  </div>
</template>

<script>
export default {
  props: ["state", "world", "input", "wasm", "metaData"],
  methods: {
    toggleAutoSave: function () {
      this.wasm.set_autosave(!this.metaData.autosave);
    },
    nextNumberFormat: function (numberFormat) {
      return {
        DEFAULT: "Scientific notation",
        SCIENTIFIC: "Natural numbers",
      }[numberFormat];
    },
    setNumberFormat: function () {
      this.$parent.numberFormat = {
        DEFAULT: "SCIENTIFIC",
        SCIENTIFIC: "DEFAULT",
      }[this.$parent.numberFormat];
    },
  },
};
</script>

<style scoped></style>
