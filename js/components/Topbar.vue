<template>
  <div>
    Top Bar
    <button v-on:click="save">Save</button>
    <button v-on:click="load">Load</button>
    <button v-on:click="hard_reset">Hard Reset</button>
    <button v-on:click="setNumberFormat">{{ nextNumberFormat($parent.numberFormat) }}</button>
    <input type="checkbox" id="autosave" v-on:click="toggleAutoSave" :checked="metaData.autosave" />
    <label for="autosave">Autosave</label>
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
    save: function () {
      this.wasm.save();
    },
    load: function () {
      this.wasm.load();
    },
    hard_reset: function () {
      this.wasm.hard_reset();
    },
  },
};
</script>

<style scoped></style>
