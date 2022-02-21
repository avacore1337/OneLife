import Vuex from 'vuex'
import Vue from 'vue/dist/vue.js'
import { tutorial_data } from './tutorial.js'

Vue.use(Vuex)

export const createStore = function (wasm) {
  return new Vuex.Store({
    state: {
      wasm: wasm,
      numberFormat: 'DEFAULT',
      state: wasm.get_state(),
      input: wasm.get_input(),
      recorded_inputs: wasm.get_recorded_inputs(),
      previous_recorded_inputs: wasm.get_previous_recorded_inputs(),
      item_queue: wasm.get_world_item_queue(),
      meta: wasm.get_meta_data(),
      tutorial_data: Object.freeze(tutorial_data),
    },
    mutations: {
      toggleNumberFormat(state) {
        state.numberFormat = {
          DEFAULT: 'SCIENTIFIC',
          SCIENTIFIC: 'DEFAULT',
        }[state.numberFormat]
      },

      update_dynamic_data(state) {
        recurse_update(state.state, state.wasm.get_state())
        recurse_update(state.input, state.wasm.get_input())
        recurse_update(state.meta, state.wasm.get_meta_data())
        if (state.meta.options.show_recorded) {
          let recorded = state.wasm.get_recorded_inputs()
          if (state.recorded_inputs.length != recorded.length) {
            state.recorded_inputs = recorded
          } else {
            recurse_update(state.recorded_inputs, recorded)
          }
          let previous_recorded = state.wasm.get_previous_recorded_inputs()
          if (state.previous_recorded_inputs.length != previous_recorded.length) {
            state.previous_recorded_inputs = previous_recorded
          } else {
            recurse_update(state.previous_recorded_inputs, previous_recorded)
          }
        }
        let queue = state.wasm.get_world_item_queue()
        if (state.item_queue.length != queue.length) {
          state.item_queue = queue
        } else {
          recurse_update(state.item_queue, queue)
        }
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

function recurse_update(o, o2) {
  for (var key in o2) {
    if (Array.isArray(o2[key])) {
      if (o[key].length != o2[key].length) {
        /* console.log(key, o[key], o2[key]) */
        o[key] = o2[key]
        continue
      }
    }
    if (typeof o2[key] == 'object') {
      recurse_update(o[key], o2[key])
      continue
    }
    if (o[key] != o2[key]) {
      /* console.log(typeof o[key]); */
      /* console.log(key, o[key], o2[key]); */
      o[key] = o2[key]
    }
  }
}
