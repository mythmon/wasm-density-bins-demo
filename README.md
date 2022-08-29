# density-bins

Testing out speeding up JS plots with Rust.

To build a WASM artifact, use [`wasm-pack`](https://rustwasm.github.io/wasm-pack/). The generated `pkg/density_bins_bg.wasm` and `pkg/density_bins.js` what you need to use the code.

```shell
$ wasm-pack build --target web --release
```

See https://observablehq.com/@mythmon/density-plot-using-rust-wasm for detailed usage.
