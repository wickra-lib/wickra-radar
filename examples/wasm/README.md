# wickra-radar WASM examples

Browser demos for the `wickra-radar-wasm` binding.

The WASM build carries the whole radar core: the same signals, the same
severity arithmetic and the same report bytes the CLI and the other nine
bindings produce. A spec is data, not code, so the bytes on this page are the
same ones `examples/node/scan.js` sends, and the alerts are the same alerts.

## Build

The module ships as a `wasm-pack` `--target web` bundle. Build it once from the
repository root:

```bash
wasm-pack build bindings/wasm --target web --release
```

That writes `bindings/wasm/pkg/` with the `.wasm` binary, the JS loader and the
type declarations the page imports.

## Run

The page loads its module over `http://`, not `file://`, because ES module
imports and `WebAssembly.instantiateStreaming` both need a real origin. Serve the
repository root:

```bash
python -m http.server 8000
```

Then open `http://localhost:8000/examples/wasm/scan.html`.

## Pages

| Page | What it does |
|------|--------------|
| `scan.html` | Scans a two-symbol perp universe with a funding-flip and an open-interest signal, then feeds the same events one at a time and reads the alerts back, showing that both operating modes return the same report. The page counterpart of `examples/node/scan.js`. |

## See also

- [examples/README.md](../README.md) — the same scan in every other language.
- [bindings/wasm/README.md](../../bindings/wasm/README.md) — the binding itself.
