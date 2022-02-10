<template>
  <div>
    <br />
    Saved Ticks: {{ metaData.saved_ticks.toFixed(0) }}
    <br />
    <button style="margin: 2px" @click="toggle_use_saved_ticks">
      {{ !metaData.use_saved_ticks ? 'Use Saved Ticks' : "Don't Use Saved Ticks" }}
    </button>
    <br />
    <MyToggle :value="metaData.options.auto_work" :click="toggle_auto_work"> Auto Work </MyToggle>
    <br />
    <MyToggle :value="metaData.options.auto_living" :click="toggle_auto_living">
      Auto Living
    </MyToggle>
    <br />
    <MyToggle :value="metaData.options.auto_buy_item" :click="toggle_auto_buy_item">
      Auto Buy Items
    </MyToggle>
    <br />
    <MyToggle :value="metaData.options.auto_buy_blessing" :click="toggle_auto_buy_blessing">
      Auto Buy Blessings
    </MyToggle>
    <br />
    <MyToggle :value="metaData.options.auto_buy_tomb" :click="toggle_auto_buy_tomb">
      Auto Buy Tombs
    </MyToggle>
    <br />
    <MyToggle :value="metaData.options.auto_rebirth" :click="toggle_auto_rebirth">
      Auto Rebirth
    </MyToggle>
    <br />
    <input v-model="end_early_criteria" size="10" />
    <br />
    <button @click="wasm.set_auto_end_early(end_early_criteria)">Set End Early Criteria</button>
    <br />
    <button @click="wasm.set_disable_tutorial(false)">Enable Tutorial</button>
    <br />
    <button style="margin: 2px" @click="toggle_show_recorded">
      {{ !metaData.options.show_recorded ? 'Show Recorded' : "Don't Show Recorded" }}
    </button>
    <br />
    <button style="margin: 2px" @click="toggle_pause">
      {{ metaData.options.paused ? 'Resume the game' : 'Pause the game' }}
    </button>
    <br />
    <button @click="tick">Tick</button>
    <br />
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
    toggle_use_saved_ticks() {
      this.wasm.use_saved_ticks(!this.metaData.use_saved_ticks)
    },
    toggle_auto_work() {
      this.wasm.set_auto_work(!this.metaData.options.auto_work)
    },
    toggle_auto_living() {
      this.wasm.set_auto_living(!this.metaData.options.auto_living)
    },
    toggle_auto_buy_item() {
      this.wasm.set_auto_buy_item(!this.metaData.options.auto_buy_item)
    },
    toggle_auto_buy_blessing() {
      this.wasm.set_auto_buy_blessing(!this.metaData.options.auto_buy_blessing)
    },
    toggle_auto_buy_tomb() {
      this.wasm.set_auto_buy_tomb(!this.metaData.options.auto_buy_tomb)
    },
    toggle_auto_rebirth() {
      this.wasm.set_auto_rebirth(!this.metaData.options.auto_rebirth)
    },
    tick(work_name) {
      this.wasm.single_tick()
      this.$parent.update_dynamic_data()
    },
    toggle_show_recorded() {
      this.wasm.set_show_recorded(!this.metaData.options.show_recorded)
    },
    toggle_pause() {
      this.wasm.set_paused(!this.metaData.options.paused)
    },
  },
}
</script>

<style scoped></style>
