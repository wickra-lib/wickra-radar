# wickra-radar (C#)

.NET bindings for [`wickra-radar`](https://github.com/wickra-lib/wickra-radar) over
the C ABI hub, via source-generated P/Invoke. Build a `Radar` from a spec JSON,
drive it with command JSON and read back alerts — the same protocol the CLI and
every other binding speak, returning the same bytes.

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

Requires .NET 8+. The native library (`wickra_radar`) must be resolvable on the
loader path — `PATH` on Windows, `LD_LIBRARY_PATH` on Linux, `DYLD_LIBRARY_PATH`
on macOS — or beside the assembly, where the bundled resolver finds it.

Licensed under either of [MIT](https://github.com/wickra-lib/wickra-radar/blob/main/LICENSE-MIT) or
[Apache-2.0](https://github.com/wickra-lib/wickra-radar/blob/main/LICENSE-APACHE) at your option.
