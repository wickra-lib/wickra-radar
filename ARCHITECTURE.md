# Architecture

`wickra-radar` is one data-driven core with many thin consumers. An alert is a
piece of **data** — a `RadarAlert`, built from a serde `RadarSpec` folded over a
perp universe (open interest, funding, order-book and liquidation events). Because
the alert is data, not renderer commands, the exact same result is produced
natively, across the C ABI and in WASM, byte-for-byte identical — and stays
identical between the parallel (rayon) and sequential builds.

## The layers

```
CONSUMERS   CLI: crates/radar-cli   ·   [optional] TUI: crates/radar-tui   ·   any language via its binding (command JSON)
      ▲ RadarAlert JSON (sorted: severity desc, then symbol)                                     ▲
CORE  crates/radar-core:  RadarSpec (JSON) → Universe<Symbol, SymbolState> (O(1)/event)
                          → per-signal score → weighted aggregation → RadarAlert
      ▼ data-driven JSON API in ten languages (like screener command_json / backtest run_json)
BINDINGS  python · node · wasm · c (C-ABI hub) → c / c++ / c# / go / java / r
CORES  wickra-core (derivatives / liquidation indicators) · wickra-exchange (live perp feeds = event shapes) · wickra-data (Candle, optional)
```

Each binding ships the same surface — a `Radar` handle plus
`command(json) -> json` and `version` — with its own README, tests, a runnable
example, and a completeness guard.

## The core is data-driven

A `RadarAlert` is a serde data-model, not a list of instructions, and the signals
that produce it are **data too**: a `RadarSpec` carries a `Vec<Signal>`, where
each signal is a `kind` + `params` + `weight`, never a Rust closure. Closures
cannot cross the C ABI or a WASM boundary; a serde data-model can, so a Python,
Go or browser consumer computes the identical alert a Rust consumer would.

## Two modes, one core, one result type

- **Batch** — `scan(universe_events, spec)` folds each perp over its whole event
  stream, scores every signal at the final state, and aggregates with weights.
  It runs in parallel across symbols via rayon (the default `parallel` feature)
  or sequentially as the WASM fallback — byte-identical either way.
- **Streaming** — `feed(symbol, event)` + `alerts()` update O(1) per event, a
  live radar over the current state of every perp.

Both modes share the same scoring and aggregation, so streaming and batch produce
the same alert for the same events.

## The five signals

Every signal yields a **normalised score in `[0, 1]`** — "how strongly this
signal fires for this perp" — which the spec weights and aggregates into a
severity:

- **`oi_delta`** — relative change in open interest over a window of events
  (leverage building up).
- **`funding_flip`** — funding-rate sign flips and extremes (crowded, stressed
  positioning).
- **`book_imbalance`** — resting bid/ask volume imbalance at the top of book.
- **`liq_cluster`** — a burst of liquidation events in a short window (a cascade
  already in progress).
- **`oi_price_divergence`** — open interest rising while price fails to follow
  (trapped positioning).

## Cross-section by design

Cascade risk is inherently a market-wide phenomenon: OI-delta clusters, funding
flips and book imbalance across **all** perps at once reveal the systemic stress
a single-symbol, price-only tool never sees. The `Universe` holds the state of
every symbol; only **ready** symbols — those that have seen enough events for
each referenced signal's warmup — contribute to an alert.

## The command boundary

Every consumer talks to the core through a single JSON-in / JSON-out function,
`Radar::command`. The binding does no logic of its own — it forwards the command
string and returns the core's response verbatim. That verbatim pass-through is
what makes the golden corpus a **cross-language** parity corpus: the same command
produces a byte-identical alert in every language, with no per-language JSON
reformatting.

## Indicators come from the Wickra core

No indicator mathematics lives in this repository. Where a signal needs a derived
series, `IndicatorSet` resolves each building block from the `wickra-core`
registry by name and parameters (the same resolver the backtester uses), so
`wickra-radar` inherits all 514 indicators — including the derivatives,
liquidation-rate and order-book-imbalance families — and any future additions for
free.

## Integration with the rest of Wickra

`wickra-radar` sits beside the other Wickra consumers — the terminal, the
screener, the X-ray and the backtester — over the same core. It depends on
`wickra-core` (indicators) and on `wickra-exchange`, whose perp derivative,
order-book and liquidation streams define the shapes of the radar's input events;
`wickra-data` (`Candle` + CSV) is optional, for price-derived signals. It only
reads and analyses market data — it never places orders and holds no order-secret
material.
