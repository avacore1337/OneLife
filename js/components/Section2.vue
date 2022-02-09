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
  height: calc(3rem + 2 px);
  display: flex;
  margin: 0 auto;
  gap: 10px;
  padding: 0.5rem;
  border-bottom: 2px solid lightgray;
  /* border-top: 2px solid lightgray; */
}

th {
  flex-grow: 1;
  flex-basis: 0;

  display: flex;
  justify-content: center;
  align-items: center;
}

td {
  flex-grow: 1;
  flex-basis: 0;
  display: flex;
  justify-content: center;
  align-items: center;
}

tr > td:first-child {
  flex-grow: 2;
}

tr > th:first-child {
  justify-content: start;
  flex-grow: 2;
}

/* tr:not(:last-child) { */
/*   border-top: 2px solid lightgray; */
/*   border-bottom: 1px solid lightgray; */
/* } */

tr:hover {
  cursor: pointer;
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
