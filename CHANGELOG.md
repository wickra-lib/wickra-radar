# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- The `radar-core` data-driven core: `RadarSpec` (JSON/TOML), the three input
  event kinds (derivatives, order-book, liquidation), the five cascade signals
  (open-interest delta, funding flip, book imbalance, liquidation cluster,
  OI/price divergence), the weighted-mean `severity` aggregation with a
  self-explaining `factors` map, `scan` over a perp universe, and the
  `Radar::command_json` JSON-over-C-ABI protocol. The parallel (rayon) and
  sequential builds are byte-for-byte identical.
- `wickra-radar` CLI: scan a universe from a spec + an event stream
  (`--stdin` / `--events`, `--threshold` / `--limit` overrides, `--format json`
  or a human-readable table).
- Ten-language surface: native Rust, Python (PyO3), Node.js (napi) and WASM
  (wasm-bindgen), plus a C ABI hub (cbindgen) backing C, C++, C#, Go, Java and R.
- Streaming: `feed` / `feed_batch` / `alerts` incremental scanning that returns
  the same bytes as a batch `scan`.
- A deterministic golden corpus (event universe, specs, byte-exact expected
  reports) and cross-language byte-equality tests across every binding.
- Test rigor: conformance, golden, streaming-equals-batch, property-based
  invariants, three cargo-fuzz targets, and the `radar-bench` criterion suite.
- One runnable "scan a universe" example per language and per-language guides
  under `docs/`.
- CI/CD: a multi-OS test matrix across ten languages, CodeQL, OpenSSF Scorecard,
  zizmor, link-check, benchmark and metadata-audit workflows, plus an authored
  (tag-gated) release workflow.
- Repository scaffolding: Cargo workspace, supply-chain configuration
  (`deny.toml`, `osv-scanner.toml`, `lychee.toml`), lint configuration
  (`clippy.toml`), `repo-metadata.toml`, and dual `MIT OR Apache-2.0` licensing.

[Unreleased]: https://github.com/wickra-lib/wickra-radar/commits/main
