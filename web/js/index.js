let imp = import('../pkg');

imp
    .then(wasm => {
        window.wasm = wasm;
        wasm.helo("hello, world!")
    })
    .catch(console.error);
