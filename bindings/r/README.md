<p align="center">
  <a href="https://wickra.org"><img src="https://raw.githubusercontent.com/wickra-lib/.github/main/profile/wickra-banner.webp?v=514" alt="Wickra Radar — a liquidation-cascade early-warning radar over 514 streaming indicators" width="100%"></a>
</p>

[![CI](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-radar/ci.svg)](https://github.com/wickra-lib/wickra-radar/actions/workflows/ci.yml)
[![codecov](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-radar/codecov.svg)](https://codecov.io/gh/wickra-lib/wickra-radar)
[![r-universe](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-radar/r-universe.svg)](https://wickra-lib.r-universe.dev)
[![License: MIT OR Apache-2.0](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-radar/license.svg)](https://github.com/wickra-lib/wickra-radar#license)

# Wickra Radar — R

---

> **▶ Live demo:** all 514 indicators over real Binance market data, computed live in your browser — **[live.wickra.org](https://live.wickra.org)** · zero backend, powered by `wickra-wasm`.

**See liquidation cascades before they happen — for R. `install.packages("wickraradar", repos = "https://wickra-lib.r-universe.dev")` — over the C ABI via `.Call`, prebuilt library fetched on install.**

R bindings for the `wickra-radar` data-driven core, over its C ABI hub (`.Call`).
Build a radar from a spec JSON, drive it with command JSON, read back the report
— the same protocol as the CLI and every other binding.

## Install

From r-universe:

```r
install.packages("wickraradar", repos = "https://wickra-lib.r-universe.dev")
```

The package's `configure` downloads the prebuilt C ABI library for this exact
version from the GitHub release and bundles it, so an ordinary install needs
nothing but a C toolchain (Rtools on Windows) for the thin `.Call` glue layer. To
build against a local checkout instead, point it at the header and library with
the environment variables below.

### Building from this repository (contributors)

The package links the `wickra_radar` C ABI, located out-of-tree via two
environment variables:

```bash
# Build the C ABI shared library first.
cargo build -p wickra-radar-c --release

export WKRADAR_INC="$PWD/bindings/c/include"
export WKRADAR_LIB="$PWD/target/release"
# The loader must also find the shared library at run time:
export LD_LIBRARY_PATH="$WKRADAR_LIB:$LD_LIBRARY_PATH"   # PATH on Windows

R CMD INSTALL bindings/r
Rscript bindings/r/tests/run_tests.R
```

## Quick start

```r
library(wickraradar)

spec <- paste0(
  '{"symbols":["AAA"],"signals":[{"kind":"funding_flip",',
  '"params":[0.0005]}],"threshold":0.0}'
)

radar <- wkradar_new(spec)
report <- wkradar_command(radar, paste0(
  '{"cmd":"scan","events":{"AAA":[',
  '{"kind":"derivatives","ts":1,"open_interest":1.0,"funding_rate":0.0003,"mark_price":50.0},',
  '{"kind":"derivatives","ts":2,"open_interest":1.0,"funding_rate":-0.0004,"mark_price":50.0}]}}'
))
cat(report, "\n")
cat(wkradar_version(), "\n")
```

### API

| Function | Description |
|----------|-------------|
| `wkradar_new(spec_json)` | Build a radar from a spec JSON (errors on an invalid spec). |
| `wkradar_command(radar, cmd_json)` | Apply a command JSON, return the response JSON. |
| `wkradar_version()` | The library version. |

Domain errors (a bad spec, an unknown command) come back in-band as
`{"ok":false,"error":...}` JSON.

## Benchmark

Every binding forwards to the same data-driven Rust core, so what this one adds is
the call overhead of R's native `.Call` interface over the C ABI, not a different result. The core's throughput is
measured by the repository's benchmark suite and the nightly `bench.yml` run; the
numbers, the machine and how to reproduce them are in the repository
[BENCHMARKS.md](https://github.com/wickra-lib/wickra-radar/blob/main/BENCHMARKS.md).

## Documentation

The full guide, the spec reference and the API documentation live in the main
repository and the documentation site:

- **Repository:** <https://github.com/wickra-lib/wickra-radar>
- **Docs** (guides, spec reference, cookbook): <https://radar.wickra.org>
- **Runnable example:** [`examples/r/`](https://github.com/wickra-lib/wickra-radar/tree/main/examples/r)

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
