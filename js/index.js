import Vue from "vue/dist/vue.js";
import("../pkg/index.js")
  .then(function (wasm) {
    const world = wasm.get_world();
    console.log(world);
    let state = wasm.get_state();
    var app_state = new Vue({
      el: "#app",
      data: {
        state: state,
        world: world,
      },

      methods: {
        save: function () {
          console.log("save");
          wasm.save();
        },
        hard_reset: function () {
          wasm.hard_reset();
          this.state = wasm.get_state();
        },
        load: function () {
          wasm.load();
          this.state = wasm.get_state();
        },
        tick: function (work_name) {
          console.log("tick");
          wasm.tick();
          this.state = wasm.get_state();
        },
        set_work: function (work_name) {
          wasm.set_work(work_name);
          console.log("Vue work: " + work_name);
        },
        buy_tier: function (index) {
          wasm.buy_tier(index);
          this.state = wasm.get_state();
        },
        rebirth: function () {
          wasm.do_rebirth();
          console.log("rebirth");
          this.state = wasm.get_state();
        },
        can_buy_tier: function (tier) {
          return wasm.can_buy_tier(tier.level);
        },
      },
    });
  })
  .catch(console.error);
