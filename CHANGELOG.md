# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Fixed

- **The core crate carried a name the release could not upload.**
  `radar-core` is outside the org's crates.io token scope, which
  creates new crates under the `wickra-` prefix only; `cargo publish` on it
  returns 403 at upload while `--dry-run` passes, and because the publish jobs
  run in parallel the release would have landed on PyPI, npm, NuGet, Maven
  Central and the Go mirror without ever reaching crates.io. The core is now
  `wickra-radar-core`, the shape of every released sibling. The
  directory keeps its name; only the package and the
  `wickra_radar_core` path moved. The same audit ran across the family
  (xray paid for this with its first tag).

### Added

- The `wickra-radar-core` data-driven core: `RadarSpec` (JSON/TOML), the three input
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
