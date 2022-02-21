<template>
  <div class="my-container">
    <div style="width: 100%">
      <h3>Story</h3>
      <div class="log-area">
        <span v-for="(text, index) in tutorial_texts" :key="index">
          {{ text }}
          <hr />
        </span>
      </div>
    </div>

    <div style="width: 100%">
      <h3>Tutorial</h3>
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
      return this.$wasm.get_completed_steps()
    },
    tutorial_tips() {
      return this.tutorial_data
      /* return this.tutorial_data.map((data) => data.tip) */
    },
    /* tutorial_text() { */
    /*   let separator = '-'.repeat(60) */
    /*   return this.$world.tutorial_texts */
    /*     .filter((element, index) => { */
    /*       return index < this.meta.info.tutorial_step */
    /*     }) */
    /*     .join('\r\n' + separator + '\r\n') */
    /* }, */
  },
  methods: {
    test() {
      console.log('test')
    },
  },
}
</script>

<style scoped>
.log-area {
  padding: 0.5rem;
  max-width: 1000px;
  width: 100%;
  border: 1px solid lightgray;
  height: 70%;
  overflow-y: scroll;
}
</style>
