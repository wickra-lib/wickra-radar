# Golden fixtures

Cross-language parity fixtures. Every binding builds a radar from each
`specs/*.json`, runs `scan` over the shared `events.json`, and asserts the
response equals `expected/<spec>.json` **byte-for-byte**. Because each binding
returns the core's compact `command_json` string verbatim, byte equality is the
exact cross-language check — the same bytes must come out of Rust, Python,
Node.js, WASM, C, C++, C#, Go, Java and R.

## Files

- **`events.json`** — the shared perp universe: a `{ "<symbol>": [event, …] }`
  map fed to every spec via `{"cmd":"scan","events":<events.json>}`. Five perps,
  each shaped to exercise one cascade signal:
  - `BTCUSDT` — open interest climbing (`oi_delta`); price rises with it, so the
    divergence signal is cleanly zero.
  - `ETHUSDT` — funding rate flips `+ → −` (`funding_flip`).
  - `SOLUSDT` — open interest up while price falls (`oi_price_divergence`).
  - `DOGEUSDT` — a cluster of liquidations (`liq_cluster`).
  - `XRPUSDT` — a lopsided order book (`book_imbalance`).
- **`specs/*.json`** — five specs: one per single signal, plus `composite.json`
  which weights all five and applies a `threshold` and a `limit` to exercise the
  aggregation, the total-order sort (severity desc, symbol asc) and truncation.
- **`expected/<spec>.json`** — the blessed report for each spec, one line of
  compact JSON.

## Regenerating the blessed output

The expected files are produced by the reference CLI (its JSON output is the same
`serde_json` serialization as `command_json`), so they can be regenerated after an
intentional change:

```bash
cargo build -p wickra-radar
for spec in golden/specs/*.json; do
  cargo run -q -p wickra-radar -- --spec "$spec" --stdin --format json \
    < golden/events.json > "golden/expected/$(basename "$spec")"
done
```

Fixtures must avoid values whose serialization is build-dependent (e.g. a signed
zero from a signal that is exactly zero): keep every score an unambiguous
positive number so the bytes match across every toolchain and optimization level.
