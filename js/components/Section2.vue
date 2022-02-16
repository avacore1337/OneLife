<template>
  <div class="section">
    <slot />
  </div>
</template>

<script>
export default {
  props: ['title'],
}
</script>

<style scoped>
table {
  width: 100%;
  height: 100%;
}

tr {
  height: calc(3rem + 2 px);
  display: flex;
  margin: 0 auto;
  gap: 10px;
  padding: 0.5rem;
  border-bottom: 2px solid lightgray;
  /* border-top: 2px solid lightgray; */

  -webkit-touch-callout: none; /* iOS Safari */
  -webkit-user-select: none; /* Safari */
  -khtml-user-select: none; /* Konqueror HTML */
  -moz-user-select: none; /* Firefox */
  -ms-user-select: none; /* Internet Explorer/Edge */
  user-select: none; /* Non-prefixed version, currently supported by Chrome and Opera */
}

td {
  white-space: nowrap;
  flex-grow: 1;
  flex-basis: 0;
  display: flex;
  justify-content: center;
  align-items: center;
}

th {
  white-space: nowrap;
  flex-grow: 1;
  flex-basis: 0;

  display: flex;
  justify-content: center;
  align-items: center;
}

tr > td:first-child {
  flex-grow: 2;
  justify-content: start;
}

tr > th:first-child {
  justify-content: start;
  flex-grow: 2;
}

tr:hover {
  cursor: pointer;
}
</style>
