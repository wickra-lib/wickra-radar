# Scoring

A scan turns each symbol's per-signal scores into one **severity** and a
self-explaining **factors** breakdown, then filters, sorts and caps the universe
into a report. The aggregation lives in `crates/radar-core/src/aggregate.rs`; the
scan assembly in `src/scan.rs`.

## Severity — the weighted mean

```
severity = (Σ wᵢ · sᵢ) / (Σ wᵢ)
```

where `sᵢ` is signal `i`'s `[0, 1]` score (see [SIGNALS.md](SIGNALS.md)) and `wᵢ`
its weight. When the weights sum to zero the severity is `0.0`. The sums run in
signal declaration order and the result is rounded to a fixed `1e-8` precision, so
the `f64` output is identical in every language and between the parallel and
sequential builds.

## The `factors` map

Every alert carries a `factors` map: each signal's rounded score keyed by its
canonical [`key`](SIGNALS.md) (`"<kind>(<params>)"`), plus a `"severity"` entry.
So an alert explains exactly which signal contributed how much:

```json
{
  "symbol": "SOLUSDT",
  "severity": 0.51764706,
  "factors": {
    "book_imbalance(1)": 0.0,
    "funding_flip(0.0005)": 0.2,
    "liq_cluster(5,30)": 0.0,
    "oi_delta(2,0.1)": 1.0,
    "oi_price_divergence(2,0.1)": 1.0,
    "severity": 0.51764706
  },
  "ts": 3
}
```

The per-signal scores are recorded even when a weight is zero, so you can watch a
signal you have chosen not to weight into the severity.

## The report

`scan` produces a `RadarReport { alerts, scanned }`:

1. **Score** every symbol in the universe.
2. **Filter** to alerts whose `severity ≥ threshold` (default `0.0`, so every
   scanned symbol is kept).
3. **Sort** by `severity` descending, then `symbol` ascending. The order is a
   total order — `f64::total_cmp` on severity — so it is deterministic even for
   equal severities and identical across builds.
4. **Cap** to the top `limit` alerts when the spec sets one.

`scanned` is the number of symbols the scan looked at (the whole fed universe),
independent of how many cleared the threshold — so you can tell "3 alerts out of
500 scanned" from "3 alerts out of 3 scanned".

## Tuning threshold and limit

- **`threshold`** trades recall for noise: `0.0` surfaces everything (ranked),
  `0.5` keeps only symbols where the weighted signals are, on balance, firing
  hard.
- **`limit`** caps the alert list to the most severe N — useful for a fixed-size
  dashboard or a rate-limited alerter. Both can be overridden per run from the
  CLI (`--threshold`, `--limit`) without editing the spec.
