## Premise

You are Cmdr. Robyn Hansen-Dale of the Federation science vessel USS _Uncommon Priors Require Origin Disputes_! The _Uncommon Priors_ has just been trapped in a pocket of twisted space with a toroidal topology. Do you have what it takes to survive—and escape?!

# Development Setup

 * Make sure you have Rust/Cargo, Node/NPM, and [wasm-pack](https://rustwasm.github.io/wasm-pack/).
 * Build the WASM binary with `wasm-pack build`.
 * Run the dev server with `npm run start` (in `www/`).

# Deploy

 * Pack up (into `dist/`) with `npm run build`.
 * Serve the contents of `dist/` from your server! Except, WebAssembly is so new that you may need to [hack the content-types registry](https://gist.github.com/WesThorburn/62ea13952749d6563ce2fb15b45f1ba8).
