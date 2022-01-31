<template>
  <div>
    <Works
      v-bind:metaData="metaData"
      v-bind:state="state"
      v-bind:input="input"
      v-bind:world="world"
      v-bind:wasm="wasm"
    />
    <Housing
      v-bind:metaData="metaData"
      v-bind:state="state"
      v-bind:input="input"
      v-bind:world="world"
      v-bind:wasm="wasm"
    />
  </div>
</template>

<script>
import Works from "./Works.vue";
import Housing from "./Housing.vue";

export default {
  props: ["metaData", "state", "world", "input", "wasm"],
  components: { Works, Housing },
};
</script>

<style scoped></style>
