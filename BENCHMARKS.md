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

_To be filled in from the criterion run in the test-rigor / docs phase._ Figures
will be the median estimate on a single machine; treat them as orders of
magnitude, not guarantees — they vary with CPU and toolchain.

## Caveats

These figures bound the scan overhead only. End-to-end time in a real run also
depends on loading the events from disk or a live feed, which these in-process
benchmarks do not capture.
