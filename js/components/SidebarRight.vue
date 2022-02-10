<template>
  <div class="column-flex">
    <br />
    Saved Ticks: {{ metaData.saved_ticks.toFixed(0) }}
    <br />
    <button style="margin: 2px" @click="wasm.toggle_use_saved_ticks">
      {{ !metaData.use_saved_ticks ? 'Use Saved Ticks' : "Don't Use Saved Ticks" }}
    </button>
    <br />
    <MyToggle :value="metaData.options.auto_work" :click="wasm.toggle_auto_work">
      Auto Work
    </MyToggle>
    <MyToggle :value="metaData.options.auto_living" :click="wasm.toggle_auto_living">
      Auto Living
    </MyToggle>
    <MyToggle :value="metaData.options.auto_buy_item" :click="wasm.toggle_auto_buy_item">
      Auto Buy Items
    </MyToggle>
    <MyToggle :value="metaData.options.auto_buy_blessing" :click="wasm.toggle_auto_buy_blessing">
      Auto Buy Blessings
    </MyToggle>
    <MyToggle :value="metaData.options.auto_buy_tomb" :click="wasm.toggle_auto_buy_tomb">
      Auto Buy Tombs
    </MyToggle>
    <MyToggle :value="metaData.options.auto_rebirth" :click="wasm.toggle_auto_rebirth">
      Auto Rebirth
    </MyToggle>
    <input v-model="end_early_criteria" size="10" />
    <button @click="wasm.set_auto_end_early(end_early_criteria)">Set End Early Criteria</button>
    <button @click="wasm.set_disable_tutorial(false)">Enable Tutorial</button>
    <button @click="wasm.toggle_show_recorded">
      {{ !metaData.options.show_recorded ? 'Show Recorded' : "Don't Show Recorded" }}
    </button>
    <button @click="wasm.toggle_paused">
      {{ metaData.options.paused ? 'Resume the game' : 'Pause the game' }}
    </button>
    <button @click="tick">Tick</button>
  </div>
</template>

<script>
import { downloadFile } from '../utility.js'
import MyToggle from './MyToggle.vue'
export default {
  components: { MyToggle },
  props: ['metaData', 'state', 'world', 'input', 'wasm'],
  data() {
    return {
      end_early_criteria: 0.0,
    }
  },
  methods: {
    tick() {
      this.wasm.single_tick()
      this.$parent.update_dynamic_data()
    },
  },
}
</script>

<style scoped>
.column-flex {
  display: flex;
  flex-direction: column;
}
</style>
