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
}).catch(console.error);

