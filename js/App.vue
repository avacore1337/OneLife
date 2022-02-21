<template>
  <div v-if="loaded" class="app">
    <b-modal ref="the-modal" hide-footer hide-header title="Using Component Methods">
      <span class="the-modal">
        <div class="d-block text-center">
          <span v-html="modalText"></span>
        </div>
        <hr />
        <b-button class="float-end" block @click="hideModal">Next</b-button>
        <b-button class="float-end" variant="danger" block @click="disable_tutorial"
          >Disable Tutorial</b-button
        >
      </span>
    </b-modal>
    <Topbar />
    <div class="my-container">
      <div class="left-sidebar">
        <Sidebar />
      </div>

      <div class="main">
        <Main />
      </div>

      <div class="right-sidebar">
        <SidebarRight />
        <Debug v-if="$world.settings.display_debug" />
      </div>
    </div>
  </div>
</template>

<script>
import Main from './components/Main.vue'
import Debug from './components/Debug.vue'
import SidebarRight from './components/SidebarRight.vue'
import Sidebar from './components/Sidebar.vue'
import Topbar from './components/Topbar.vue'
import { mapState } from 'vuex'

import 'bootstrap/dist/css/bootstrap.css'
import 'bootstrap-vue/dist/bootstrap-vue.css'

export default {
  components: {
    Main,
    Debug,
    Sidebar,
    SidebarRight,
    Topbar,
  },
  props: ['wasm', 'tutorial_data'],
  computed: mapState(['state', 'meta', 'input']),
  data() {
    return {
      loaded: false,
      numberFormat: 'DEFAULT',
      modalText: '',
      updateCount: 0,
    }
  },
  mounted() {
    this.loaded = true
    this.set_keyboard_listeners()

    let self = this
    setInterval(function () {
      if (
        self.meta.options.paused ||
        self.state.life_stats.is_dying ||
        self.state.life_stats.dead
      ) {
        self.wasm.paused()
      } else {
        self.wasm.tick()
      }
      self.do_update()
    }, 1000 / 30)
  },
  methods: {
    set_keyboard_listeners() {
      window.addEventListener('keydown', (event) => {
        if (event.code == 'Space') {
          event.preventDefault()
          this.wasm.toggle_paused()
          this.do_update()
        }
      })
    },
    do_update() {
      this.updateCount += 1
      if (this.updateCount % this.meta.options.update_rate === 0) {
        this.$store.commit('update_dynamic_data')
        this.updateModal()
      }
    },
    updateModal() {
      let modal = this.$refs['the-modal']
      if (this.meta.info.show_tutorial && modal.isHidden) {
        console.log('update modal')
        this.modalText = this.tutorial_data[this.meta.info.tutorial_step]
        modal.show()
      }
    },
    showModal() {
      this.$refs['the-modal'].show()
    },
    hideModal() {
      this.wasm.next_info_step()
      this.$refs['the-modal'].hide()
    },
    toggleModal() {},
    disable_tutorial() {
      this.wasm.set_disable_tutorial(true)
      this.wasm.next_info_step()
      this.$refs['the-modal'].hide()
    },
  },
}
</script>

<style>
body {
  background-color: #232c3a;
  color: white;
  overflow-x: hidden;
}

span.the-modal {
  white-space: pre-line;
  color: black;
}

.my-container {
  display: flex;
  /* max-width: 1920; */
  margin: 0 auto;
  gap: 10px;
  padding: 10px;
}

.left-sidebar {
  width: 280px;
  flex-shrink: 0;
}

.right-sidebar {
  width: 280px;
  flex-shrink: 0;
}

.main {
  flex-grow: 1;
}

div.app {
  /* width: 100vw; */
  /* height: 100vh; */
}
</style>
