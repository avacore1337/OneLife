<template>
  <div>
    Info
    <div style="max-width: 1000px">
      <b-form-textarea
        readonly
        id="textarea"
        v-model="tutorial_text"
        rows="24"
        max-rows="24"
      ></b-form-textarea>
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

<style scoped></style>
