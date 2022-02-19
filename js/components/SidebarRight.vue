<template>
  <div class="column-flex" style="margin-top: 1rem">
    <ItemQueue v-if="state.rebirth_stats.unlocks.can_queue_item" :item_queue="item_queue" />
    <Automation
      v-if="state.rebirth_stats.unlocks.can_auto_work"
      :metaData="metaData"
      :state="state"
    />
    <h4 class="section-header">Options</h4>
    <div class="section column-flex">
      Saved Ticks: {{ metaData.saved_ticks.toFixed(0) }}
      <b-button @click="$wasm.toggle_use_saved_ticks">
        {{ !metaData.use_saved_ticks ? 'Use Saved Ticks' : "Don't Use Saved Ticks" }}
      </b-button>
      <MyToggle :value="metaData.options.show_bought_items" :click="$wasm.toggle_show_bought_items">
        Show Bought Items
      </MyToggle>
      <MyToggle
        :value="metaData.options.show_bought_upgrades"
        :click="$wasm.toggle_show_bought_upgrades"
      >
        Show Bought Upgrades
      </MyToggle>
      <MyToggle :value="metaData.options.show_recorded" :click="$wasm.toggle_show_recorded">
        Show Recorded
      </MyToggle>
      <b-button @click="$wasm.toggle_paused">
        <icon-with-text
          v-if="metaData.options.paused"
          :icon="world.icons['Play']"
          text="Resume the game"
        />
        <icon-with-text
          v-if="!metaData.options.paused"
          :icon="world.icons['Pause']"
          text="Pause the game"
        />
      </b-button>
    </div>
  </div>
</template>

<script>
import { downloadFile } from '../utility.js'
import ItemQueue from './ItemQueue.vue'
import Automation from './Automation.vue'
import MyToggle from './MyToggle.vue'
import FormatNumber from './FormatNumber.vue'
export default {
  components: { ItemQueue, Automation, FormatNumber, MyToggle },
  props: ['metaData', 'state', 'world', 'input', 'item_queue'],
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
  width: 100%;
  flex-direction: column;
  gap: 5px;
}
</style>
