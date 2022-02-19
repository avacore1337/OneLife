<template>
  <Section2>
    <table>
      <tr class="header-row">
        <th style="flex-grow: 3">Blessing</th>
        <th style="flex-grow: 3">Effect</th>
        <th>Level</th>
        <th>Cost</th>
      </tr>
      <tr
        v-for="[blessing, blessing_state] in visible_blessings"
        :key="blessing.name"
        :class="{ mydisabled: !blessing_state.is_unlocked }"
        @click="wasm.buy_blessing(blessing.name)"
      >
        <td style="flex-grow: 3">
          <icon-with-text :icon="blessing.icon" :text="blessing.display_name" />
        </td>
        <td style="flex-grow: 3">
          {{ blessing_state.effect_description }}
        </td>
        <td>{{ blessing_state.level }},</td>
        <td>
          <icon-with-text :icon="$world.icons['DivineFavor']">
            <FormatNumber :value="blessing_state.next_level_cost" />
          </icon-with-text>
        </td>
      </tr>
    </table>
  </Section2>
</template>

<script>
import Section2 from './Section2.vue'
import FormatNumber from './FormatNumber.vue'
export default {
  components: { Section2, FormatNumber },
  props: ['metaData', 'state', 'input', 'wasm'],
  computed: {
    visible_blessings() {
      let self = this
      return self.$world.blessings
        .map((w, i) => {
          return [w, self.state.blessings[i]]
        })
        .filter(([w, s]) => {
          return s.is_visible
        })
    },
  },
  methods: {
    toggle_auto_buy_blessing() {
      this.wasm.set_auto_buy_blessing(!this.metaData.options.auto_buy_blessing)
    },
  },
}
</script>

<style scoped></style>
