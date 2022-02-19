import Vuex from 'vuex'
import Vue from 'vue/dist/vue.js'

Vue.use(Vuex)

export const createStore = function (wasm) {
  return new Vuex.Store({
    state: {
      wasm: wasm,
      numberFormat: 'DEFAULT',
    },
    mutations: {
      toggleNumberFormat(state) {
        state.numberFormat = {
          DEFAULT: 'SCIENTIFIC',
          SCIENTIFIC: 'DEFAULT',
        }[state.numberFormat]
      },
    },
    getters: {
      getNextNumberFormat(state) {
        return {
          DEFAULT: 'Scientific notation',
          SCIENTIFIC: 'Natural numbers',
        }[state.numberFormat]
      },
    },
  })
}
