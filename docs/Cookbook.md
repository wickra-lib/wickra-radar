# Cookbook

Short, runnable recipes. Each drives the same core through the JSON command
protocol; the CLI examples assume you have built the workspace (`cargo build`).

## Scan a universe from the CLI

```bash
cargo run -p wickra-radar -- \
  --spec golden/specs/composite.json --stdin --format json < golden/events.json
```

The `--format json` output is exactly the bytes every binding returns from a
`scan` command. Drop `--format json` for a human-readable table of alerts.

## Feed a directory of per-symbol files

Instead of one stdin object, point `--events` at a directory of
`<SYMBOL>.jsonl` files (one JSON `Event` per line; the symbol is the file name):

```bash
cargo run -p wickra-radar -- --spec golden/specs/composite.json --events ./universe/
```

## Override threshold and limit per run

The spec's `threshold` and `limit` can be overridden without editing it — handy
for a fixed-size dashboard or a quieter alert stream:

```bash
cargo run -p wickra-radar -- \
  --spec golden/specs/composite.json --stdin --threshold 0.4 --limit 5 < golden/events.json
```

## Stream events and read alerts (Python)

```python
import json
from wickra_radar import Radar

spec = json.dumps({
    "symbols": ["AAA"],
    "signals": [{"kind": "funding_flip", "params": [0.0005]}],
    "threshold": 0.0,
})
r = Radar(spec)

# Feed events one at a time as they arrive from a live feed...
r.command(json.dumps({"cmd": "feed", "symbol": "AAA", "event":
    {"kind": "derivatives", "ts": 1, "open_interest": 1.0, "funding_rate": 0.0003, "mark_price": 50.0}}))
r.command(json.dumps({"cmd": "feed", "symbol": "AAA", "event":
    {"kind": "derivatives", "ts": 2, "open_interest": 1.0, "funding_rate": -0.0004, "mark_price": 50.0}}))

# ...then read the report whenever you want.
report = json.loads(r.command('{"cmd":"alerts"}'))
print(report["scanned"], report["alerts"][0]["severity"])  # 1 1.0
```

Passing the same two events to a single `scan` command returns the identical
report — the streaming and batch paths agree byte-for-byte.

## Reset for a new window

```python
r.command('{"cmd":"reset"}')
json.loads(r.command('{"cmd":"alerts"}'))  # {"alerts": [], "scanned": 0}
```

## Check the version

```bash
cargo run -p wickra-radar -- --version
```

or, from any binding, `{"cmd":"version"}` → `{"version":"0.1.0"}`.

## See also

[Architecture](ARCHITECTURE.md) · [Events](EVENTS.md) · [Signals](SIGNALS.md) ·
[Scoring](SCORING.md) · [Streaming](STREAMING.md).
