<!DOCTYPE html>
<template>
  <div class="centered-container" style="position: relative">
    <!-- Rounded switch -->
    <label class="switch">
      <input type="checkbox" :checked="value" @click="click" />
      <span class="slider round"></span>
    </label>
    <slot />
  </div>
</template>

<script>
export default {
  props: ['value', 'click'],
}
</script>

<style scoped>
.switch {
  position: relative;
  display: inline-block;
  width: 2.2rem;
  height: 1.2rem;
}

/* Hide default HTML checkbox */
.switch input {
  opacity: 0;
  width: 0;
  height: 0;
}

/* The slider */
.slider {
  position: absolute;
  cursor: pointer;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-color: #ccc;
  -webkit-transition: 0.4s;
  transition: 0.4s;
}

.slider:before {
  position: absolute;
  content: '';
  height: 1rem;
  width: 1rem;
  left: 2px;
  bottom: 2px;
  background-color: white;
  -webkit-transition: 0.4s;
  transition: 0.4s;
}

input:checked + .slider {
  background-color: #2196f3;
}

input:focus + .slider {
  box-shadow: 0 0 1px #2196f3;
}

input:checked + .slider:before {
  -webkit-transform: translateX(1rem);
  -ms-transform: translateX(1rem);
  transform: translateX(1rem);
}

/* Rounded sliders */
.slider.round {
  border-radius: 1.2rem;
}

.slider.round:before {
  border-radius: 50%;
}

.centered-container {
  display: flex;
  align-items: center;
  gap: 5px;
}
</style>
