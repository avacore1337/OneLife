<template>
  <span>
    <span class="vertical-align" v-if="text === ''">
      <v-icon :name="icon.name" size="lg" color="white" style="margin-right: 0.3rem" />
      <slot />
    </span>
    <span class="vertical-align" v-if="text !== ''">
      <v-icon :name="icon.name" size="lg" color="white" style="margin-right: 0.3rem" />
      <span style="white-space: nowrap"> {{ text }} </span>
    </span>
  </span>
</template>

<script>
export default {
  props: {
    icon: Object,
    text: {
      type: String,
      default: '',
    },
  },
}
</script>

<style scoped>
.fa-icon {
  fill: currentColor;
}
</style>
