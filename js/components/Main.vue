<template>
  <b-tabs class="my-navbar">
    <b-tab key="life-tab" title="Life">
      <div class="my-container">
        <div class="main-item">
          <Works :meta="meta" :state="state" :input="input" />
          <Housing :meta="meta" :state="state" :input="input" />
        </div>
        <div class="main-item">
          <Activities :state="state" :input="input" />
          <Blessings
            v-if="state.rebirth_stats.unlocks.has_faith"
            :meta="meta"
            :state="state"
            :input="input"
          />
          <BoostItems :state="state" :input="input" :meta="meta" :item_queue="item_queue" />
          <div v-if="meta.options.show_recorded">
            <RecordedInputs
              title="Previous Recorded Inputs"
              :recorded_inputs="previous_recorded_inputs"
              :remove_recorded="$wasm.remove_previous_recorded"
              :clear_recorded="$wasm.clear_previous_recorded"
            />
            <RecordedInputs
              title="Current Inputs"
              :recorded_inputs="recorded_inputs"
              :clear_recorded="$wasm.clear_recorded"
              :remove_recorded="$wasm.remove_recorded"
            />
          </div>
        </div>
      </div>
    </b-tab>

    <b-tab
      v-if="
        state.life_stats.dead || state.life_stats.is_dying || state.rebirth_stats.rebirth_count > 0
      "
      key="death-tab"
      title="Death"
    >
      <div class="my-container">
        <div class="main-item">
          <Tombs :meta="meta" :state="state" :input="input" />
        </div>
        <div class="main-item">
          <Death :state="state" :input="input" />
          <RebirthUpgrades
            v-if="state.rebirth_stats.tier > 0"
            :state="state"
            :input="input"
            :meta="meta"
          />
        </div>
      </div>
    </b-tab>

    <b-tab key="settings-tab" title="Settings">
      <Settings :state="state" :input="input" :meta="meta" />
    </b-tab>

    <b-tab key="info-tab" title="Info">
      <Info :state="state" :input="input" :meta="meta" />
    </b-tab>
  </b-tabs>
</template>

<script>
import Works from './Works.vue'
import Housing from './Housing.vue'
import Blessings from './Blessings.vue'
import Activities from './Activities.vue'
import RecordedInputs from './RecordedInputs.vue'
import BoostItems from './BoostItems.vue'

import Tombs from './Tombs.vue'

import Death from './Death.vue'
import RebirthUpgrades from './RebirthUpgrades.vue'

import Settings from './Settings.vue'

import Info from './Info.vue'

export default {
  components: {
    Works,
    Housing,
    Activities,
    BoostItems,
    Blessings,
    Tombs,
    RecordedInputs,
    Death,
    RebirthUpgrades,
    Settings,
    Info,
  },
  props: ['item_queue', 'meta', 'state', 'input', 'recorded_inputs', 'previous_recorded_inputs'],
}
</script>

<style scoped>
b-tab {
  color: white;
}

.main-item {
  flex-grow: 1;
  flex-basis: 0;
}

.my-navbar {
}
</style>
