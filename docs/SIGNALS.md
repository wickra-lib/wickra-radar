# Signals

A spec lists one or more weighted signals; each scores how strongly it fires for
one perp, as a number in `[0, 1]`. The scoring lives in
`crates/radar-core/src/symbol_state.rs` (`score`), reading the per-symbol windowed
buffers. Each signal has a `kind`, a `params` array (arity checked by
`RadarSpec::validate`) and an optional `weight` (default `1.0`). A signal that has
not seen enough data yet scores `0.0`, and any non-finite intermediate result is
clamped to `0.0`; every score is clamped to `[0, 1]`.

Each signal's canonical **key** is `"<kind>(<params>)"` (e.g.
`"oi_delta(20,0.1)"`), and that key is its entry in an alert's `factors` map.

## `oi_delta` — `[window, ref_pct]`

Relative change in open interest over `window` events — leverage building up.

```json
{ "kind": "oi_delta", "params": [20.0, 0.1], "weight": 1.0 }
```

`score = |(oi_now − oi_window_ago) / oi_window_ago| / ref_pct`, clamped to
`[0, 1]`. A `ref_pct` of `0.1` means "a 10% move over the window scores 1.0".
Ready once more than `window` derivatives events have arrived.

## `funding_flip` — `[flip_threshold]`

Funding-rate magnitude, doubled on a sign flip — positioning stress.

```json
{ "kind": "funding_flip", "params": [0.0005] }
```

`base = |funding_now| / flip_threshold`; if the sign flipped since the previous
funding value (`funding_now × funding_prev < 0`) the score is `base × 2`,
otherwise `base` — then clamped. Ready once at least two derivatives events have
arrived.

## `book_imbalance` — `[ref_abs]`

Top-of-book bid/ask volume imbalance.

```json
{ "kind": "book_imbalance", "params": [1.0] }
```

`imb = (bid_volume − ask_volume) / (bid_volume + ask_volume)`;
`score = |imb| / ref_abs`, clamped. A `ref_abs` of `1.0` means "a fully
one-sided book scores 1.0". Reads the most recent `orderbook` snapshot; ready
once one has arrived.

## `liq_cluster` — `[window, ref_qty]`

Sum of liquidation quantity over the last `window` liquidations — a cascade in
progress.

```json
{ "kind": "liq_cluster", "params": [5.0, 30.0] }
```

`score = (Σ qty over the last window liquidations) / ref_qty`, clamped. A
`ref_qty` of `30.0` means "30 units of liquidations in the window scores 1.0".
Ready once any liquidation has arrived.

## `oi_price_divergence` — `[window, ref_pct]`

Open interest rising while price falls, or vice versa — trapped positioning, the
core early-warning signal.

```json
{ "kind": "oi_price_divergence", "params": [20.0, 0.1], "weight": 3.0 }
```

With `d_oi` and `d_px` the relative changes in open interest and mark price over
`window`, `score = max(−d_oi × d_px, 0) / ref_pct²`, clamped. The product is
positive only when OI and price move in **opposite** directions; same-direction
moves score `0.0`. Ready once more than `window` derivatives events have arrived.

## Weights and the `factors` map

Each signal's `weight` scales its contribution to the aggregated severity (see
[SCORING.md](SCORING.md)); it defaults to `1.0` and must be finite and
non-negative. Two signals may not share a `key` — that would collide in the
`factors` map — so `RadarSpec::validate` rejects a spec with a duplicate
`kind(params)`.
