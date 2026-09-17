<p align="center">
  <a href="https://wickra.org"><img src="https://raw.githubusercontent.com/wickra-lib/.github/main/profile/wickra-banner.webp?v=514-7" alt="Wickra Radar — a liquidation-cascade early-warning radar over 514 streaming indicators" width="100%"></a>
</p>

[![CI](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-radar/ci.svg)](https://github.com/wickra-lib/wickra-radar/actions/workflows/ci.yml)
[![codecov](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-radar/codecov.svg)](https://codecov.io/gh/wickra-lib/wickra-radar)
[![NuGet](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-radar/nuget.svg)](https://www.nuget.org/packages/Wickra.Radar)
[![License: MIT OR Apache-2.0](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-radar/license.svg)](https://github.com/wickra-lib/wickra-radar#license)

# Wickra Radar — C#

---

> **▶ Live demo:** all 514 indicators over real Binance market data, computed live in your browser — **[live.wickra.org](https://live.wickra.org)** · zero backend, powered by `wickra-wasm`.

**See liquidation cascades before they happen — for C#. `dotnet add package Wickra.Radar` — prebuilt native library, no system dependencies.**

.NET bindings for [`wickra-radar`](https://github.com/wickra-lib/wickra-radar) over
the C ABI hub, via source-generated P/Invoke. Build a `Radar` from a spec JSON,
drive it with command JSON and read back alerts — the same protocol the CLI and
every other binding speak, returning the same bytes.

## Install

```bash
dotnet add package Wickra.Radar
```

The native library ships prebuilt per platform under `runtimes/<rid>/native/`,
selected automatically. There is nothing to compile. Targets .NET 8 and later.

Requires .NET 8+. The native library (`wickra_radar`) must be resolvable on the
loader path — `PATH` on Windows, `LD_LIBRARY_PATH` on Linux, `DYLD_LIBRARY_PATH`
on macOS — or beside the assembly, where the bundled resolver finds it.

## Quick start

```csharp
using Wickra.Radar;

const string spec = """
{"symbols":["BTCUSDT","ETHUSDT"],
 "signals":[{"kind":"funding_flip","params":[0.0005]},
            {"kind":"oi_delta","params":[3,0.1]}],
 "threshold":0.2}
""";

using var radar = new Radar(spec);

// One-shot: scan a whole universe of events at once.
string report = radar.Command("""{"cmd":"scan","events":{"BTCUSDT":[ … ],"ETHUSDT":[ … ]}}""");

// Streaming: feed events as they arrive, then read the current alerts.
radar.Command("""{"cmd":"feed","symbol":"BTCUSDT","event":{"kind":"derivatives","ts":1,"open_interest":100.0,"funding_rate":0.0001,"mark_price":60000.0}}""");
string alerts = radar.Command("""{"cmd":"alerts"}""");
```

Both operating modes return the same report bytes for the same events: `scan`
over a universe equals `feed` / `feed_batch` per symbol followed by `alerts`.
The core pins this in Rust and the binding's test suite checks it across the
committed golden corpus.

Every `Command` call is executed exactly once. The C ABI's length-then-fill
protocol needs two calls when the first buffer is too small, and the hub caches
the response of a mutating command (`feed`, `feed_batch`) until it has been
delivered, so a retry never feeds an event twice.

## Benchmark

Every binding forwards to the same data-driven Rust core, so what this one adds is
the call overhead of `[LibraryImport]` P/Invoke over the C ABI, not a different result. The core's throughput is
measured by the repository's benchmark suite and the nightly `bench.yml` run; the
numbers, the machine and how to reproduce them are in the repository
[BENCHMARKS.md](https://github.com/wickra-lib/wickra-radar/blob/main/BENCHMARKS.md).

## Documentation

The full guide, the spec reference and the API documentation live in the main
repository and the documentation site:

- **Repository:** <https://github.com/wickra-lib/wickra-radar>
- **Docs** (guides, spec reference, cookbook): <https://radar.wickra.org>
- **Runnable example:** [`examples/csharp/`](https://github.com/wickra-lib/wickra-radar/tree/main/examples/csharp)

Wickra Radar ships native bindings for Python, Node.js, WASM and Rust, plus a C ABI hub that any
C-capable language (C, C++, C#, Go, Java, R) links against — all forwarding to the
same data-driven, `unsafe`-forbidden Rust core.

## Security

Found a security issue? **Please don't open a public issue.** Report it privately
via the repository's *Security* tab (*"Report a vulnerability"*) or email
**support@wickra.org** with a subject line starting `[wickra security]`. Full
policy: <https://github.com/wickra-lib/wickra-radar/blob/main/SECURITY.md>.

## Disclaimer

Wickra Radar is analysis software: it computes early-warning signals over
historical and live market data. It is provided "as is", without warranty of any
kind, and is **not financial advice** — it places no orders. Trading carries risk
of loss; review the code and use at your own discretion.

## License

Licensed under either of [Apache-2.0](https://github.com/wickra-lib/wickra-radar/blob/main/LICENSE-APACHE)
or [MIT](https://github.com/wickra-lib/wickra-radar/blob/main/LICENSE-MIT) at your option.
