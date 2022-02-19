<template>
  <div>
    <h4 class="section-header">Debug</h4>
    <div class="section">
      Presets
      <br />
      <b-dropdown id="dropdown-1" text="Presets" no-flip lazy>
        <b-dropdown-item-button
          v-for="name in presets"
          :key="name"
          @click="$wasm.set_preset_saves(name)"
          >{{ name }}</b-dropdown-item-button
        >
      </b-dropdown>
      <br />
      <button @click="$wasm.test">Test</button>
      <br />
      <button @click="$wasm.set_gamespeed(1)">Set GameSpeed 1</button>
      <br />
      <button @click="$wasm.set_gamespeed(10)">Set GameSpeed 10</button>
      <br />
      <button @click="$wasm.set_gamespeed(100)">Set GameSpeed 100</button>
      <br />
      <button @click="$wasm.set_gamespeed(1000)">Set GameSpeed 1000</button>
      <br />
      <b-button @click="tick">Tick</b-button>
      <br />
      <button @click="print_frontend_debug_state">Print Frontend Debug State</button>
      <br />
      <button @click="print_frontend_debug_world">Print Frontend Debug World</button>
      <br />
      <button @click="$wasm.print_debug_intermediate">Print Debug Intermediate</button>
      <br />
      <button @click="$wasm.print_debug_state">Print Debug State</button>
      <br />
      <button @click="$wasm.print_debug_meta">Print Debug Meta</button>
      <br />
      <br />
      <button @click="$wasm.grow_old">Grow Old</button>
      <br />
      <input v-model="money" size="10" />
      <br />
      <button @click="$wasm.give_money(money)">Give Money</button>
      <br />
      <input v-model="coins" size="10" />
      <br />
      <button @click="$wasm.give_coins(coins)">Give Coins</button>
      <br />
      <input v-model="divine_favor" size="10" />
      <br />
      <button @click="$wasm.give_divine_favor(divine_favor)">Give Divine Favor</button>
      <br />
      <br />
    </div>
  </div>
</template>

<script>
import { downloadFile } from '../utility.js'
export default {
  props: ['state', 'world'],
  data() {
    return {
      presets: [],
      money: 1000000000.0,
      coins: 1000000.0,
      divine_favor: 10000.0,
    }
  },
  mounted() {
    this.presets = this.$wasm.get_preset_saves()
  },
  methods: {
    print_frontend_debug_state() {
      console.log(this.state)
    },
    print_frontend_debug_world() {
      console.log(this.world)
    },
    tick() {
      this.$wasm.single_tick()
      this.$parent.update_dynamic_data()
    },
  },
}
</script>

<style scoped></style>
