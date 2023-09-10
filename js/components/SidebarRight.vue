<template>
  <div class="column-flex" style="margin-top: 1rem">
    <ItemQueue v-if="state.rebirth_stats.unlocks.can_queue_item" />
    <Automation v-if="state.rebirth_stats.unlocks.can_auto_work" />
    <h4 class="section-header">Options</h4>
    <div class="section column-flex">
      Saved Ticks: {{ meta.saved_ticks.toFixed(0) }}
      <b-button @click="$wasm.toggle_use_saved_ticks">
        {{ !meta.use_saved_ticks ? 'Use Saved Ticks' : "Don't Use Saved Ticks" }}
      </b-button>
      <MyToggle :value="meta.options.show_bought_items" :click="$wasm.toggle_show_bought_items">
        Show Bought Items
      </MyToggle>
      <MyToggle
        :value="meta.options.show_bought_upgrades"
        :click="$wasm.toggle_show_bought_upgrades"
      >
        Show Bought Upgrades
      </MyToggle>
      <MyToggle :value="meta.options.show_recorded" :click="$wasm.toggle_show_recorded">
        Show Recorded
      </MyToggle>
      <b-button @click="$wasm.toggle_paused">
        <icon-with-text
          v-if="meta.options.paused"
          :icon="$world.icons['Play']"
          text="Resume the game"
        />
        <icon-with-text
          v-if="!meta.options.paused"
          :icon="$world.icons['Pause']"
          text="Pause the game"
        />
      </b-button>
    </div>

    <!-- TODO build in actual date & hash injection -->
    <h4 class="section-header">Version</h4>
    <div class="section column-flex">
      <span v-b-tooltip.hover.left="version_build_tooltip">
        Build: {{ meta.info.version_build_data }}
      </span>
      <span v-b-tooltip.hover.left="version_commit_tooltip">
        Commit: {{ meta.info.version_commit_data }}
      </span>
    </div>
  </div>
</template>

<script>
import { downloadFile } from '../utility.js'
import ItemQueue from './ItemQueue.vue'
import Automation from './Automation.vue'
import MyToggle from './MyToggle.vue'
import FormatNumber from './FormatNumber.vue'
import { mapState } from 'vuex'

export default {
  components: { ItemQueue, Automation, FormatNumber, MyToggle },
  data() {
    return {
      version_build_tooltip: 'The build version of the game you are playing.',
      version_commit_tooltip: 'The date & hash of the commit this version was built from.',
      end_early_criteria: 0.0,
    }
  },
  methods: {},
  computed: mapState(['state', 'meta']),
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