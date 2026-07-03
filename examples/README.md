# Examples

A runnable "scan a perp universe" example in every language. Each one builds a
Radar from the same spec (a single `funding_flip` signal), scans a two-event
inline universe (funding flips `+0.0003 → -0.0004` on `AAA`) and prints the
version and the report. The examples are self-contained: the spec and events are
inline, so there is no shared `data/` directory to load (the cross-language golden
fixtures live in [`../golden/`](../golden)).

| Language | Path | Run |
|----------|------|-----|
| Rust | [`rust/`](rust/) | `cargo run -p wickra-radar-example` |
| Python | [`python/scan.py`](python/scan.py) | `pip install wickra-radar && python examples/python/scan.py` |
| Node.js | [`node/`](node/) | `cd examples/node && npm install && node scan.js` |
| C / C++ | [`c/`](c/) | see below |
| Go | [`go/`](go/) | `cd examples/go && go run .` |
| C# | [`csharp/Scan/`](csharp/Scan/) | `dotnet run --project examples/csharp/Scan` |
| Java | [`java/Scan.java`](java/Scan.java) | see the header comment |
| R | [`r/scan.R`](r/scan.R) | `Rscript examples/r/scan.R` |

The native bindings (Python, Node.js) load their own compiled library. The bindings
that go through the C ABI (Go, C#, Java, R, and the C / C++ example itself) need the
C ABI library built first:

```bash
cargo build --release -p wickra-radar-c
```

## C / C++

The C and C++ examples build with CMake and run under ctest:

```bash
cargo build --release -p wickra-radar-c
cmake -S examples/c -B examples/c/build
cmake --build examples/c/build --config Release
ctest --test-dir examples/c/build -C Release --output-on-failure
```

On Windows the build copies `wickra_radar.dll` next to each executable, since there
is no rpath.

## Expected output

Every example prints the version and the report, for example:

```text
wickra-radar 0.1.0
{"alerts":[{"symbol":"AAA","severity":1.0,"factors":{"funding_flip(0.0005)":1.0,"severity":1.0},"ts":2}],"scanned":1}
```
