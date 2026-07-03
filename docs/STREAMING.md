# Streaming

`scan` folds a whole event universe at once. The same core also runs
**incrementally**: feed events per symbol as they arrive and read the report at
any point. Both paths go through one shared aggregation, so they return
byte-identical JSON.

## `scan` vs `feed` + `alerts`

- **`scan`** takes the whole batch — `{"cmd":"scan","events":{"SYM":[…], …}}` —
  folds every symbol from scratch, and returns the `RadarReport`.
- **`feed`** applies one event to one symbol — `{"cmd":"feed","symbol":"AAA","event":{…}}` — updating that symbol's state in place.
- **`feed_batch`** applies a list of events to one symbol in order —
  `{"cmd":"feed_batch","symbol":"AAA","events":[…]}`.
- **`alerts`** reads the current `RadarReport` from the accumulated state, without
  feeding anything.

Feeding every event and then calling `alerts` yields **the same bytes** as passing
the same events to `scan` — both build the same per-symbol `SymbolState` and call
the same `report_from_states`. `tests/streaming_eq_batch.rs` pins this for every
golden spec, three ways: batch `scan`, per-event `feed` + `alerts`, and
`feed_batch` + `alerts`.

## Driving a live radar

```json
{"cmd":"feed","symbol":"BTCUSDT","event":{"kind":"derivatives","ts":1,"open_interest":100.0,"funding_rate":0.0001,"mark_price":60000.0}}
{"cmd":"feed","symbol":"BTCUSDT","event":{"kind":"derivatives","ts":2,"open_interest":110.0,"funding_rate":0.0001,"mark_price":60050.0}}
{"cmd":"alerts"}
```

Each `feed` is O(1) in the event and bounded in memory — the per-symbol buffers
are capped to the largest window any signal in the spec needs — so a radar can run
indefinitely over a live feed of thousands of perps. Call `alerts` on whatever
cadence your consumer wants (every tick, every second, on demand).

## Reset

`reset` clears all accumulated symbol state but keeps the spec, so `alerts`
afterwards returns `{"alerts":[],"scanned":0}`. A consumer that wants to start a
fresh window — a new session, a new day — just calls `reset` and keeps feeding;
the spec and every signal definition stay put. To change the signals instead, send
`set_spec` with a new `RadarSpec`.

## Determinism

Because a report is a pure fold over the per-symbol buffers, the same `(spec,
events)` always yields the same bytes — across languages (every binding returns
the core's JSON verbatim), across the streaming and batch paths, and across the
parallel (rayon) and sequential builds. That determinism is what lets the golden
corpus pin the output byte-for-byte; see [SCORING.md](SCORING.md) for how the
severity is kept `f64`-stable.
