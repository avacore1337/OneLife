import App from './App.vue'
import IconWithText from './components/IconWithText.vue'

import { createStore } from './store.js'

import { BootstrapVue, BVConfig } from 'bootstrap-vue'
import Vue from 'vue/dist/vue.js'
import VueGtag from 'vue-gtag'
import Icon from 'vue-awesome/components/Icon'

import 'vue-awesome/icons'
import '../css/styles.css'
import 'bootstrap/dist/css/bootstrap.css'
import 'bootstrap-vue/dist/bootstrap-vue.css'

Vue.use(VueGtag, {
  config: { id: 'G-JW5LS1NXT6' },
})

Vue.use(BVConfig, {
  BTooltip: {
    delay: {
      show: 400,
      hide: 100,
    },
    boundaryPadding: 80,
  },
})

Vue.use(BootstrapVue)

Vue.config.performance = true
Vue.component('VIcon', Icon)
Vue.component('IconWithText', IconWithText)

import('../pkg/index.js')
  .then(function (wasm) {
    Vue.prototype.$wasm = wasm
    Vue.prototype.$world = Object.freeze(wasm.get_world())
    wasm.load()
    new Vue({
      el: '#app',
      store: createStore(wasm),
      render: (h) =>
        h(App, {
          props: { wasm },
        }),
    })
  })
  .catch(console.error)
