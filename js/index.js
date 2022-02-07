import App from './App.vue'
import MyIcon from './components/MyIcon.vue'

import { store }from './store.js'

import { BootstrapVue } from 'bootstrap-vue'
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
Vue.use(BootstrapVue)

Vue.config.performance = true
Vue.component('VIcon', Icon)
Vue.component('MyIcon', MyIcon)

import('../pkg/index.js')
  .then(function (wasm) {
    wasm.load()
    new Vue({
      el: '#app',
      store,
      render: (h) =>
        h(App, {
          props: { wasm },
        }),
    })
  })
  .catch(console.error)
