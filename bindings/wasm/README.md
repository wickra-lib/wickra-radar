<p align="center">
  <a href="https://wickra.org"><img src="https://raw.githubusercontent.com/wickra-lib/.github/main/profile/wickra-banner.webp?v=514" alt="Wickra Radar — a liquidation-cascade early-warning radar over 514 streaming indicators" width="100%"></a>
</p>

[![CI](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-radar/ci.svg)](https://github.com/wickra-lib/wickra-radar/actions/workflows/ci.yml)
[![codecov](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-radar/codecov.svg)](https://codecov.io/gh/wickra-lib/wickra-radar)
[![npm](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-radar/npm.svg)](https://www.npmjs.com/package/wickra-radar-wasm)
[![License: MIT OR Apache-2.0](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-radar/license.svg)](https://github.com/wickra-lib/wickra-radar#license)

# Wickra Radar — WASM

---

> **▶ Live demo:** all 514 indicators over real Binance market data, computed live in your browser — **[live.wickra.org](https://live.wickra.org)** · zero backend, powered by `wickra-wasm`.

**See liquidation cascades before they happen — for WASM. `npm install wickra-radar-wasm` — pure WebAssembly, runs anywhere a modern JS engine does.**

WASM bindings for the `wickra-radar` data-driven core, compiled to WebAssembly
with wasm-bindgen. Build a `Radar` from a spec JSON, drive it with command JSON,
read back the report — the same protocol as every other binding, running in the
browser.

The core is built with `--no-default-features`, so the scan folds
**sequentially** (no rayon thread pool in the browser sandbox) and byte-identical
to the native parallel build.

## Install

```bash
npm install wickra-radar-wasm
```

### Building from this repository (contributors)

```bash
wasm-pack build --target web
```

This emits `pkg/` with the `.wasm` module and JS glue.

## Quick start

```js
import init, { Radar, version } from "wickra-radar-wasm";

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

### API

| Member | Description |
|--------|-------------|
| `new Radar(specJson)` | Build a radar from a spec JSON (throws on an invalid spec). |
| `radar.command(cmdJson)` | Apply a command JSON, return the response JSON. |
| `radar.version()` / `version()` | The library version. |

## Benchmark

Every binding forwards to the same data-driven Rust core, so what this one adds is
the call overhead of wasm-bindgen, not a different result. The core's throughput is
measured by the repository's benchmark suite and the nightly `bench.yml` run; the
numbers, the machine and how to reproduce them are in the repository
[BENCHMARKS.md](https://github.com/wickra-lib/wickra-radar/blob/main/BENCHMARKS.md).

## Documentation

The full guide, the spec reference and the API documentation live in the main
repository and the documentation site:

- **Repository:** <https://github.com/wickra-lib/wickra-radar>
- **Docs** (guides, spec reference, cookbook): <https://radar.wickra.org>
- **Runnable example:** [`examples/wasm/`](https://github.com/wickra-lib/wickra-radar/tree/main/examples/wasm)

Wickra Radar ships native bindings for Python, Node.js, WASM and Rust, plus a C ABI hub that any
C-capable language (C, C++, C#, Go, Java, R) links against — all forwarding to the
same data-driven, `unsafe`-forbidden Rust core.

## Security

Found a security issue? **Please don't open a public issue.** Report it privately
via the repository's *Security* tab (*"Report a vulnerability"*) or email
**support@wickra.org** with a subject line starting `[wickra security]`. Full
policy: <https://github.com/wickra-lib/wickra-radar/blob/main/SECURITY.md>.

## Disclaimer

Wickra Radar is analysis software: it computes early-warning signals over
historical and live market data. It is provided "as is", without warranty of any
kind, and is **not financial advice** — it places no orders. Trading carries risk
of loss; review the code and use at your own discretion.

## License

Licensed under either of [Apache-2.0](https://github.com/wickra-lib/wickra-radar/blob/main/LICENSE-APACHE)
or [MIT](https://github.com/wickra-lib/wickra-radar/blob/main/LICENSE-MIT) at your option.
