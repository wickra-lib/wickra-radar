# Wickra Radar — WASM

WASM bindings for the `wickra-radar` data-driven core, compiled to WebAssembly
with wasm-bindgen. Build a `Radar` from a spec JSON, drive it with command JSON,
read back the report — the same protocol as every other binding, running in the
browser.

The core is built with `--no-default-features`, so the scan folds
**sequentially** (no rayon thread pool in the browser sandbox) and byte-identical
to the native parallel build.

## Build

```bash
wasm-pack build --target web
```

This emits `pkg/` with the `.wasm` module and JS glue.

## Usage

```js
import init, { Radar, version } from "./pkg/wickra_radar_wasm.js";

await init();

const spec = JSON.stringify({
  symbols: ["AAA"],
  signals: [{ kind: "funding_flip", params: [0.0005] }],
  threshold: 0.0,
});

const radar = new Radar(spec);
const events = {
  AAA: [
    { kind: "derivatives", ts: 1, open_interest: 1.0, funding_rate: 0.0003, mark_price: 50.0 },
    { kind: "derivatives", ts: 2, open_interest: 1.0, funding_rate: -0.0004, mark_price: 50.0 },
  ],
};
const report = JSON.parse(radar.command(JSON.stringify({ cmd: "scan", events })));

console.log(report.scanned, report.alerts[0].symbol);
console.log(version());
```

## API

| Member | Description |
|--------|-------------|
| `new Radar(specJson)` | Build a radar from a spec JSON (throws on an invalid spec). |
| `radar.command(cmdJson)` | Apply a command JSON, return the response JSON. |
| `radar.version()` / `version()` | The library version. |

## License

`MIT OR Apache-2.0`.
