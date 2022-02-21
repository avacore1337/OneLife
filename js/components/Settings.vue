<template>
  <div class="column-flex">
    <h3>Settings</h3>
    <b-button @click="$wasm.save">Save</b-button>
    <b-button @click="$wasm.load">Load</b-button>
    <b-button @click="$wasm.hard_reset">Hard Reset</b-button>
    <MyToggle :value="meta.autosave" :click="$wasm.toggle_autosave"> AutoSave </MyToggle>
    <h4>Display options</h4>
    <b-button @click="$wasm.toggle_disable_tutorial">
      {{ meta.info.disable_tutorial ? 'Enable Tutorial' : 'Disable Tutorial' }}
    </b-button>
    <b-button @click="$wasm.toggle_show_recorded">
      {{ !meta.options.show_recorded ? 'Show Recorded' : "Don't Show Recorded" }}
    </b-button>
    <b-button @click="setNumberFormat">
      {{ nextNumberFormat }}
    </b-button>
    FPS Settings
    <div>
      <b-button-group>
        <b-button
          v-for="button in buttons"
          :key="button.update_rate"
          :pressed="meta.options.update_rate == button.update_rate"
          @click="set_update_rate(button.update_rate)"
          >{{ button.fps }}</b-button
        >
      </b-button-group>
    </div>
    <h4>Import/Export saves</h4>
    <b-button @click="download_save">Download Save</b-button>
    <div style="max-width: 1000px">
      <b-form-textarea
        id="textarea"
        v-model="save_text"
        placeholder="Paste save here"
        rows="6"
        max-rows="6"
      ></b-form-textarea>
    </div>
    <b-button @click="import_save">Import Save</b-button>
    <b-button @click="export_save">Export Save</b-button>
  </div>
</template>

<script>
import { downloadFile } from '../utility.js'
import FormatNumber from './FormatNumber.vue'
import MyToggle from './MyToggle.vue'
import { mapState } from 'vuex'

export default {
  components: { FormatNumber, MyToggle },
  data() {
    return {
      save_text: '',
      buttons: [
        { fps: '30', update_rate: 1 },
        { fps: '15', update_rate: 2 },
        { fps: '10', update_rate: 3 },
        { fps: '3', update_rate: 10 },
        { fps: '1', update_rate: 30 },
      ],
    }
  },
  computed: {
    ...mapState(['state', 'meta', 'input']),
    nextNumberFormat() {
      return this.$store.getters.getNextNumberFormat
    },
  },
  methods: {
    set_update_rate(rate) {
      this.$wasm.set_update_rate(rate)
      this.$store.commit('update_dynamic_data')
    },
    download_save() {
      downloadFile(`gamesave_${Date.now()}.txt`, this.$wasm.export_save())
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
          this.$wasm.import_save(data)
        }
      })(f)
      reader.readAsText(f)
    },
    import_save() {
      this.$wasm.import_save(this.save_text)
    },
    export_save() {
      this.save_text = this.$wasm.export_save()
    },
    setNumberFormat() {
      this.$store.commit('toggleNumberFormat')
    },
  },
}
</script>

<style scoped>
button:focus {
  outline: none;
  box-shadow: none;
}

button {
  max-width: 200px;
  flex-grow: 0;
}

.column-flex {
  display: flex;
  flex-direction: column;
  gap: 5px;
}
</style>
