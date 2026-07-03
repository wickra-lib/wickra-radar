<p align="center">
  <a href="https://wickra.org"><img src="https://raw.githubusercontent.com/wickra-lib/.github/main/profile/wickra-banner.webp?v=514" alt="Wickra Radar — a liquidation-cascade early-warning radar over 514 streaming indicators" width="100%"></a>
</p>

[![Built on Wickra](https://img.shields.io/badge/built%20on-wickra-3b82f6)](https://github.com/wickra-lib/wickra)
[![Status](https://img.shields.io/badge/status-pre--release-orange)](https://github.com/wickra-lib/wickra-radar)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue)](#license)

<!-- Skeleton README (P-RAD-0.12). The full ~20-badge block (CI, CodeQL, codecov,
     crates.io/PyPI/npm/NuGet/Maven/Go/R-universe, Scorecard, Best-Practices,
     Provenance, Docs, Verified) and the finished sections are assembled in
     P-RAD-8.1, once the per-product badge SVGs are generated in the .github repo
     (P-RAD-8.2). Until then this stays link-clean (no 404s on the repo page). -->

---

# Wickra Radar

**See liquidation cascades before they happen — OI, funding, order-book and liquidation signals across every perp in parallel over 514 O(1) streaming indicators.**

Wickra Radar is one data-driven core, `radar-core`: a serde `RadarSpec` is folded
over a perp universe — open interest, funding, order-book and liquidation events —
with 514 O(1) streaming indicators, scored per signal and aggregated with weights
into a `RadarAlert`. Thousands of symbols update in parallel, turning the whole
market into a **crash early-warning seismograph** that price-only tools never see.

Because the alert is **data, not code**, the exact same output crosses the C ABI
and WASM unchanged — and stays byte-for-byte identical between the parallel
(rayon) and sequential builds. The core is exposed as a **JSON-over-C-ABI data
API** (`Radar::command`) in **Rust, Python, Node.js, WASM, C, C++, C#, Go, Java
and R**, with a command-line reference consumer.

## Status

**Pre-release — under active construction.** This repository is being built out
phase by phase (scaffold → core → CLI → ten language bindings → golden corpus →
property/fuzz tests → CI → docs). It is not yet published to any registry.

## Documentation

The full documentation — the `RadarSpec` / signal reference, the alert
data-model, and per-binding quickstarts — is finalized in this README and under
`docs/` during the documentation phase.

## License

Dual-licensed under [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE), at your option.

## Disclaimer

Wickra Radar is analysis software: it computes early-warning signals over
historical and live market data. It does not provide financial advice and places
no orders. Trading carries risk; use at your own discretion.
