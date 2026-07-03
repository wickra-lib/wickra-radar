//! Criterion benchmarks for `scan`: how the fold scales with the universe size
//! (100 / 1k / 10k perps) and the number of events per perp (16 vs 64). The same
//! benchmark, run with and without the `parallel` feature, measures the rayon
//! path against the sequential one.

use std::collections::BTreeMap;
use std::hint::black_box;

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use radar_core::{scan, Event, RadarSpec, Side, Signal, SignalKind};

/// A synthetic event stream for one perp: `n` events on a varied (non-geometric)
/// price and open-interest path, mixing derivatives, order-book and liquidation
/// events so every signal has data to score.
fn events_for(seed: usize, n: usize) -> Vec<Event> {
    let mut events = Vec::with_capacity(n);
    for index in 0..n {
        let step = f64::from(u32::try_from(index).unwrap());
        let ts = i64::try_from(index).unwrap() + 1;
        let price = 100.0 + 10.0 * (step / 8.0).sin() + 0.05 * step;
        let oi = 500.0 + step + f64::from(u32::try_from(seed % 100).unwrap());
        match index % 4 {
            0 => events.push(Event::Orderbook {
                ts,
                bid_volume: 800.0 + step,
                ask_volume: 1200.0,
                best_bid: price - 0.5,
                best_ask: price + 0.5,
            }),
            3 => events.push(Event::Liquidation {
                ts,
                side: if index % 2 == 0 {
                    Side::Buy
                } else {
                    Side::Sell
                },
                qty: 2.0 + step * 0.1,
                price,
            }),
            _ => events.push(Event::Derivatives {
                ts,
                open_interest: oi,
                funding_rate: 0.0002 * (if index % 8 < 4 { 1.0 } else { -1.0 }),
                mark_price: price,
            }),
        }
    }
    events
}

/// A universe of `perps` symbols, each with `per_perp` events.
fn universe(perps: usize, per_perp: usize) -> BTreeMap<String, Vec<Event>> {
    (0..perps)
        .map(|i| (format!("PERP{i:05}"), events_for(i, per_perp)))
        .collect()
}

/// A composite spec exercising all five signals.
fn spec() -> RadarSpec {
    RadarSpec {
        venues: vec![],
        symbols: vec![],
        signals: vec![
            Signal {
                kind: SignalKind::OiDelta,
                params: vec![4.0, 0.1],
                weight: 1.0,
            },
            Signal {
                kind: SignalKind::FundingFlip,
                params: vec![0.0005],
                weight: 2.0,
            },
            Signal {
                kind: SignalKind::BookImbalance,
                params: vec![1.0],
                weight: 1.0,
            },
            Signal {
                kind: SignalKind::LiqCluster,
                params: vec![8.0, 30.0],
                weight: 1.5,
            },
            Signal {
                kind: SignalKind::OiPriceDivergence,
                params: vec![4.0, 0.1],
                weight: 3.0,
            },
        ],
        threshold: 0.0,
        limit: None,
    }
}

fn bench_scan(criterion: &mut Criterion) {
    let spec = spec();
    let mut group = criterion.benchmark_group("scan");
    group.sample_size(10);
    for &perps in &[100usize, 1_000, 10_000] {
        for &per_perp in &[16usize, 64] {
            let universe = universe(perps, per_perp);
            let total = u64::try_from(perps * per_perp).unwrap();
            group.throughput(Throughput::Elements(total));
            group.bench_with_input(
                BenchmarkId::from_parameter(format!("{perps}perp_{per_perp}ev")),
                &(&universe, &spec),
                |bencher, (universe, spec)| {
                    bencher.iter(|| black_box(scan(black_box(universe), black_box(spec))));
                },
            );
        }
    }
    group.finish();
}

criterion_group!(benches, bench_scan);
criterion_main!(benches);
