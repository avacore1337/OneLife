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
.section {
  width: 100%;
  margin-bottom: 1rem;
  padding: 1rem 1rem 1rem 2rem;
  background-color: #333c4a;
  border-radius: 3rem 3rem 0 0;
  border-top: 0.4rem solid #1a202b;
  border-left: 0.4rem solid #1a202b;
  border-right: 0.4rem solid #1a202b;
}

.title {
  padding-bottom: 1rem;
}

table {
  width: 100%;
  height: 100%;
}

tr {
  height: 4rem;
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

tr.disabled {
  cursor: inherit;
  background-color: #84878a;
  color: rgba(255, 255, 255, 0.5);
}

.selected {
  text-decoration: underline;
  font-weight: bold;
}
</style>
