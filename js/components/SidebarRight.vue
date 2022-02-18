<template>
  <div class="column-flex" style="margin-top: 1rem">
    <ItemQueue
      v-if="state.rebirth_stats.unlocks.can_queue_item"
      :wasm="wasm"
      :item_queue="item_queue"
    />
    <div v-if="state.rebirth_stats.unlocks.can_auto_work">
      <h4 class="section-header">Automation</h4>
      <div class="section flex-column">
        <MyToggle
          v-if="state.rebirth_stats.unlocks.can_auto_work"
          :value="metaData.options.auto_work"
          :click="wasm.toggle_auto_work"
        >
          Auto Work
        </MyToggle>
        <MyToggle
          v-if="state.rebirth_stats.unlocks.can_auto_living"
          :value="metaData.options.auto_living"
          :click="wasm.toggle_auto_living"
        >
          Auto Living
        </MyToggle>
        <MyToggle
          v-if="state.rebirth_stats.unlocks.can_auto_buy_item"
          :value="metaData.options.auto_buy_item"
          :click="wasm.toggle_auto_buy_item"
        >
          Auto Buy Items
        </MyToggle>
        <MyToggle
          v-if="state.rebirth_stats.unlocks.can_auto_buy_blessing"
          :value="metaData.options.auto_buy_blessing"
          :click="wasm.toggle_auto_buy_blessing"
        >
          Auto Buy Blessings
        </MyToggle>
        <MyToggle
          v-if="state.rebirth_stats.unlocks.can_auto_buy_tomb"
          :value="metaData.options.auto_buy_tomb"
          :click="wasm.toggle_auto_buy_tomb"
        >
          Auto Buy Tombs
        </MyToggle>
        <MyToggle
          v-if="state.rebirth_stats.unlocks.can_auto_rebirth"
          :value="metaData.options.auto_rebirth"
          :click="wasm.toggle_auto_rebirth"
        >
          Auto Rebirth
        </MyToggle>
        <div v-if="state.rebirth_stats.unlocks.can_auto_end_early">
          <input v-model="end_early_criteria" size="10" />
          <b-button @click="wasm.set_auto_end_early(end_early_criteria)"
            >Set End Early Criteria</b-button
          >
        </div>
      </div>
    </div>
    <h4 class="section-header">Options</h4>
    <div class="section flex-column">
      Saved Ticks: {{ metaData.saved_ticks.toFixed(0) }}
      <b-button @click="wasm.toggle_use_saved_ticks">
        {{ !metaData.use_saved_ticks ? 'Use Saved Ticks' : "Don't Use Saved Ticks" }}
      </b-button>
      <MyToggle :value="metaData.options.show_bought_items" :click="wasm.toggle_show_bought_items">
        Show Bought Items
      </MyToggle>
      <MyToggle
        :value="metaData.options.show_bought_upgrades"
        :click="wasm.toggle_show_bought_upgrades"
      >
        Show Bought Upgrades
      </MyToggle>
      <MyToggle :value="metaData.options.show_recorded" :click="wasm.toggle_show_recorded">
        Show Recorded
      </MyToggle>
      <b-button @click="wasm.toggle_paused">
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
  gap: 10px;
}
</style>
