<template>
  <Section title="Housing">
    <b-button v-if="state.rebirth_stats.unlocks.can_auto_living" v-on:click="toggle_auto_living" size="sm">
      {{ !metaData.options.auto_living ? "Auto Living" : "Don't Auto Living" }}
    </b-button>
    <table>
      <tr
        v-for="[housing, housing_state] in visible_housing"
        v-on:click="wasm.set_housing(housing.name)"
        v-bind:class="{ disabled: !housing_state.is_unlocked }"
        :key="housing.name"
      >
        <td>
          <p>
            <span v-bind:class="{ selected: input.housing === housing.name }">{{ housing.display_name }} </span>
            <span v-if="!housing_state.is_unlocked" style="float: right"
              >Required Money: {{ housing.required_money }}
            </span>
            <span style="float: right">Cost: {{ housing.upkeep }}/s </span>
            <br />
            Happiness boost: {{ housing.happiness_factor }},
            <span v-if="housing.health_effect != 0.0"> Health Effect: {{ housing.health_effect }} </span>
          </p>
        </td>
      </tr>
    </table>
  </Section>
</template>

<script>
import Section from "./Section.vue";

export default {
  props: ["metaData", "state", "world", "input", "wasm"],
  components: { Section },
  methods: {
    toggle_auto_living: function () {
      this.wasm.set_auto_living(!this.metaData.options.auto_living);
    },
  },
  computed: {
    visible_housing: function () {
      let self = this;
      return self.world.housing
        .map((w, i) => {
          return [w, self.state.housing[i]];
        })
        .filter(([w, s]) => {
          return s.is_visible;
        });
    },
  },
};
</script>

<style scoped></style>
