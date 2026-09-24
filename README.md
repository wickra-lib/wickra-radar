<p align="center">
  <a href="https://wickra.org"><img src="https://raw.githubusercontent.com/wickra-lib/.github/main/profile/wickra-banner.webp?v=514-7" alt="Wickra Radar — a liquidation-cascade early-warning radar over 514 streaming indicators" width="100%"></a>
</p>

[![Built on Wickra](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-radar/built-on.svg)](https://github.com/wickra-lib/wickra)
[![Status](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-radar/status.svg)](https://github.com/wickra-lib/wickra-radar)
[![CI](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-radar/ci.svg)](https://github.com/wickra-lib/wickra-radar/actions/workflows/ci.yml)
[![CodeQL](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-radar/codeql.svg)](https://github.com/wickra-lib/wickra-radar/actions/workflows/codeql.yml)
[![codecov](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-radar/codecov.svg)](https://codecov.io/gh/wickra-lib/wickra-radar)
[![GitHub release](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-radar/release.svg)](https://github.com/wickra-lib/wickra-radar/releases/latest)
[![crates.io](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-radar/crates.svg)](https://crates.io/crates/wickra-radar)
[![PyPI](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-radar/pypi.svg)](https://pypi.org/project/wickra-radar/)
[![npm](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-radar/npm.svg)](https://www.npmjs.com/package/wickra-radar)
[![NuGet](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-radar/nuget.svg)](https://www.nuget.org/packages/Wickra.Radar)
[![Maven Central](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-radar/maven.svg)](https://central.sonatype.com/artifact/org.wickra/wickra-radar)
[![Go module](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-radar/go.svg)](https://pkg.go.dev/github.com/wickra-lib/wickra-radar-go)
[![R-universe](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-radar/r-universe.svg)](https://wickra-lib.r-universe.dev)
[![License: MIT OR Apache-2.0](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-radar/license.svg)](#license)
[![OpenSSF Scorecard](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-radar/scorecard.svg)](https://scorecard.dev/viewer/?uri=github.com/wickra-lib/wickra-radar)
[![OpenSSF Best Practices](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-radar/best-practices.svg)](https://www.bestpractices.dev)
[![Build provenance](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-radar/provenance.svg)](https://github.com/wickra-lib/wickra-radar/attestations)
[![Docs](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-radar/docs.svg)](https://radar.wickra.org)
[![Verified across 10 languages](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-radar/verified.svg)](golden/)
[![Live demo](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-radar/live-demo.svg)](https://live.wickra.org)

---

**See liquidation cascades before they happen — open-interest, funding, order-book and liquidation signals scored across every perp in parallel.**

> **▶ Live demos:** the backtester compiled to WebAssembly, an equity curve building bar by bar — **[backtest-live.wickra.org](https://backtest-live.wickra.org)**;
> one StrategySpec side by side in Python, Rust, JS and Go — **[playground.wickra.org](https://playground.wickra.org)**;
> all 514 indicators of the core over a real Binance feed — **[live.wickra.org](https://live.wickra.org)**. Zero backend, all of them.

**Part of the [Wickra ecosystem](#ecosystem):** the same data-driven core and ten-language binding surface also power [wickra-exchange](https://github.com/wickra-lib/wickra-exchange), [wickra-backtest](https://github.com/wickra-lib/wickra-backtest), [wickra-terminal](https://github.com/wickra-lib/wickra-terminal) and 20 more — see [the full list](https://github.com/wickra-lib).

Wickra Radar is one data-driven core, [`wickra-radar-core`](crates/radar-core): a serde
`RadarSpec` is folded over a perp universe — open interest, funding, order-book
and liquidation events — into a `RadarReport` of severity-scored `RadarAlert`s.
Each symbol runs a handful of O(1) streaming signals; the per-signal scores are
aggregated with weights into a single severity in `[0, 1]`. Thousands of symbols
update in parallel, turning the whole market into a **crash early-warning
seismograph** that price-only tools never see.

Because the alert is **data, not code**, the exact same output crosses the C ABI
and WASM unchanged — and stays byte-for-byte identical between the parallel
(rayon) and sequential (the WASM fallback) builds. The core is exposed as a
**JSON-over-C-ABI data API** (`Radar::command`) in **Rust, Python, Node.js, WASM,
C, C++, C#, Go, Java and R**, with a command-line reference consumer.

- **OI delta** — a burst in open interest over a rolling window.
- **Funding flip** — funding rate crossing zero (longs ↔ shorts pay).
- **Book imbalance** — resting bid/ask liquidity skew.
- **Liquidation cluster** — liquidation events bunching in a short window.
- **OI / price divergence** — open interest rising while price stalls or falls.

```bash
# Scan a perp universe from a spec + an event batch, raw RadarReport JSON
# (the same bytes every binding returns):
cargo run -p wickra-radar -- --spec golden/specs/composite.json --stdin --format json < golden/events.json

# Human-readable table of alerts:
cargo run -p wickra-radar -- --spec golden/specs/composite.json --stdin < golden/events.json
```

## Status

**0.1.4 — the current release.** The core, the CLI, all ten language bindings,
the byte-exact golden corpus, property + fuzz tests, benchmarks and one runnable
example per language are in place and green across the full CI matrix (10
languages × 3 OS). Track progress in [ROADMAP.md](ROADMAP.md).

## Documentation

- [Architecture](ARCHITECTURE.md) — the core, the data-driven boundary, the binding surface.
- Signal & spec reference and per-binding quickstarts under [`docs/`](docs); one runnable example per language under [`examples/`](examples).
- [ROADMAP.md](ROADMAP.md) · [BENCHMARKS.md](BENCHMARKS.md) · [THREAT_MODEL.md](THREAT_MODEL.md) · [SECURITY.md](SECURITY.md).

## Quickstart

The `--spec` file is a `RadarSpec`; events are read either from `--stdin` (one
JSON object `{"SYMBOL":[event, …], …}`) or from `--events <dir>`, a directory of
per-symbol `<SYMBOL>.jsonl` files (one JSON `Event` per line). `--limit` and
`--threshold` override the spec.

## RadarSpec / signals

A spec is a JSON (or TOML) document: a list of `signals`, an optional severity
`threshold`, and an optional top-N `limit`. Each signal names a `kind`, its
numeric `params`, and an optional `weight` (default `1.0`). The report scores
every symbol, keeps those at or above `threshold`, and returns the top `limit`
sorted by severity (descending), then symbol (ascending).

```json
{
  "signals": [
    { "kind": "oi_delta", "params": [2.0, 0.1], "weight": 1.0 },
    { "kind": "funding_flip", "params": [0.0005], "weight": 2.0 },
    { "kind": "book_imbalance", "params": [1.0], "weight": 1.0 },
    { "kind": "liq_cluster", "params": [5.0, 30.0], "weight": 1.5 },
    { "kind": "oi_price_divergence", "params": [2.0, 0.1], "weight": 3.0 }
  ],
  "threshold": 0.2,
  "limit": 3
}
```

- **Signals** (`kind`): `oi_delta`, `funding_flip`, `book_imbalance`, `liq_cluster`, `oi_price_divergence`.
- **Alert** — `RadarAlert { symbol, severity, factors, ts }`; `factors` is the
  per-signal score map plus the aggregated `severity`, so every alert explains
  itself. The report is `RadarReport { alerts, scanned }`.

## Streaming, and why it is deterministic

`scan` folds a whole batch at once; `feed` / `feed_batch` drive the same per-symbol
state incrementally and `alerts` reads the report at any point — the streaming
path and the batch path go through one shared `report_from_states`, so they
return **byte-identical** JSON. The parallel (rayon) and sequential builds agree
bit-for-bit too: alerts sort by a total order (`f64::total_cmp` on severity, then
symbol), never a partial float compare.

## Use in any language

The same `Radar` handle — construct from a JSON spec, drive with
`command(json) -> json`, read `version` — is reachable from every binding:

```python
from wickra_radar import Radar
r = Radar('{"signals":[{"kind":"funding_flip","params":[0.0005]}],"threshold":0.0}')
report = r.command('{"cmd":"scan","events":{"AAA":['
                   '{"kind":"derivatives","ts":1,"open_interest":1.0,"funding_rate":0.0003,"mark_price":50.0},'
                   '{"kind":"derivatives","ts":2,"open_interest":1.0,"funding_rate":-0.0004,"mark_price":50.0}]}}')
# report is a JSON RadarReport: {"alerts":[{"symbol":"AAA","severity":1.0,...}],"scanned":1}
```

The C ABI hub (`bindings/c`) backs C, C++, C#, Go, Java and R; Rust, Python,
Node.js and WASM are native. See each `bindings/<lang>/README.md` and the runnable
[`examples/`](examples).

## Project layout

```
crates/radar-core     the data-driven core (RadarSpec, Universe, signals, aggregate, scan, command_json)
crates/radar-cli      the CLI (bin: wickra-radar)
crates/radar-bench    criterion benchmarks
bindings/{python,node,wasm,c,go,csharp,java,r}   the ten-language surface
golden/               a deterministic event universe, specs, and byte-exact expected reports
fuzz/                 cargo-fuzz targets (spec_parse, command_json, scan)
examples/             one runnable "scan a universe" example per language
```

## Building everything from source

```bash
cargo build --workspace
cargo test  --workspace --all-features
cargo test  --workspace --no-default-features   # sequential build path
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo run -p wickra-radar -- --spec golden/specs/composite.json --stdin --format json < golden/events.json
```

Each binding builds from its own directory — see the per-binding READMEs under
`bindings/`.

## Testing

Run the suites with the commands in
[Building everything from source](#building-everything-from-source).

- **`wickra-radar-core`** — unit tests per signal, the scoring and aggregation
  path, the parallel-versus-sequential parity, property tests over the event
  stream and the command envelope. The golden fixtures in `golden/` are the
  anchor: the same `(spec, events)` pair must scan to the same report bytes
  here as in every binding.
- **Every binding** asserts the *same* golden bytes. That is the whole
  cross-language claim, so it is checked the same way in each one rather than
  approximated per language: Python with pytest (and a plain runner on 3.9),
  Node with `node --test`, WASM through the nodejs build, C and C++ through
  `ctest`, C# with `dotnet test`, Go with `go test`, Java with JUnit, and R
  with the shipped `tests/smoke.R` plus the repository's `run_tests.R`.
- **Examples** — every example under `examples/` runs in CI and is held to the
  version and the alerts it prints.
- **Fuzz** — `fuzz/` holds libFuzzer targets over spec parsing, the command
  envelope and the scan; CI runs each for a short smoke.

## Requirements

- **Rust 1.86+** — the workspace MSRV; the Node binding needs **Rust 1.88**.
- **Python 3.9+** — the Python binding.
- **Node 22+** — the Node binding.
- **Go 1.23+** — the Go binding.
- **Java 22+** — the Java binding.
- **R 4.1+** — the R package.
- **.NET 8+** — the C# binding.
- A **C11 / C++17** compiler with CMake 3.15+ for the C and C++ examples.

See each `bindings/<lang>/README.md` for the per-language build and install.

## Benchmarks

`crates/radar-bench` measures `scan` scaling by universe size and events per
symbol, parallel vs sequential. See [BENCHMARKS.md](BENCHMARKS.md).

## Ecosystem

Part of the [Wickra](https://github.com/wickra-lib/wickra) family — each one a
data-driven core with a CLI and the same ten-language binding surface:

- [**wickra**](https://github.com/wickra-lib/wickra) — main library (Rust core + Python / Node.js / WASM bindings + a C ABI for C / C++ / C# / Go / Java / R)
- [**wickra-playground**](https://github.com/wickra-lib/wickra-playground) — a polyglot strategy playground: one StrategySpec live side by side in Python, Rust, JS and Go, entirely in the browser
- [**wickra-exchange**](https://github.com/wickra-lib/wickra-exchange) — unified market-data + execution across ten crypto exchanges
- [**wickra-backtest**](https://github.com/wickra-lib/wickra-backtest) — event-driven backtester over the Wickra core
- [**wickra-terminal**](https://github.com/wickra-lib/wickra-terminal) — the trading terminal: a TUI and a browser renderer over the stack
- [**wickra-screener**](https://github.com/wickra-lib/wickra-screener) — parallel multi-symbol screening over 514 streaming indicators
- [**wickra-xray**](https://github.com/wickra-lib/wickra-xray) — market-microstructure explorer: footprint, order-book heatmap, liquidation map, funding/OI divergence
- [**wickra-copilot**](https://github.com/wickra-lib/wickra-copilot) — local market copilot grounded in real order-book, liquidation and funding microstructure
- [**wickra-shazam**](https://github.com/wickra-lib/wickra-shazam) — match an asset's current microstructure fingerprint against its entire history
- [**wickra-benchmark**](https://github.com/wickra-lib/wickra-benchmark) — reproducible, golden-verified benchmark suite — recompute any (strategy, dataset, report) in ten languages and confirm it byte-for-byte
- [**wickra-strategy-ci**](https://github.com/wickra-lib/wickra-strategy-ci) — Jest for trading strategies: golden-pin the report, catch regressions in CI, property-test against fuzzed data
- [**wickra-verify**](https://github.com/wickra-lib/wickra-verify) — confirm or refute a claimed backtest report against its strategy and data, in ten languages
- [**wickra-proof**](https://github.com/wickra-lib/wickra-proof) — Proof-of-Backtest: deterministic (spec, data) → report + blake3 hash, recomputable byte-for-byte in ten languages
- [**wickra-zk**](https://github.com/wickra-lib/wickra-zk) — prove a backtest zero-knowledge — on-chain-verifiable performance without revealing the data or the strategy
- [**wickra-impact**](https://github.com/wickra-lib/wickra-impact) — the backtester that knows you would have moved the market: agent-based fills on the real historical L2 order book
- [**wickra-darwin**](https://github.com/wickra-lib/wickra-darwin) — evolutionary strategy search at millions of backtests per second, mutating and crossing JSON specs across the 514-indicator space
- [**wickra-gym**](https://github.com/wickra-lib/wickra-gym) — a Gymnasium-compatible, microstructure-aware backtest environment with O(1) steps for deterministic RL rollouts
- [**wickra-feature-store**](https://github.com/wickra-lib/wickra-feature-store) — OHLCV and microstructure streams into ML-ready feature matrices over 514 O(1) streaming indicators
- [**wickra-genome**](https://github.com/wickra-lib/wickra-genome) — a vector database of the whole market: every asset a 514-dim live vector, for similarity search, clustering and anomaly detection
- [**wickra-timemachine**](https://github.com/wickra-lib/wickra-timemachine) — scrub the whole market like a video — every symbol, full order book, rewound to any moment via deterministic re-fold
- [**wickra-synth**](https://github.com/wickra-lib/wickra-synth) — deterministic synthetic market microstructure: OHLCV, order book, trades and funding from a single seed
- [**wickra-compile**](https://github.com/wickra-lib/wickra-compile) — compile a strategy spec into a standalone deployable: a WASM module, a self-contained binary, or a `no_std` artifact
- [**wickra-embed**](https://github.com/wickra-lib/wickra-embed) — allocation-free, `no_std` streaming indicators for bare-metal and HFT, byte-for-byte identical to the core
- [**wickra-pico**](https://github.com/wickra-lib/wickra-pico) — the O(1) indicator core running bare-metal on a $5 Raspberry Pi Pico — the LED blinks on the EMA cross

Docs at [docs.wickra.org](https://docs.wickra.org); the marketing site and
in-browser demo at [wickra.org](https://wickra.org).

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) and [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md).
Commits are signed and in English; open a PR against `main`.

## Security

See [SECURITY.md](SECURITY.md) and [THREAT_MODEL.md](THREAT_MODEL.md). Report
vulnerabilities privately — never in a public issue.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option. Use it, fork it, modify it, redistribute it — commercially or
not — file issues, send pull requests; all welcome.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

## Disclaimer

Wickra Radar is analysis software: it computes early-warning signals over
historical and live market data. It is provided "as is", without warranty of any
kind, and is **not financial advice** — it places no orders. Trading carries risk
of loss; review the code and use at your own discretion.

---

<p align="center">
  <a href="https://github.com/wickra-lib/wickra-radar">
    <img alt="GitHub stars" src="https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-radar/stars.svg">
  </a>
  <a href="https://github.com/wickra-lib/wickra-radar/network/members">
    <img alt="GitHub forks" src="https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-radar/forks.svg">
  </a>
  <a href="https://github.com/wickra-lib/wickra-radar/issues">
    <img alt="GitHub issues" src="https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-radar/issues.svg">
  </a>
</p>

<p align="center">
  Built on <a href="https://github.com/wickra-lib/wickra">Wickra</a>. If it saved you time, the cheapest way to say thanks is to ⭐ the repo.
</p>

<p align="center">
  <img alt="wickra-radar star history" width="640"
       src="https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-radar/star-history.svg">
</p>
