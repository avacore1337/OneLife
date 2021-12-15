<template>
  <b-progress class="notransition w-75" :value="value.toFixed(decimalPoints)" animated></b-progress>
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
