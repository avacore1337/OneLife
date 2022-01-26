<template>
  <div v-if="true">
    <b-progress v-if="value !== undefined" class="notransition w-75" :value="value.toFixed(decimalPoints)"></b-progress>
  </div>
</template>

<script>
export default {
  props: ["value", "decimalPoints"],
};
</script>

<style scoped>
.notransition > div.progress-bar {
  -webkit-transition: none !important;
  -moz-transition: none !important;
  -o-transition: none !important;
  -ms-transition: none !important;
  transition: none !important;
}
</style>
