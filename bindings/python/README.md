# Wickra Radar — Python

Python bindings for [`radar-core`](https://github.com/wickra-lib/wickra-radar),
built with [PyO3] and [maturin]. The surface mirrors every other Wickra binding:
build a `Radar` from a spec JSON, drive it with command JSONs, and read back the
report.

## Install

```sh
pip install wickra-radar
```

## Usage

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

## Surface

- **`Radar(spec_json)`** builds a radar from a spec JSON (`""` or `"{}"` for an
  empty handle whose spec is set later). Raises `ValueError` on a malformed spec.
- **`radar.command(cmd_json)`** applies a command JSON (`set_spec`, `feed`,
  `feed_batch`, `scan`, `alerts`, `reset`, `version`) and returns the response
  JSON — a `RadarReport` for `scan`/`alerts`. Domain errors come back in-band as
  `{"ok":false,"error":...}`.
- **`Radar.version()`** returns the library version.

## Build from source

```sh
maturin develop --release
pytest -q
```

[PyO3]: https://pyo3.rs
[maturin]: https://www.maturin.rs
