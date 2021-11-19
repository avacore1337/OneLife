<template>
  <div style="border: solid; margin: 2px">
    Activity
    <ul>
      <li v-for="activity in world.activities" :key="activity.name">
        <button v-on:click="set_activity(activity.name)" style="margin: 2px">
          <span v-if="activity.name == input.activity">{{ activity.name }} &lt;-- </span>
          <span v-if="activity.name != input.activity">{{ activity.name }}</span>
        </button>
      </li>
    </ul>
  </div>
</template>

<script>
export default {
  props: ["state", "world", "input", "wasm"],
  methods: {
    set_activity: function (activity_name) {
      this.wasm.set_activity(activity_name);
    },
  },
};
</script>

<style scoped></style>
