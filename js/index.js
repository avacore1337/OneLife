import Vue from 'vue/dist/vue.js';
import("../pkg/index.js").then(function(wasm) {
const hard_reset = document.getElementById("Hard Reset");
hard_reset.addEventListener("click", event => {
	console.log("hard reset");
	wasm.hard_reset();
	this.state = wasm.get_state();
});
const save = document.getElementById("Save");
save.addEventListener("click", event => {
	console.log("save");
	wasm.save();
});
const load = document.getElementById("Load");
load.addEventListener("click", event => {
	console.log("load");
	wasm.load();
	this.state = wasm.get_state();
});
const world = wasm.get_world();
console.log(world);
let state =  wasm.get_state();
    var app_state = new Vue({
      el: '#app',
      data: {
        state: state,
        works: world.works,
      },

  methods: {
    tick: function (work_name) {
	console.log("tick");
	wasm.tick();
	this.state = wasm.get_state();
    },
    set_work: function (work_name) {
	wasm.set_work(work_name);
	console.log("Vue work: " + work_name);
    },
    rebirth: function () {
	wasm.do_rebirth();
	console.log("rebirth");
	this.state = wasm.get_state();
    },
  },
    })


}).catch(console.error);

