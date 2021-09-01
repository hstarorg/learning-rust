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

function fib(n) {
  if (n === 1 || n === 2) {
    return 1;
  } else {
    return fib(n - 1) + fib(n - 2);
  }
}

document.querySelector("#btn2").addEventListener(
  "click",
  () => {
    const N = 35;

    console.info("原生 JS 计算（t1）：");
    console.time("t1");
    fib(N);
    console.timeEnd("t1");

    console.log("\n");

    console.info("Rust Wasm 计算（t2）：");
    console.time("t2");
    wasmIns.fib(N);
    console.timeEnd("t2");

    console.log("\n\n");
  },
  false
);
