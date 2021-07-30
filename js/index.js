import("../pkg/index.js").then(function(wasm) {
const mines = document.getElementById("Mines");
mines.addEventListener("click", event => {
	console.log("mines");
	wasm.set_work("Mines");
});
const fields = document.getElementById("Fields");
fields.addEventListener("click", event => {
	console.log("fields");
	wasm.set_work("Fields");
});
const servant = document.getElementById("Servant");
servant.addEventListener("click", event => {
	console.log("servant");
	wasm.set_work("Servant");
});
const tick = document.getElementById("Tick");
tick.addEventListener("click", event => {
	console.log("tick");
	wasm.tick();
});
}).catch(console.error);

