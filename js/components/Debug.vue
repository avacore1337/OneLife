<template>
  <div>
    <h3>Debug</h3>
    <br />
    Presets
    <div>
      <b-dropdown id="dropdown-1" text="Presets" no-flip class="m-md-2" lazy>
        <b-dropdown-item-button v-for="name in presets" :key="name" @click="wasm.set_preset_saves(name)">{{
          name
        }}</b-dropdown-item-button>
      </b-dropdown>
    </div>
    <br />
    <button v-on:click="wasm.test">Test</button>
    <br />
    <button v-on:click="wasm.set_gamespeed(1)">Set GameSpeed 1</button>
    <br />
    <button v-on:click="wasm.set_gamespeed(10)">Set GameSpeed 10</button>
    <br />
    <button v-on:click="wasm.set_gamespeed(100)">Set GameSpeed 100</button>
    <br />
    <button v-on:click="wasm.set_gamespeed(1000)">Set GameSpeed 1000</button>
    <br />
    <br />
    <button v-on:click="print_frontend_debug_state">Print Frontend Debug State</button>
    <br />
    <button v-on:click="print_frontend_debug_world">Print Frontend Debug World</button>
    <br />
    <button v-on:click="wasm.print_debug_intermediate">Print Debug Intermediate</button>
    <br />
    <button v-on:click="wasm.print_debug_state">Print Debug State</button>
    <br />
    <button v-on:click="wasm.print_debug_meta">Print Debug Meta</button>
    <br />
    <br />
    <button v-on:click="wasm.grow_old">Grow Old</button>
    <br />
    <input v-model="money" size="10" />
    <br />
    <button v-on:click="wasm.give_money(money)">Give Money</button>
    <br />
    <input v-model="coins" size="10" />
    <br />
    <button v-on:click="wasm.give_coins(coins)">Give Coins</button>
    <br />
    <input v-model="divine_favor" size="10" />
    <br />
    <button v-on:click="wasm.give_divine_favor(divine_favor)">Give Divine Favor</button>
    <br />
    <br />
  </div>
</template>

<script>
import { downloadFile } from "../utility.js";
export default {
  props: ["metaData", "state", "world", "input", "wasm"],
  data() {
    return {
      presets: [],
      money: 1000000000.0,
      coins: 1000000.0,
      divine_favor: 10000.0,
    };
  },
  mounted: function () {
    this.presets = this.wasm.get_preset_saves();
  },
  methods: {
    print_frontend_debug_state: function () {
      console.log(this.state);
    },
    print_frontend_debug_world: function () {
      console.log(this.world);
    },
  },
};
</script>

<style scoped></style>
