<!DOCTYPE html>
<template>
  <div class="progress" style="position: relative">
    <div class="my-bar" :style="{ width: value + '%' }" id="myBar"></div>
    <div class="progress-text">{{ name }}</div>
  </div>
</template>

<script>
export default {
  props: ['value', 'name'],
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
  top: 0px;
  bottom: 0px;
  padding: 0.5rem;
  color: black;
  box-sizing: inherit;
}
</style>
