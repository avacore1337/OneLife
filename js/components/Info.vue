<template>
  <div class="my-container">
    <div>
      <h3>Story</h3>
      <div class="log-area">
        <span v-for="text in world.tutorial_texts">
          {{ text }}
          <hr />
        </span>
      </div>
    </div>

    <div>
      <h3>Tutorial</h3>
      <div class="log-area">
        <span v-for="text in world.tutorial_texts">
          {{ text }}
          <hr />
        </span>
      </div>
    </div>
  </div>
</template>

<script>
import FormatNumber from './FormatNumber.vue'
export default {
  components: { FormatNumber },
  props: ['state', 'world', 'input', 'wasm', 'metaData'],
  computed: {
    tutorial_text() {
      let separator = '-'.repeat(60)
      return this.world.tutorial_texts
        .filter((element, index) => {
          return index < this.metaData.info.tutorial_step
        })
        .join('\r\n' + separator + '\r\n')
    },
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
  border: 1px solid lightgray;
  height: 70%;
  overflow-y: scroll;
}
</style>
