import("../pkg/index.js").then(function(wasm) {
const mines = document.getElementById("Mines");
mines.addEventListener("click", event => {
	console.log("mines");
	wasm.work_mines();
});
const fields = document.getElementById("Fields");
fields.addEventListener("click", event => {
	console.log("fields");
	wasm.work_fields();
});
const servant = document.getElementById("Servant");
servant.addEventListener("click", event => {
	console.log("servant");
	wasm.work_servant();
});
const tick = document.getElementById("Tick");
tick.addEventListener("click", event => {
	console.log("tick");
	wasm.tick();
});
}).catch(console.error);

