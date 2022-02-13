<template>
  <div class="column-flex">
    <ItemQueue
      v-if="state.rebirth_stats.unlocks.can_queue_item"
      :wasm="wasm"
      :item_queue="item_queue"
    />
    Saved Ticks: {{ metaData.saved_ticks.toFixed(0) }}
    <b-button @click="wasm.toggle_use_saved_ticks">
      {{ !metaData.use_saved_ticks ? 'Use Saved Ticks' : "Don't Use Saved Ticks" }}
    </b-button>
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
    <b-button @click="wasm.set_auto_end_early(end_early_criteria)">Set End Early Criteria</b-button>
    <b-button @click="wasm.toggle_show_recorded">
      {{ !metaData.options.show_recorded ? 'Show Recorded' : "Don't Show Recorded" }}
    </b-button>
    <b-button @click="wasm.toggle_paused">
      <span v-if="metaData.options.paused">
        <my-icon :icon="world.icons['Play']" style="margin-right: 0.3rem" />
        Resume the game
      </span>
      <span v-if="!metaData.options.paused">
        <my-icon :icon="world.icons['Pause']" style="margin-right: 0.3rem" />
        Pause the game
      </span>
    </b-button>
  </div>
</template>

<script>
import { downloadFile } from '../utility.js'
import ItemQueue from './ItemQueue.vue'
import MyToggle from './MyToggle.vue'
import FormatNumber from './FormatNumber.vue'
export default {
  components: { ItemQueue, FormatNumber, MyToggle },
  props: ['metaData', 'state', 'world', 'input', 'wasm', 'item_queue'],
  data() {
    return {
      end_early_criteria: 0.0,
    }
  },
  methods: {},
}
</script>

<style scoped>
.column-flex {
  display: flex;
  flex-direction: column;
  gap: 5px;
}
</style>
