# Events

The radar folds a perp universe: a map of `symbol → [Event, …]`. An `Event` is an
internally-tagged JSON object (`{"kind": "...", ...}`, `snake_case`), and the
field names mirror the [`wickra-exchange`](https://github.com/wickra-lib/wickra-exchange)
derivative / order-book / liquidation streams
(`crates/radar-core/src/event.rs`). Every timestamp is an integer `ts` (seconds
or milliseconds, consistent with the feed).

```json
{
  "BTCUSDT": [
    { "kind": "derivatives", "ts": 1, "open_interest": 100.0, "funding_rate": 0.0001, "mark_price": 60000.0 },
    { "kind": "orderbook",   "ts": 2, "bid_volume": 1000.0, "ask_volume": 1400.0, "best_bid": 59990.0, "best_ask": 60010.0 },
    { "kind": "liquidation", "ts": 3, "side": "sell", "qty": 5.0, "price": 59800.0 }
  ]
}
```

## Event kinds

- **`derivatives`** — `{ ts, open_interest, funding_rate, mark_price }`. A perp
  snapshot: open interest (contracts or base units), the current funding rate (a
  small signed fraction), and the mark price. Feeds `oi_delta`, `funding_flip` and
  `oi_price_divergence`.
- **`orderbook`** — `{ ts, bid_volume, ask_volume, best_bid, best_ask }`. A
  top-of-book snapshot: resting bid/ask volume near the top and the best bid/ask.
  Only the latest snapshot matters; it feeds `book_imbalance`.
- **`liquidation`** — `{ ts, side, qty, price }`. A forced close. `side` is
  `buy` (a **long** was liquidated — a buy to close) or `sell` (a **short** was
  liquidated). Feeds `liq_cluster`.

## How events are folded

Each symbol keeps a `SymbolState` — small windowed buffers, updated O(1) per
event: the open-interest and mark-price series (for the windowed OI/price
signals), the funding-rate series (last two values decide a flip), the most
recent order-book snapshot, and a rolling buffer of liquidation quantities. The
buffers are capped to the largest window any signal in the spec needs, so memory
per symbol is bounded no matter how long the stream runs.

Events do **not** have to arrive in order within `scan` — each symbol is folded in
array order — but a real feed delivers them by time, and the alert's `ts` is the
symbol's last-seen event timestamp.

## Where events come from

The CLI reads them from `--stdin` (one JSON object `{"SYM":[…], …}`) or from
`--events <dir>`, a directory of per-symbol `<SYMBOL>.jsonl` files (one JSON
`Event` per line). A live feed produces the same shapes from the
`wickra-exchange` derivative / book / liquidation streams, so a replayed capture
and a synthetic fixture are interchangeable. The `golden/events.json` fixture is a
deterministic example of all three kinds; see
[`golden/README.md`](../golden/README.md).
