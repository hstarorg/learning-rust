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

function fib_tail(n, nValue, total) {
  if (n === 1 || n === 2) {
    return nValue;
  } else {
    return fib_tail(n - 1, total, nValue + total);
  }
}

document.querySelector("#btn2").addEventListener(
  "click",
  () => {
    const N = Number(document.querySelector("#in").value);
    // 字符串不合法
    if (N !== N || N < 1 || parseInt(N, 10) !== N) {
      alert("输入不合法，需要提供正整数");
      return;
    }

    console.info("原生 JS 计算（t1）：");
    console.time("t1");
    fib(N);
    // 如下是尾调用优化版本
    // fib_tail(N, 1, 2);
    console.timeEnd("t1");

    console.log("\n");

    console.info("Rust Wasm 计算（t2）：");
    console.time("t2");
    wasmIns.fib(N);
    // 如下是尾调用优化版本
    wasmIns.fib_tail(N, 1, 2);
    console.timeEnd("t2");

    console.log("\n\n");
  },
  false
);
