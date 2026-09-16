<p align="center">
  <a href="https://wickra.org"><img src="https://raw.githubusercontent.com/wickra-lib/.github/main/profile/wickra-banner.webp?v=514" alt="Wickra Radar — a liquidation-cascade early-warning radar over 514 streaming indicators" width="100%"></a>
</p>

[![CI](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-radar/ci.svg)](https://github.com/wickra-lib/wickra-radar/actions/workflows/ci.yml)
[![codecov](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-radar/codecov.svg)](https://codecov.io/gh/wickra-lib/wickra-radar)
[![npm](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-radar/npm.svg)](https://www.npmjs.com/package/wickra-radar)
[![License: MIT OR Apache-2.0](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-radar/license.svg)](https://github.com/wickra-lib/wickra-radar#license)

# Wickra Radar — Node.js

---

> **▶ Live demo:** all 514 indicators over real Binance market data, computed live in your browser — **[live.wickra.org](https://live.wickra.org)** · zero backend, powered by `wickra-wasm`.

**See liquidation cascades before they happen — for Node.js. `npm install wickra-radar` — prebuilt native binary, no system dependencies.**

Node.js bindings for
[`wickra-radar-core`](https://github.com/wickra-lib/wickra-radar), built with
[napi-rs]. The surface mirrors every other Wickra binding: build a `Radar` from
a spec JSON, drive it with command JSONs, and read back the report.

## Install

```bash
npm install wickra-radar
```

The native addon ships as a prebuilt binary per platform (Linux, macOS,
Windows — x64 and arm64), selected automatically through optional
dependencies. There is nothing to compile.

The right prebuilt native binary is pulled in automatically as an optional
dependency for your platform.

### Building from this repository (contributors)

```sh
npm install
npm run build
npm test
```

## Quick start

```js
const { Radar } = require("wickra-radar");

const spec = JSON.stringify({
  symbols: ["AAA"],
  signals: [{ kind: "funding_flip", params: [0.0005] }],
  threshold: 0.0,
});

const radar = new Radar(spec);
const report = JSON.parse(radar.command(JSON.stringify({ cmd: "scan", events: {
  AAA: [
    { kind: "derivatives", ts: 1, open_interest: 1.0, funding_rate: 0.0003, mark_price: 50.0 },
    { kind: "derivatives", ts: 2, open_interest: 1.0, funding_rate: -0.0004, mark_price: 50.0 },
  ],
}})));
console.log(report.scanned, report.alerts[0].symbol); // 1 AAA
```

### Surface

- **`new Radar(spec_json)`** builds a radar from a spec JSON (`""` or `"{}"` for
  an empty handle whose spec is set later). Throws on a malformed spec.
- **`radar.command(cmd_json)`** applies a command JSON (`set_spec`, `feed`,
  `feed_batch`, `scan`, `alerts`, `reset`, `version`) and returns the response
  JSON — a `RadarReport` for `scan`/`alerts`. Domain errors come back in-band as
  `{"ok":false,"error":...}`.
- **`radar.version()` / `version()`** return the library version.

## Benchmark

Every binding forwards to the same data-driven Rust core, so what this one adds is
the call overhead of napi-rs, not a different result. The core's throughput is
measured by the repository's benchmark suite and the nightly `bench.yml` run; the
numbers, the machine and how to reproduce them are in the repository
[BENCHMARKS.md](https://github.com/wickra-lib/wickra-radar/blob/main/BENCHMARKS.md).

## Documentation

The full guide, the spec reference and the API documentation live in the main
repository and the documentation site:

- **Repository:** <https://github.com/wickra-lib/wickra-radar>
- **Docs** (guides, spec reference, cookbook): <https://radar.wickra.org>
- **Runnable example:** [`examples/node/`](https://github.com/wickra-lib/wickra-radar/tree/main/examples/node)

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

[napi-rs]: https://napi.rs
