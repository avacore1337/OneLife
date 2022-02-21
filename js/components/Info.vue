<template>
  <div class="my-container">
    <div style="width: 100%">
      <h3>Story</h3>
      <div class="log-area">
        <span v-for="(text, index) in tutorial_texts" :key="index">
          <span v-html="text"></span>
          <hr />
        </span>
      </div>
    </div>

    <div style="width: 100%">
      <h3>Tips</h3>
      <div class="log-area">
        <span v-for="(text, index) in tutorial_tips" :key="index">
          {{ text }}
          <hr />
        </span>
      </div>
    </div>
  </div>
</template>

<script>
import FormatNumber from './FormatNumber.vue'
import { mapState } from 'vuex'

export default {
  components: { FormatNumber },
  computed: {
    ...mapState(['state', 'meta', 'input', 'tutorial_data']),
    tutorial_texts() {
      return this.completed_steps.map((data) => data.basetext).filter((data) => data != '')
    },
    tutorial_tips() {
      return this.completed_steps.map((data) => data.tip).filter((data) => data != '')
    },
    completed_steps() {
      let watch = this.meta.info.tutorial_step // needed for watch updates
      return this.$wasm.get_completed_steps().map((entry) => this.tutorial_data[entry])
    },
  },
}
</script>

<style scoped>
.log-area {
  padding: 0.2rem;
  max-width: 1000px;
  width: 100%;
  border: 1px solid lightgray;
  height: 70%;
  overflow-y: scroll;
  white-space: pre-line;
}
</style>
