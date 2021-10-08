import Vue from "vue/dist/vue.js";
import("../pkg/index.js")
  .then(function (wasm) {
    const world = wasm.get_world();
    console.log(world);
    wasm.load();
    let state = wasm.get_state();
    let input = wasm.get_input();
    let presets = wasm.get_preset_saves();
    console.log(JSON.stringify(input, undefined, 2));
    console.log(JSON.stringify(state, undefined, 2));
    // console.log(JSON.stringify(presets, undefined, 2));
    var app_state = new Vue({
      el: "#app",
      data: {
        state: state,
        input: input,
        world: world,
        presets: presets,
        paused: false,
      },

      mounted: function () {
        let self = this;
        setInterval(function () {
          if (self.paused) {
            return;
          }

          wasm.tick();
          self.state = wasm.get_state();
          self.input = wasm.get_input();
        }, 100);
      },
      methods: {
        save: function () {
          console.log("save");
          wasm.save();
        },
        hard_reset: function () {
          wasm.hard_reset();
        },
        load: function () {
          wasm.load();
        },
        tick: function (work_name) {
          console.log("tick");
          wasm.tick();
        },
        set_work: function (work_name) {
          wasm.set_work(work_name);
          console.log("Vue work: " + work_name);
        },
        set_housing: function (housing_name) {
          wasm.set_housing(housing_name);
          console.log("Vue work: " + housing_name);
        },
        buy_tier: function (index) {
          wasm.buy_tier(index);
        },
        load_preset: function (preset) {
          this.state = preset[0];
          this.input = preset[1];
        },
        rebirth: function () {
          wasm.do_rebirth();
          console.log("rebirth");
        },
        toggle_pause: function () {
          this.paused = !this.paused;
        },
        can_buy_tier: function (tier) {
          return wasm.can_buy_tier(tier.level);
        },
        prettyPrint: function (value) {
          if (typeof value !== "number") {
            return value;
          }

          if (value < 1000) {
            return Number.parseFloat(value.toFixed(1)).toString();
          }

          const ending = ["K", "M", "B", "T", "Qa", "Qi", "He", "Se", "Oc", "No", "De"];
          let index = -1;
          while (value >= 1000 && index < ending.length - 1) {
            value /= 1000;
            index++;
          }

          return Number.parseFloat(value.toFixed(1)).toString() + ending[index];
        },
      },
    });
  })
  .catch(console.error);
