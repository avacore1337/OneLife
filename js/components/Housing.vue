<template>
  <div style="border: solid; margin: 2px">
    Housing
    <ul>
      <li v-for="housing in world.housing" :key="housing.name">
        <button v-on:click="set_housing(housing.name)" style="margin: 2px">
          <span v-if="housing.name == input.housing">{{ housing.name }} &lt;-- </span>
          <span v-if="housing.name != input.housing">{{ housing.name }}</span>
        </button>
      </li>
    </ul>
  </div>
</template>

<script>
export default {
  props: ["state", "world", "input", "wasm"],
  methods: {
    set_housing: function (housing_name) {
      this.wasm.set_housing(housing_name);
    },
  },
};
</script>

<style scoped></style>
