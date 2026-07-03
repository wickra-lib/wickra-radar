# Wickra Radar — C&#35;

C# bindings for the `wickra-radar` data-driven core over its C ABI hub
(source-generated P/Invoke). Build a `Radar` from a spec JSON, drive it with
command JSON, read back the report — the same protocol as every other binding.

## Install

```bash
dotnet add package Wickra.Radar
```

## Usage

```csharp
using Wickra.Radar;

const string spec = """
{"symbols":["AAA"],"signals":[{"kind":"funding_flip","params":[0.0005]}],"threshold":0.0}
""";

using var radar = new Radar(spec);

string report = radar.Command("""
{"cmd":"scan","events":{"AAA":[
{"kind":"derivatives","ts":1,"open_interest":1.0,"funding_rate":0.0003,"mark_price":50.0},
{"kind":"derivatives","ts":2,"open_interest":1.0,"funding_rate":-0.0004,"mark_price":50.0}]}}
""");
Console.WriteLine(report);
Console.WriteLine(Radar.Version());
```

## API

| Member | Description |
|--------|-------------|
| `new Radar(string specJson)` | Build a radar from a spec JSON (throws `ArgumentException` on an invalid spec). |
| `string Command(string cmdJson)` | Apply a command JSON, return the response JSON. |
| `static string Version()` | The library version. |
| `Dispose()` | Free the native handle (via `IDisposable`). |

Domain errors (a bad spec, an unknown command) come back in-band as
`{"ok":false,"error":...}` JSON. The native library is located by a
`DllImportResolver` that probes the default search paths and the Cargo `target/`
directory, validating each candidate with a sentinel export check.

## License

`MIT OR Apache-2.0`.
