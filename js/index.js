import Vue from "vue/dist/vue.js";
import("../pkg/index.js")
  .then(function (wasm) {
    const world = wasm.get_world();
    console.log(world);
    wasm.load();
    let state = wasm.get_state();
    let input = wasm.get_state();
    console.log(state);
    var app_state = new Vue({
      el: "#app",
      data: {
        state: state,
        input: input,
        world: world,
      },

      mounted: function () {
        let self = this;
        setInterval(function () {
          wasm.tick();
          self.state = wasm.get_state();
          self.input = wasm.get_input();
          // console.log(self);
          // console.log("app");
          // self.$forceUpdate();
          // this code runs every second
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
        rebirth: function () {
          wasm.do_rebirth();
          console.log("rebirth");
        },
        can_buy_tier: function (tier) {
          return wasm.can_buy_tier(tier.level);
        },
        prettyPrint: function (value) {
          if (typeof value !== 'number') {
            return value;
          }

          if (value < 1000) {
            return Number.parseFloat(value.toFixed(1)).toString();
          }

          const ending = ['K', 'M', 'B', 'T', 'Qa', 'Qi', 'He', 'Se', 'Oc', 'No', 'De'];
          let index = -1;
          while (value >= 1000 && index < ending.length - 1) {
            value /= 1000;
            index++;
          }

          return Number.parseFloat(value.toFixed(1)).toString() + ending[index];
        }
      },
    });
  })
  .catch(console.error);
