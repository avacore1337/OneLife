<template>
  <p>{{ message }}</p>
</template>

<script>
export default {
  props: ["message"],
};
</script>

<style scoped>
p {
  color: red;
}
</style>
