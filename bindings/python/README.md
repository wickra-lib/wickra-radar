<p align="center">
  <a href="https://wickra.org"><img src="https://raw.githubusercontent.com/wickra-lib/.github/main/profile/wickra-banner.webp?v=514-7" alt="Wickra Radar — a liquidation-cascade early-warning radar over 514 streaming indicators" width="100%"></a>
</p>

[![CI](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-radar/ci.svg)](https://github.com/wickra-lib/wickra-radar/actions/workflows/ci.yml)
[![codecov](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-radar/codecov.svg)](https://codecov.io/gh/wickra-lib/wickra-radar)
[![PyPI](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-radar/pypi.svg)](https://pypi.org/project/wickra-radar/)
[![License: MIT OR Apache-2.0](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-radar/license.svg)](https://github.com/wickra-lib/wickra-radar#license)

# Wickra Radar — Python

---

> **▶ Live demo:** all 514 indicators over real Binance market data, computed live in your browser — **[live.wickra.org](https://live.wickra.org)** · zero backend, powered by `wickra-wasm`.

**See liquidation cascades before they happen — for Python. `pip install wickra-radar` — prebuilt wheels for Linux, macOS and Windows, nothing to compile.**

Python bindings for [`wickra-radar-core`](https://github.com/wickra-lib/wickra-radar),
built with [PyO3] and [maturin]. The surface mirrors every other Wickra binding:
build a `Radar` from a spec JSON, drive it with command JSONs, and read back the
report.

## Install

```bash
pip install wickra-radar
```

Pre-built wheels ship for Linux, macOS and Windows — there is nothing to
compile and no C library to track down.

### Building from this repository (contributors)

```sh
maturin develop --release
pytest -q
```

## Quick start

```python
import json
from wickra_radar import Radar

spec = json.dumps({
    "symbols": ["AAA"],
    "signals": [{"kind": "funding_flip", "params": [0.0005]}],
    "threshold": 0.0,
})

radar = Radar(spec)
report = json.loads(radar.command(json.dumps({"cmd": "scan", "events": {
    "AAA": [
        {"kind": "derivatives", "ts": 1, "open_interest": 1.0, "funding_rate": 0.0003, "mark_price": 50.0},
        {"kind": "derivatives", "ts": 2, "open_interest": 1.0, "funding_rate": -0.0004, "mark_price": 50.0},
    ],
}})))
print(report["scanned"], report["alerts"][0]["symbol"])  # 1 AAA
```

### Surface

- **`Radar(spec_json)`** builds a radar from a spec JSON (`""` or `"{}"` for an
  empty handle whose spec is set later). Raises `ValueError` on a malformed spec.
- **`radar.command(cmd_json)`** applies a command JSON (`set_spec`, `feed`,
  `feed_batch`, `scan`, `alerts`, `reset`, `version`) and returns the response
  JSON — a `RadarReport` for `scan`/`alerts`. Domain errors come back in-band as
  `{"ok":false,"error":...}`.
- **`Radar.version()`** returns the library version.

## Benchmark

Every binding forwards to the same data-driven Rust core, so what this one adds is
the call overhead of PyO3, not a different result. The core's throughput is
measured by the repository's benchmark suite and the nightly `bench.yml` run; the
numbers, the machine and how to reproduce them are in the repository
[BENCHMARKS.md](https://github.com/wickra-lib/wickra-radar/blob/main/BENCHMARKS.md).

## Documentation

The full guide, the spec reference and the API documentation live in the main
repository and the documentation site:

- **Repository:** <https://github.com/wickra-lib/wickra-radar>
- **Docs** (guides, spec reference, cookbook): <https://radar.wickra.org>
- **Runnable example:** [`examples/python/`](https://github.com/wickra-lib/wickra-radar/tree/main/examples/python)

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

[PyO3]: https://pyo3.rs
[maturin]: https://www.maturin.rs
