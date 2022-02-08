import Vuex from 'vuex'
import Vue from 'vue/dist/vue.js'

Vue.use(Vuex)

export const store = new Vuex.Store({
  state: {
    count: 0,
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
