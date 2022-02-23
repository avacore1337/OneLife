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

Vue.component('VIcon', Icon)
Vue.component('IconWithText', IconWithText)
Vue.config.devtools = false
Vue.config.performance = false

import('../pkg/index.js')
  .then(function (wasm) {
    Vue.prototype.$wasm = wasm
    let world = Object.freeze(wasm.get_world())
    console.log('debug:', world.settings.display_debug)
    Vue.prototype.$world = world
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
