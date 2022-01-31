<template>
  <b-tabs>
    <b-tab title="Life">
      <div style="margin-left: 1%; float: left; width: 49%">
        <Works
          v-bind:metaData="metaData"
          v-bind:state="state"
          v-bind:input="input"
          v-bind:world="world"
          v-bind:wasm="wasm"
        />
        <Housing
          v-bind:metaData="metaData"
          v-bind:state="state"
          v-bind:input="input"
          v-bind:world="world"
          v-bind:wasm="wasm"
        />
      </div>
      <div style="margin-left: 1%; float: left; width: 49%">
        <Activities v-bind:state="state" v-bind:input="input" v-bind:world="world" v-bind:wasm="wasm" />
        <BoostItems
          v-bind:state="state"
          v-bind:input="input"
          v-bind:world="world"
          v-bind:wasm="wasm"
          v-bind:metaData="metaData"
          v-bind:item_queue="item_queue"
        />
        <Blessings
          v-if="state.rebirth_stats.unlocks.has_faith"
          v-bind:metaData="metaData"
          v-bind:state="state"
          v-bind:input="input"
          v-bind:world="world"
          v-bind:wasm="wasm"
        />
        <div v-if="$parent.show_recorded">
          <RecordedInputs
            v-bind:recorded_inputs="previous_recorded_inputs"
            v-bind:wasm="wasm"
            v-bind:remove_recorded="wasm.remove_previous_recorded"
            v-bind:clear_recorded="wasm.clear_previous_recorded"
          />
          <RecordedInputs
            v-bind:recorded_inputs="recorded_inputs"
            v-bind:wasm="wasm"
            v-bind:clear_recorded="wasm.clear_recorded"
            v-bind:remove_recorded="wasm.remove_recorded"
          />
        </div>
      </div>
    </b-tab>
    <b-tab title="Death">
      <div style="margin-left: 1%; float: left; width: 49%">
        <Tombs
          v-bind:metaData="metaData"
          v-bind:state="state"
          v-bind:input="input"
          v-bind:world="world"
          v-bind:wasm="wasm"
        />
      </div>
      <div style="margin-left: 1%; float: left; width: 49%">
        <Death v-bind:state="state" v-bind:input="input" v-bind:world="world" v-bind:wasm="wasm" />
        <RebirthUpgrades
          v-if="state.rebirth_stats.tier > 0"
          v-bind:state="state"
          v-bind:input="input"
          v-bind:world="world"
          v-bind:wasm="wasm"
          v-bind:metaData="metaData"
        />
      </div>
    </b-tab>
    <b-tab title="Settings">
      <Settings
        v-bind:state="state"
        v-bind:input="input"
        v-bind:world="world"
        v-bind:wasm="wasm"
        v-bind:metaData="metaData"
      />
    </b-tab>
  </b-tabs>
</template>

<script>
import Works from "./Works.vue";
import Housing from "./Housing.vue";
import Blessings from "./Blessings.vue";
import Activities from "./Activities.vue";
import RecordedInputs from "./RecordedInputs.vue";
import BoostItems from "./BoostItems.vue";

import Tombs from "./Tombs.vue";

import Death from "./Death.vue";
import RebirthUpgrades from "./RebirthUpgrades.vue";

import Settings from "./Settings.vue";

export default {
  props: ["item_queue", "metaData", "state", "world", "input", "wasm"],
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
  },
};
</script>

<style scoped>
b-tab {
  color: white;
}
</style>
