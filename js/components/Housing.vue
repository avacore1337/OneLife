<template>
  <Section2 title="Housing">
    <table>
      <tr class="header-row">
        <th>Housing</th>
        <th>Happiness</th>
        <th>Health</th>
        <th>Cost</th>
      </tr>
      <tr
        v-for="[housing, housing_state] in visible_housing"
        :key="housing.name"
        :class="{ mydisabled: !housing_state.is_unlocked }"
        @click="$wasm.set_housing(housing.name)"
      >
        <td>
          <span :class="{ selected: input.housing === housing.name }"
            >{{ housing.display_name }}
          </span>
        </td>
        <td v-if="!housing_state.is_unlocked">
          Required Money: <FormatNumber :value="housing.required_money" />
        </td>
        <td v-if="housing_state.is_unlocked">{{ housing.happiness_factor }}X</td>
        <td v-if="housing_state.is_unlocked">
          {{ housing.health_effect }}
        </td>
        <td v-if="housing_state.is_unlocked"><FormatNumber :value="housing.upkeep" />/s</td>
      </tr>
    </table>
  </Section2>
</template>

<script>
import Section2 from './Section2.vue'
import FormatNumber from './FormatNumber.vue'
import { mapState } from 'vuex'

export default {
  components: { Section2, FormatNumber },
  computed: {
    ...mapState(['state', 'meta', 'input']),
    visible_housing() {
      let self = this
      return self.$world.housing
        .map((w, i) => {
          return [w, self.state.housing[i]]
        })
        .filter(([w, s]) => {
          return s.is_visible
        })
    },
  },
  methods: {
    toggle_auto_living() {
      this.$wasm.set_auto_living(!this.meta.options.auto_living)
    },
  },
}
</script>

<style scoped></style>
