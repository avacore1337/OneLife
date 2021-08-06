import Vue from "vue/dist/vue.js";
import("../pkg/index.js")
  .then(function (wasm) {
    const world = wasm.get_world();
    console.log(world);
    let state = wasm.get_state();
    console.log(state);
    var app_state = new Vue({
      el: "#app",
      data: {
        state: state,
        world: world,
      },

      mounted: function () {
        let self = this;
        setInterval(function () {
          wasm.tick();
          self.state = wasm.get_state();
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
      },
    });
  })
  .catch(console.error);
