# Architecture (internals)

The top-level [ARCHITECTURE.md](../ARCHITECTURE.md) gives the high-level shape;
this page covers how the core actually turns a spec + an event universe into a
report. The whole product is **one data-driven core** (`radar-core`) and N thin
consumers — the CLI and the ten language bindings — each of which only ships a
spec, feeds events and reads back a report.

## The pipeline

```
RadarSpec (JSON/TOML)             events: { symbol -> [Event, …] } (JSON)
   │  parse + validate               │  parse (internally-tagged by "kind")
   │  (≥1 signal, right arity,         ▼
   │   finite weights/threshold)   per symbol: SymbolState fold (O(1)/event)
   ▼                                  │  windowed OI / funding / book / liq buffers
scan(events, spec)  ◄─────────────────┘
   │  for each symbol → severity(state, spec) → RadarAlert
   │  filter ≥ threshold → sort → cap to limit
   ▼
RadarReport { alerts: Vec<RadarAlert>, scanned }
   │  serde_json::to_string  (compact, fixed-precision floats)
   ▼
the exact bytes every binding returns from a `scan` command
```

- **`RadarSpec`** (`crates/radar-core/src/spec.rs`) — `venues`, `symbols`, an ordered list of weighted `Signal`s, a `threshold` and an optional `limit`. `validate()` rejects empty signals, wrong parameter arity, a non-finite/negative weight, a non-finite threshold, a zero limit, and two signals sharing a `factors` key.
- **`Event`** (`src/event.rs`) — an internally-tagged enum: `derivatives`, `orderbook`, `liquidation`. See [EVENTS.md](EVENTS.md).
- **`SymbolState`** (`src/symbol_state.rs`) — per-symbol windowed buffers folded one event at a time; `score(signal)` reads them into a `[0, 1]` score. See [SIGNALS.md](SIGNALS.md).
- **`severity`** (`src/aggregate.rs`) — the weighted mean of a symbol's signal scores, plus the self-explaining `factors` map. See [SCORING.md](SCORING.md).
- **`scan`** (`src/scan.rs`) — folds every symbol, scores it, filters by threshold, sorts and caps.

## Parallel vs sequential

With the default `parallel` feature the per-symbol folds run across rayon; without
it (the WASM build, `--no-default-features`) they run sequentially. The two are
**byte-for-byte identical** — alerts sort by a total order (`f64::total_cmp` on
severity, then symbol ascending), never a partial float compare, and every score
is rounded to a fixed `1e-8` precision. The golden suite runs under both feature
sets in CI to prove it.

## The command protocol

Every binding drives the core through one entry point, `Radar::command_json`,
whose envelope is `{"cmd": "..."}` and whose reply is **always a JSON string**. A
domain error is returned in-band as `{"ok":false,"error":"..."}` — never a panic
or a thrown exception. The commands are `set_spec`, `feed`, `feed_batch`, `scan`,
`alerts`, `reset` and `version`. Because the reply is the core's compact JSON
verbatim, the report is byte-identical across every language.

## Data-driven boundary

The report is **data, not code**: a `RadarAlert` carries a `symbol`, a `severity`,
a `factors` breakdown and a `ts`, and a consumer decides what to do with it. That
is why the same output crosses the C ABI and WASM unchanged, and why an alert
router, a dashboard or a bot can be written in any language without linking the
core's internals.
