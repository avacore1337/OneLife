<template>
  <div>
    <h4 class="section-header">Automation</h4>
    <div class="section column-flex">
      <MyToggle
        v-if="state.rebirth_stats.unlocks.can_auto_work"
        :value="metaData.options.auto_work"
        :click="wasm.toggle_auto_work"
        v-b-tooltip.hover.left="auto_work_tooltip"
      >
        Auto Work
      </MyToggle>

      <MyToggle
        v-if="state.rebirth_stats.unlocks.can_auto_living"
        :value="metaData.options.auto_living"
        :click="wasm.toggle_auto_living"
        v-b-tooltip.hover.left="auto_living_tooltip"
      >
        Auto Living
      </MyToggle>

      <MyToggle
        v-if="state.rebirth_stats.unlocks.can_auto_buy_item"
        :value="metaData.options.auto_buy_item"
        :click="wasm.toggle_auto_buy_item"
        v-b-tooltip.hover.left="auto_buy_item_tooltip"
      >
        Auto Buy Items
      </MyToggle>

      <MyToggle
        v-if="state.rebirth_stats.unlocks.can_auto_buy_blessing"
        :value="metaData.options.auto_buy_blessing"
        :click="wasm.toggle_auto_buy_blessing"
        v-b-tooltip.hover.left="auto_buy_blessing_tooltip"
      >
        Auto Buy Blessings
      </MyToggle>

      <MyToggle
        v-if="state.rebirth_stats.unlocks.can_auto_buy_tomb"
        :value="metaData.options.auto_buy_tomb"
        :click="wasm.toggle_auto_buy_tomb"
        v-b-tooltip.hover.left="auto_buy_tomb_tooltip"
      >
        Auto Buy Tombs
      </MyToggle>

      <MyToggle
        v-if="state.rebirth_stats.unlocks.can_auto_rebirth"
        :value="metaData.options.auto_rebirth"
        :click="wasm.toggle_auto_rebirth"
        v-b-tooltip.hover.left="auto_rebirth_tooltip"
      >
        Auto Rebirth
      </MyToggle>
      <div
        class="column-flex"
        v-if="state.rebirth_stats.unlocks.can_auto_end_early"
        v-b-tooltip.hover.left="end_early_tooltip"
      >
        <input
          placeholder="End early criteria, 0 for disable"
          v-model="end_early_criteria"
          size="10"
        />
        <b-button @click="wasm.set_auto_end_early(end_early_criteria)"
          >Set End Early Criteria</b-button
        >
      </div>
    </div>
  </div>
</template>

<script>
import MyToggle from './MyToggle.vue'
export default {
  components: { MyToggle },
  props: ['metaData', 'state', 'wasm'],
  data() {
    return {
      end_early_criteria: 0.0,
      end_early_tooltip:
        'The amount of coins needed for the end early criteria to trigger, enter 0 to disable',
      auto_work_tooltip:
        'Will set the active work to the highest tier one within the category (Labor, Soldier, etc)',
      auto_living_tooltip:
        'Will set the active housing to the highest one that doesn`t exceed your income',
      auto_buy_item_tooltip: 'Will buy items as soon as they can be afforded',
      auto_buy_blessing_tooltip: 'Will upgrade any blessing as soon as it can be afforded',
      auto_buy_tomb_tooltip: 'Will upgrade the tomb as soon as can be afforded',
      auto_rebirth_tooltip: 'Automatically start a new run as soon as you die',
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
