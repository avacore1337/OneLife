<!DOCTYPE html>
<template>
  <div class="progress" style="position: relative">
    <div id="myBar" class="my-bar" :style="{ width: value + '%' }"></div>
    <div class="progress-text">
      <span v-if="name !== undefined" :class="{ selected: selected }">
        {{ name }}
      </span>
      <slot v-if="name === undefined" />
    </div>
  </div>
</template>

<script>
export default {
  props: ['value', 'name', 'selected'],
}
</script>

<style scoped>
.progress {
  width: 100%;
  background-color: #ddd;
  height: 2rem;
}

.my-bar {
  position: relative;
  height: 2rem;
  background-color: #0d6efd;
}

.progress-text {
  position: absolute;
  padding: 0.5rem;
  color: black;
  box-sizing: inherit;
  white-space: nowrap;
  width: 100%;
}
.selected {
  text-decoration: underline;
  font-weight: bold;
}
</style>
