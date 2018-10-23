const wasm = import("../build/USS_Uncommon_Priors");

console.log("Hello Webpack World!");

wasm.then(wasm => {
    wasm.rah();
});
