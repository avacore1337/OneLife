<template>
  <div>
    <h3>Settings</h3>
    <br />
    <button @click="wasm.save">Save</button>
    <br />
    <button @click="wasm.load">Load</button>
    <br />
    <button @click="wasm.hard_reset">Hard Reset</button>
    <br />
    <input id="autosave" type="checkbox" :checked="metaData.autosave" @click="toggleAutoSave" />
    <label for="autosave">Autosave</label>
    <br />
    Saved Ticks: <FormatNumber :value="metaData.saved_ticks" />
    <br />
    <button style="margin: 2px" @click="toggle_use_saved_ticks">
      {{ !metaData.use_saved_ticks ? 'Use Saved Ticks' : "Don't Use Saved Ticks" }}
    </button>

    <br />
    <button @click="wasm.set_disable_tutorial(false)">Enable Tutorial</button>
    <br />
    <button style="margin: 2px" @click="toggle_show_recorded">
      {{ !metaData.options.show_recorded ? 'Show Recorded' : "Don't Show Recorded" }}
    </button>
    <br />
    <button style="margin: 2px" @click="toggle_pause">
      {{ metaData.options.paused ? 'Resume the game' : 'Pause the game' }}
    </button>
    <br />
    <br />
    FPS Settings
    <br />
    <button @click="wasm.set_update_rate(1)">30</button>
    <button @click="wasm.set_update_rate(2)">15</button>
    <button @click="wasm.set_update_rate(3)">10</button>
    <button @click="wasm.set_update_rate(10)">3</button>
    <button @click="wasm.set_update_rate(30)">1</button>
    <br />
    <br />
    <button style="margin: 2px" @click="download_save">Download Save</button>
    <br />
    <div style="max-width: 1000px">
      <b-form-textarea
        id="textarea"
        v-model="save_text"
        placeholder="Paste save here"
        rows="6"
        max-rows="6"
      ></b-form-textarea>
    </div>
    <button style="margin: 2px" @click="import_save">Import Save</button>
    <button style="margin: 2px" @click="export_save">Export Save</button>
    <br />
    <button @click="setNumberFormat">
      {{ nextNumberFormat }}
    </button>
  </div>
</template>

<script>
import { downloadFile } from '../utility.js'
import FormatNumber from './FormatNumber.vue'
export default {
  components: { FormatNumber },
  props: ['metaData', 'state', 'world', 'input', 'wasm'],
  data() {
    return {
      save_text: '',
    }
  },
  computed: {
    nextNumberFormat() {
      return this.$store.getters.getNextNumberFormat
    },
  },
  methods: {
    toggle_use_saved_ticks() {
      this.wasm.use_saved_ticks(!this.metaData.use_saved_ticks)
    },
    toggle_show_recorded() {
      this.wasm.set_show_recorded(!this.metaData.options.show_recorded)
    },
    toggle_pause() {
      this.wasm.set_paused(!this.metaData.options.paused)
    },
    download_save() {
      // TODO: This should be exported by the backend
      downloadFile(`gamesave_${Date.now()}.txt`, this.wasm.export_save())
    },
    import_save_file(event) {
      // TODO: This is only on the frontend atm, it doesn't actually save the changes
      var files = event.target.files
      var f = files[0]
      var reader = new FileReader()
      var self = this

      reader.onload = (function (theFile) {
        return function (e) {
          var data = e.target.result
          console.log(data)
          this.wasm.import_save(data)
        }
      })(f)
      reader.readAsText(f)
    },
    import_save() {
      this.wasm.import_save(this.save_text)
    },
    export_save() {
      this.save_text = this.wasm.export_save()
    },
    toggleAutoSave() {
      this.wasm.set_autosave(!this.metaData.autosave)
    },
    setNumberFormat() {
      this.$store.commit('toggleNumberFormat')
    },
  },
}
</script>

<style scoped></style>
