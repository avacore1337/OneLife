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
        <div v-if="show_recorded">
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
      <div style="margin-left: 1%; float: left; width: 49%">text</div>
    </b-tab>
    <b-tab title="Settings"> hello </b-tab>
  </b-tabs>
</template>

<script>
import Works from "./Works.vue";
import Housing from "./Housing.vue";
import Tombs from "./Tombs.vue";
import Blessings from "./Blessings.vue";
import Activities from "./Activities.vue";
import RecordedInputs from "./RecordedInputs.vue";
import BoostItems from "./BoostItems.vue";

export default {
  props: ["metaData", "state", "world", "input", "wasm"],
  components: { Works, Housing, Activities, BoostItems, Blessings, Tombs, RecordedInputs },
};
</script>

<style scoped>
b-tab {
  color: white;
}
</style>
