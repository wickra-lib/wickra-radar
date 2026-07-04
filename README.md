<p align="center">
  <a href="https://wickra.org"><img src="https://raw.githubusercontent.com/wickra-lib/.github/main/profile/wickra-banner.webp?v=514" alt="Wickra Radar — a liquidation-cascade early-warning radar over 514 streaming indicators" width="100%"></a>
</p>

[![Built on Wickra](https://img.shields.io/badge/built%20on-wickra-3b82f6)](https://github.com/wickra-lib/wickra)
[![Status](https://img.shields.io/badge/status-pre--release-orange)](https://github.com/wickra-lib/wickra-radar)
[![CI](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-radar/ci.svg)](https://github.com/wickra-lib/wickra-radar/actions/workflows/ci.yml)
[![CodeQL](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-radar/codeql.svg)](https://github.com/wickra-lib/wickra-radar/actions/workflows/codeql.yml)
[![codecov](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-radar/codecov.svg)](https://codecov.io/gh/wickra-lib/wickra-radar)
[![License: MIT OR Apache-2.0](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-radar/license.svg)](#license)
[![OpenSSF Scorecard](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-radar/scorecard.svg)](https://scorecard.dev/viewer/?uri=github.com/wickra-lib/wickra-radar)
[![OpenSSF Best Practices](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-radar/best-practices.svg)](https://www.bestpractices.dev/)
[![Build provenance](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-radar/provenance.svg)](https://github.com/wickra-lib/wickra-radar/attestations)
[![Docs](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-radar/docs.svg)](https://wickra.org)
[![Live demo](https://img.shields.io/badge/live%20demo-live.wickra.org-3b82f6)](https://live.wickra.org)

---

# Wickra Radar

**See liquidation cascades before they happen — open-interest, funding, order-book and liquidation signals scored across every perp in parallel.**

> **▶ Live demo:** all 514 indicators over real Binance market data, computed live in your browser — **[live.wickra.org](https://live.wickra.org)** · zero backend, powered by `wickra-wasm`.

> **Part of the [Wickra ecosystem](https://github.com/wickra-lib):** the same data-driven core and ten-language binding surface also power [wickra-exchange](https://github.com/wickra-lib/wickra-exchange), [wickra-backtest](https://github.com/wickra-lib/wickra-backtest), [wickra-terminal](https://github.com/wickra-lib/wickra-terminal), [wickra-screener](https://github.com/wickra-lib/wickra-screener), [wickra-xray](https://github.com/wickra-lib/wickra-xray), [wickra-radar](https://github.com/wickra-lib/wickra-radar), [wickra-copilot](https://github.com/wickra-lib/wickra-copilot) and [wickra-shazam](https://github.com/wickra-lib/wickra-shazam).

Wickra Radar is one data-driven core, [`radar-core`](crates/radar-core): a serde
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

## Status

**Pre-release — functionally complete, CI-verified, not yet published.** The core,
the CLI, all ten language bindings, the byte-exact golden corpus, property + fuzz
tests, benchmarks and one runnable example per language are in place and green
across the full CI matrix (10 languages × 3 OS). Not yet released to any registry
— track progress in [ROADMAP.md](ROADMAP.md).

## Documentation

- [Architecture](ARCHITECTURE.md) — the core, the data-driven boundary, the binding surface.
- Signal & spec reference and per-binding quickstarts under [`docs/`](docs); one runnable example per language under [`examples/`](examples).
- [ROADMAP.md](ROADMAP.md) · [BENCHMARKS.md](BENCHMARKS.md) · [THREAT_MODEL.md](THREAT_MODEL.md) · [SECURITY.md](SECURITY.md).

## Quickstart

```bash
# Scan a perp universe from a spec + an event batch, raw RadarReport JSON
# (the same bytes every binding returns):
cargo run -p wickra-radar -- --spec golden/specs/composite.json --stdin --format json < golden/events.json

# Human-readable table of alerts:
cargo run -p wickra-radar -- --spec golden/specs/composite.json --stdin < golden/events.json
```

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

## Building from source

```bash
cargo build --workspace
cargo test  --workspace --all-features
cargo test  --workspace --no-default-features   # sequential build path
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo run -p wickra-radar -- --spec golden/specs/composite.json --stdin --format json < golden/events.json
```

## Requirements

- **Rust** ≥ 1.86 (workspace MSRV; the Node binding needs ≥ 1.88).
- Binding toolchains as needed: Node ≥ 22, Python ≥ 3.9, a C toolchain, .NET 8,
  JDK 22+, Go 1.23, R — see each `bindings/<lang>/README.md`.

## Benchmarks

`crates/radar-bench` measures `scan` scaling by universe size and events per
symbol, parallel vs sequential. See [BENCHMARKS.md](BENCHMARKS.md).

## Ecosystem

Part of the [Wickra](https://github.com/wickra-lib/wickra) family — each one a
data-driven core with a CLI and the same ten-language binding surface:

- [**wickra**](https://github.com/wickra-lib/wickra) — the core library: 514 O(1) streaming indicators across ten languages
- [**wickra-exchange**](https://github.com/wickra-lib/wickra-exchange) — unified market-data + execution across ten crypto exchanges
- [**wickra-backtest**](https://github.com/wickra-lib/wickra-backtest) — event-driven backtester over the Wickra core
- [**wickra-terminal**](https://github.com/wickra-lib/wickra-terminal) — the trading terminal: a TUI and a browser renderer over the stack
- [**wickra-screener**](https://github.com/wickra-lib/wickra-screener) — parallel multi-symbol screening over 514 streaming indicators
- [**wickra-xray**](https://github.com/wickra-lib/wickra-xray) — market-microstructure explorer: footprint, order-book heatmap, liquidation map, funding/OI divergence
- [**wickra-radar**](https://github.com/wickra-lib/wickra-radar) — perp-universe alert radar: OI delta, funding flip, book imbalance, liquidation clusters, OI/price divergence
- [**wickra-copilot**](https://github.com/wickra-lib/wickra-copilot) — local market copilot grounded in real order-book, liquidation and funding microstructure
- [**wickra-shazam**](https://github.com/wickra-lib/wickra-shazam) — match an asset's current microstructure fingerprint against its entire history

Docs at [docs.wickra.org](https://docs.wickra.org); the marketing site and
in-browser demo at [wickra.org](https://wickra.org).

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) and [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md).
Commits are signed and in English; open a PR against `main`.

## Security

See [SECURITY.md](SECURITY.md) and [THREAT_MODEL.md](THREAT_MODEL.md). Report
vulnerabilities privately — never in a public issue.

## License

Dual-licensed under either [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE), at
your option.

## Disclaimer

Wickra Radar is analysis software: it computes early-warning signals over
historical and live market data. It is provided "as is", without warranty of any
kind, and is **not financial advice** — it places no orders. Trading carries risk
of loss; review the code and use at your own discretion.
