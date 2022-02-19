<template>
  <Section2>
    <div
      v-if="
        meta.options.show_bought_upgrades &&
        state.rebirth_stats.rebirth_upgrades.some((upgrade) => upgrade.is_purchased)
      "
      style="margin-bottom: 1rem"
    >
      <table>
        <tr class="header-row">
          <th style="flex-grow: 1">Bought Upgrade</th>
          <th style="flex-grow: 1">Effect</th>
        </tr>
        <tr v-for="[upgrade, upgrade_state] in bought_upgrades" :key="upgrade.name" class="info-tr">
          <td style="flex-grow: 1">{{ upgrade.display_name }}</td>
          <td style="flex-grow: 1">
            <icon-with-text :icon="upgrade.icon" :text="upgrade.effect_description" />
          </td>
        </tr>
      </table>
    </div>
    <table>
      <tr class="header-row">
        <th style="flex-grow: 2">Rebirth Upgrade</th>
        <th style="flex-grow: 1">Effect</th>
        <th style="flex-grow: 1">Cost</th>
      </tr>
      <tr
        v-for="[upgrade, upgrade_state] in visible_unbought_upgrades"
        :key="upgrade.name"
        :class="{ mydisabled: !upgrade_state.is_unlocked }"
        @click="$wasm.buy_rebirth_upgrade(upgrade.name)"
      >
        <td style="flex-grow: 2">{{ upgrade.display_name }}</td>
        <td style="flex-grow: 1">
          <icon-with-text :icon="upgrade.icon" :text="upgrade.effect_description" />
        </td>
        <td style="flex-grow: 1">
          <icon-with-text :icon="$world.icons['Coin']">
            <FormatNumber :value="upgrade.purchasing_cost" />
          </icon-with-text>
        </td>
      </tr>
    </table>
  </Section2>
</template>

<script>
import Section2 from './Section2.vue'
import FormatNumber from './FormatNumber.vue'
import { compare } from '../utility.js'
import { mapState } from 'vuex'

export default {
  components: { Section2, FormatNumber },
  computed: {
    ...mapState(['state', 'meta', 'input']),
    visible_unbought_upgrades() {
      let self = this
      return self.$world.rebirth_upgrades
        .map((w, i) => {
          return [w, self.state.rebirth_stats.rebirth_upgrades[i]]
        })
        .filter(([w, s]) => {
          return s.is_visible && !s.is_purchased
        })
        .sort(compare)
    },
    bought_upgrades() {
      let self = this
      return self.$world.rebirth_upgrades
        .map((w, i) => {
          return [w, self.state.rebirth_stats.rebirth_upgrades[i]]
        })
        .filter(([w, s]) => {
          return s.is_purchased
        })
        .sort(compare)
    },
  },
  methods: {
    toggle_show_bought() {
      this.$wasm.set_show_bought_upgrades(!this.meta.options.show_bought_upgrades)
    },
  },
}
</script>

<style scoped></style>
