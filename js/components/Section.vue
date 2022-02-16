<template>
  <div class="section">
    <h3 class="title">{{ title }}</h3>
    <slot />
  </div>
</template>

<script>
export default {
  props: ['title'],
}
</script>

<style scoped>
.title {
  padding-bottom: 1rem;
}

table {
  width: 100%;
  height: 100%;
}

tr {
  height: 4rem;
  -webkit-touch-callout: none; /* iOS Safari */
  -webkit-user-select: none; /* Safari */
  -khtml-user-select: none; /* Konqueror HTML */
  -moz-user-select: none; /* Firefox */
  -ms-user-select: none; /* Internet Explorer/Edge */
  user-select: none; /* Non-prefixed version, currently supported by Chrome and Opera */
}

tr:not(:last-child) {
  border-top: 1px solid lightgray;
  border-bottom: 1px solid lightgray;
}

tr:hover {
  cursor: pointer;
}

tr > td:first-child {
  width: 30%;
}

tr > td:not(:first-child) {
  width: 70%;
}
</style>
