import("../pkg/index.js").then(function(wasm) {
const tick = document.getElementById("Tick");
tick.addEventListener("click", event => {
	console.log("tick");
	wasm.tick();
});
const hard_reset = document.getElementById("Hard Reset");
hard_reset.addEventListener("click", event => {
	console.log("hard reset");
	wasm.hard_reset();
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
});
    var app_state = new Vue({
      el: '#app_state',
      data: {
        state: wasm.get_state(),
      },

  // methods: {
  //   set_work: function (work_name) {
	// wasm.set_work(work_name);
	// console.log("Vue work: " + work_name);
  //   }
  // },
    })
const world = wasm.get_world();
console.log(world);
    var app = new Vue({
      el: '#app',
      data: {
        message: 'Hello Vue!',
        works: world.works,
      },

  methods: {
    set_work: function (work_name) {
	wasm.set_work(work_name);
	console.log("Vue work: " + work_name);
    }
  },
    })
}).catch(console.error);

