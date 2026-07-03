# Benchmarks

A radar's cost is dominated by folding a perp universe (open interest, funding,
order-book and liquidation events) into per-signal scores and aggregating a
`RadarAlert` across every symbol. The benchmarks here measure that **core scan
work**, so throughput scales predictably with the universe size and the number of
signals a spec references.

## What is measured

The `radar-bench` crate (criterion) covers a scan across a matrix of:

- **Universe size** — the number of symbols and the number of events per symbol
  folded before the alert is built.
- **Signals** — specs enabling one signal vs all five.
- **Mode** — a full batch `scan` vs the streaming `feed` + `alerts` path.

## Methodology

Run against fixed, in-process synthetic universes so the numbers are reproducible
and contain no I/O variance:

```bash
cargo bench -p radar-bench
```

## Results

Measured on one developer machine (release build, `parallel` feature), median
criterion estimates. Treat these as orders of magnitude, not guarantees — they
vary with CPU and toolchain.

A full batch `scan` with the composite spec (all five signals) over a synthetic
universe:

| Universe     | 16 events/perp | 64 events/perp |
|--------------|---------------:|---------------:|
| 100 perps    |       ~0.43 ms |       ~0.45 ms |
| 1,000 perps  |        ~3.7 ms |        ~3.8 ms |
| 10,000 perps |         ~37 ms |         ~40 ms |

The scan is **roughly linear in the number of symbols** — 10× the universe is
about 10× the time — because each symbol folds independently. It is nearly flat
across events-per-perp: each signal reads a small fixed window, so most of the
per-symbol cost is fixed overhead rather than proportional to the stream length.
At 10,000 perps × 64 events (640k events) a full five-signal scan runs in ~40 ms —
on the order of 16M events/second.

## Caveats

These figures bound the scan overhead only. End-to-end time in a real run also
depends on loading the events from disk or a live feed, which these in-process
benchmarks do not capture.
