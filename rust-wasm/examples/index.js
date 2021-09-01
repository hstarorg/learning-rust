import("../pkg/rust_wasm.js")
  .then((wasm) => {
    window.wasmIns = wasm;
    wasm.print("Hi WebAssembly by rust!");
  })
  .catch(console.error);

document.querySelector("#btn1").addEventListener(
  "click",
  () => {
    wasmIns.wasm_alert("Hi rust!");
  },
  false
);
