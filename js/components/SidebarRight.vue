<template>
  <div>
    <h3>Settings</h3>
    <br />
    Saved Ticks: {{ metaData.saved_ticks.toFixed(0) }}
    <br />
    <button style="margin: 2px" @click="toggle_use_saved_ticks">
      {{ !metaData.use_saved_ticks ? 'Use Saved Ticks' : "Don't Use Saved Ticks" }}
    </button>
    <br />
    <!-- Default switch
<div class="custom-control custom-switch">
  <input type="checkbox" class="custom-control-input" id="customSwitches">
  <label class="custom-control-label" for="customSwitches">Toggle this switch element</label>
</div> -->
    <button style="margin: 2px" @click="toggle_auto_work">
      {{ !metaData.options.auto_work ? 'Auto Work' : "Don't Auto Work" }}
    </button>
    <br />
    <button style="margin: 2px" @click="toggle_auto_living">
      {{ !metaData.options.auto_living ? 'Auto Living' : "Don't Auto Living" }}
    </button>
    <br />
    <button style="margin: 2px" @click="toggle_auto_buy_item">
      {{ !metaData.options.auto_buy_item ? 'Auto Buy Item' : "Don't Auto Buy Item" }}
    </button>
    <br />
    <button style="margin: 2px" @click="toggle_auto_buy_blessing">
      {{ !metaData.options.auto_buy_blessing ? 'Auto Buy Blessing' : "Don't Auto Buy Blessing" }}
    </button>
    <br />
    <button style="margin: 2px" @click="toggle_auto_buy_tomb">
      {{ !metaData.options.auto_buy_tomb ? 'Auto Buy Tomb' : "Don't Auto Buy Tomb" }}
    </button>
    <br />
    <button style="margin: 2px" @click="toggle_auto_rebirth">
      {{ !metaData.options.auto_rebirth ? 'Auto Rebirth' : "Don't Auto Rebirth" }}
    </button>
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
export default {
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
