<template>
  <div v-if="loaded" class="app">
    <b-modal ref="the-modal" hide-footer hide-header title="Using Component Methods">
      <span class="the-modal">
        <div class="d-block text-center">
          <h3>{{ modalText }}</h3>
        </div>
        <b-button class="float-end" block @click="hideModal">Next</b-button>
        <b-button class="float-end" variant="danger" block @click="disable_tutorial"
          >Disable Tutorial</b-button
        >
      </span>
    </b-modal>
    <div>
      <Topbar />
    </div>
    <div class="my-container">
      <div class="left-sidebar">
        <Sidebar :state="state" :input="input" meta="meta" />
      </div>

      <div class="main">
        <Main
          :item_queue="item_queue"
          :meta="meta"
          :state="state"
          :input="input"
          :wasm="wasm"
          :previous_recorded_inputs="previous_recorded_inputs"
          :recorded_inputs="recorded_inputs"
        />
      </div>

      <div class="right-sidebar">
        <div>
          <SidebarRight :meta="meta" :state="state" :input="input" :item_queue="item_queue" />
        </div>
        <div v-if="$world.settings.display_debug">
          <Debug :state="state" />
        </div>
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
  props: ['wasm'],
  data() {
    return {
      loaded: false,
      world: {},
      state: {},
      input: {},
      recorded_inputs: {},
      previous_recorded_inputs: {},
      item_queue: [],
      meta: {},
      numberFormat: 'DEFAULT',
      modalText: '',
      updateCount: 0,
    }
  },
  mounted() {
    this.$world = Object.freeze(this.wasm.get_world())
    this.state = this.wasm.get_state()
    this.input = this.wasm.get_input()
    this.recorded_inputs = this.wasm.get_recorded_inputs()
    this.previous_recorded_inputs = this.wasm.get_previous_recorded_inputs()
    this.meta = this.wasm.get_meta_data()
    this.item_queue = this.wasm.get_world_item_queue()
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
    recurse_update(o, o2) {
      for (var key in o2) {
        if (Array.isArray(o2[key])) {
          if (o[key].length != o2[key].length) {
            /* console.log(key, o[key], o2[key]) */
            o[key] = o2[key]
            continue
          }
        }
        if (typeof o2[key] == 'object') {
          this.recurse_update(o[key], o2[key])
          continue
        }
        if (o[key] != o2[key]) {
          /* console.log(typeof o[key]); */
          /* console.log(key, o[key], o2[key]); */
          o[key] = o2[key]
        }
      }
    },
    do_update() {
      this.updateCount += 1
      if (this.updateCount % this.meta.options.update_rate === 0) {
        this.update_dynamic_data()
        this.updateModal()
      }
    },
    update_dynamic_data() {
      this.recurse_update(this.state, this.wasm.get_state())
      this.recurse_update(this.input, this.wasm.get_input())
      this.recurse_update(this.meta, this.wasm.get_meta_data())
      if (this.meta.options.show_recorded) {
        let recorded = this.wasm.get_recorded_inputs()
        if (this.recorded_inputs.length != recorded.length) {
          this.recorded_inputs = recorded
        } else {
          this.recurse_update(this.recorded_inputs, recorded)
        }
        let previous_recorded = this.wasm.get_previous_recorded_inputs()
        if (this.previous_recorded_inputs.length != previous_recorded.length) {
          this.previous_recorded_inputs = previous_recorded
        } else {
          this.recurse_update(this.previous_recorded_inputs, previous_recorded)
        }
      }
      let queue = this.wasm.get_world_item_queue()
      if (this.item_queue.length != queue.length) {
        this.item_queue = queue
      } else {
        this.recurse_update(this.item_queue, queue)
      }
    },
    updateModal() {
      let modal = this.$refs['the-modal']
      if (this.meta.info.show_tutorial && modal.isHidden) {
        console.log('update modal')
        this.modalText = this.$world.tutorial_texts[this.meta.info.tutorial_step]
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
