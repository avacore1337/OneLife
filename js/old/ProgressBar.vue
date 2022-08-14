<template>
  <div v-if="true">
    <b-progress
      v-if="value !== undefined"
      class="notransition w-75"
      :value="value.toFixed(decimalPoints)"
    ></b-progress>
  </div>
</template>

<script>
export default {
  props: ['value', 'decimalPoints'],
}
</script>

<style scoped></style>
