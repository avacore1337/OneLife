<template>
  <span>
    <span v-if="text === ''" class="vertical-align">
      <v-icon :name="icon.name" size="lg" :color="color" style="margin-right: 0.3rem" />
      <slot />
    </span>
    <span v-if="text !== ''" class="vertical-align">
      <v-icon :name="icon.name" size="lg" color="white" style="margin-right: 0.3rem" />
      <span style="white-space: nowrap"> {{ text }} </span>
    </span>
  </span>
</template>

<script>
export default {
  props: {
    icon: {
      type: Object,
      required: true,
    },
    text: {
      type: String,
      default: '',
    },
    color: {
      type: String,
      default: 'white',
    },
  },
}
</script>

<style scoped>
.fa-icon {
  fill: currentColor;
}
</style>
