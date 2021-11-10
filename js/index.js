import Vue from "vue/dist/vue.js";
import { BootstrapVue } from "bootstrap-vue";
import { downloadFile } from "./utility.js";

import "../css/styles.css";
import "bootstrap/dist/css/bootstrap.css";
import "bootstrap-vue/dist/bootstrap-vue.css";

Vue.use(BootstrapVue);

import("../pkg/index.js")
  .then(function (wasm) {
    const world = wasm.get_world();
    wasm.load();
    let state = wasm.get_state();
    let input = wasm.get_input();
    let presets = wasm.get_preset_saves();
    var app_state = new Vue({
      el: "#app",
      data: {
        state: state,
        input: input,
        world: world,
        presets: presets,
        paused: false,
        numberFormat: "DEFAULT",
      },
      mounted: function () {
        document.getElementById("import_save_button").addEventListener("change", this.import_save, false);
        let self = this;
        setInterval(function () {
          if (self.paused) {
            return;
          }

          wasm.tick();
          self.state = wasm.get_state();
          self.input = wasm.get_input();
        }, 1000 / 30);
      },
      methods: {
        printableNumbers: function (num) {
          if (num < 100) {
            return num.toFixed(1);
          }
          if (num < 10000) {
            return Math.floor(num).toString();
          }

          if (this.numberFormat === "SCIENTIFIC") {
            let exponent = 1;
            while (num >= 10) {
              num /= 10;
              exponent++;
            }

            return `${num.toFixed(1)}e${exponent}`;
          }

          const ending = ["K", "M", "B", "T", "Qa", "Qi", "He", "Se", "Oc", "No", "De"];
          let index = -1;
          while (num >= 10000 && index < ending.length - 1) {
            num /= 1000;
            index++;
          }

          return `${num.toFixed(1)}${ending[index]}`;
        },
        nextNumberFormat: function (numberFormat) {
          return {
            DEFAULT: "Scientific notation",
            SCIENTIFIC: "Natural numbers",
          }[numberFormat];
        },
        setNumberFormat: function () {
          this.numberFormat = {
            DEFAULT: "SCIENTIFIC",
            SCIENTIFIC: "DEFAULT",
          }[this.numberFormat];
        },
        save: function () {
          wasm.save();
        },
        print_debug: function () {
          wasm.print_debug();
        },
        hard_reset: function () {
          wasm.hard_reset();
        },
        load: function () {
          wasm.load();
        },
        set_gamespeed: function (game_speed) {
          wasm.set_gamespeed(game_speed);
        },
        tick: function (work_name) {
          wasm.single_tick();

          this.state = wasm.get_state();
          this.input = wasm.get_input();
        },
        set_work: function (work_name) {
          wasm.set_work(work_name);
        },
        set_housing: function (housing_name) {
          wasm.set_housing(housing_name);
        },
        buy_tier: function (index) {
          wasm.buy_tier(index);
        },
        load_preset: function (preset) {
          wasm.set_preset_saves(preset);
        },
        rebirth: function () {
          wasm.do_rebirth();
        },
        toggle_pause: function () {
          this.paused = !this.paused;
        },
        set_autosave: function (autosave) {
          wasm.set_autosave(autosave);
        },
        can_buy_tier: function (tier) {
          return wasm.can_buy_tier(tier.level);
        },
        export_save: function () {
          // TODO: This should be exported by the backend
          downloadFile(`gamesave_${Date.now()}.txt`, wasm.export_save());
        },
        import_save: function (event) {
          // TODO: This is only on the frontend atm, it doesn't actually save the changes
          var files = event.target.files;
          var f = files[0];
          var reader = new FileReader();
          var self = this;

          reader.onload = (function (theFile) {
            return function (e) {
              var data = e.target.result;
              console.log(data);
              wasm.import_save(data);
            };
          })(f);
          reader.readAsText(f);
        },
        prettyPrintDays: function (total_days) {
          const years = Math.floor(total_days / 365);
          const days = total_days % 365;

          if (years === 0) {
            return `${days} days`;
          } else if (days === 0) {
            return `${years} years`;
          }
          return `${years} years and ${days.toFixed(0)} days`;
        },
        prettyPrint: function (value) {
          if (typeof value !== "number") {
            return value;
          }

          return this.printableNumbers(value);
        },
      },
    });
  })
  .catch(console.error);
