<p align="center">
  <a href="https://wickra.org"><img src="https://raw.githubusercontent.com/wickra-lib/.github/main/profile/wickra-banner.webp?v=514" alt="Wickra Radar — a liquidation-cascade early-warning radar over 514 streaming indicators" width="100%"></a>
</p>

[![CI](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-radar/ci.svg)](https://github.com/wickra-lib/wickra-radar/actions/workflows/ci.yml)
[![codecov](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-radar/codecov.svg)](https://codecov.io/gh/wickra-lib/wickra-radar)
[![GitHub release](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-radar/release.svg)](https://github.com/wickra-lib/wickra-radar/releases/latest)
[![License: MIT OR Apache-2.0](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-radar/license.svg)](https://github.com/wickra-lib/wickra-radar#license)

# Wickra Radar — C / C++

---

> **▶ Live demo:** all 514 indicators over real Binance market data, computed live in your browser — **[live.wickra.org](https://live.wickra.org)** · zero backend, powered by `wickra-wasm`.

**See liquidation cascades before they happen — for C / C++. `cargo build -p wickra-radar-c --release` — a prebuilt shared/static library plus a generated `wickra_radar.h`, no system dependencies.**

The C ABI is the hub every C-capable language (C, C++, C#, Go, Java, R) links
against. It exposes `wickra-radar-core` as a tiny, JSON-shaped surface built as both a
`cdylib` (dynamic library) and a `staticlib`.

## Install

Grab the prebuilt header + library for your platform from the
[GitHub releases](https://github.com/wickra-lib/wickra-radar/releases) — each archive
has `wickra_radar.h`, the C++ wrapper where the binding ships one, and the shared/static
library — or build from source:

```bash
cargo build -p wickra-radar-c --release
# -> target/release/libwickra_radar.{so,dylib} or wickra_radar.dll (+ import lib) + a staticlib
```

Then compile against the header and link the library.

## Quick start

[`examples/c/scan.c`](https://github.com/wickra-lib/wickra-radar/blob/main/examples/c/scan.c) is the runnable example the CI smoke job executes; in full:

```c
/* A minimal C example: scan a perp universe through the wickra-radar C ABI. */
#include <stdio.h>
#include <stdlib.h>

#include "wickra_radar.h"

static const char *SPEC =
    "{\"symbols\":[\"AAA\"],\"signals\":["
    "{\"kind\":\"funding_flip\",\"params\":[0.0005]}],\"threshold\":0.0}";

static const char *SCAN =
    "{\"cmd\":\"scan\",\"events\":{\"AAA\":["
    "{\"kind\":\"derivatives\",\"ts\":1,\"open_interest\":1.0,\"funding_rate\":0.0003,\"mark_price\":50.0},"
    "{\"kind\":\"derivatives\",\"ts\":2,\"open_interest\":1.0,\"funding_rate\":-0.0004,\"mark_price\":50.0}]}}";

/* Length-out protocol: learn the length, then read into a caller buffer.
   Returns a malloc'd NUL-terminated string the caller must free, or NULL. */
static char *run(WickraRadar *radar, const char *cmd) {
    int len = wickra_radar_command(radar, cmd, NULL, 0);
    if (len < 0) {
        fprintf(stderr, "command failed: code %d\n", len);
        return NULL;
    }
    char *buf = (char *)malloc((size_t)len + 1);
    if (!buf) {
        return NULL;
    }
    wickra_radar_command(radar, cmd, buf, (size_t)len + 1);
    return buf;
}

int main(void) {
    WickraRadar *radar = wickra_radar_new(SPEC);
    if (!radar) {
        fprintf(stderr, "failed to build radar\n");
        return 1;
    }

    char *report = run(radar, SCAN);
    if (!report) {
        wickra_radar_free(radar);
        return 1;
    }

    printf("wickra-radar %s\n", wickra_radar_version());
    printf("report: %s\n", report);

    free(report);
    wickra_radar_free(radar);
    return 0;
}
```

### Surface

```c
#include "wickra_radar.h"

WickraRadar *wickra_radar_new(const char *spec_json);
void        wickra_radar_free(WickraRadar *handle);
int32_t     wickra_radar_command(WickraRadar *handle,
                                const char *cmd_json,
                                char *out, size_t cap);
const char *wickra_radar_version(void);
```

- **`wickra_radar_new`** builds a radar from a spec JSON (`""` or `"{}"` for an
  empty handle whose spec is set later). Returns `NULL` if the argument is null,
  not UTF-8, or not a valid spec.
- **`wickra_radar_free`** destroys a handle (null is a no-op).
- **`wickra_radar_command`** applies a command JSON and writes the response JSON
  into the caller's buffer using a length-out protocol (below).
- **`wickra_radar_version`** returns a static, NUL-terminated version string (do
  not free).

### Command / response protocol

Everything after construction goes through `wickra_radar_command`. Commands are
JSON objects with a `"cmd"` field: `set_spec`, `feed`, `feed_batch`, `scan`,
`alerts`, `reset`, `version`. Responses are JSON, e.g. a `RadarReport` for
`scan`/`alerts`, `{"version":...}` for `version`, or `{"ok":true}` for a
mutation. A bad spec or unknown command comes back in-band as
`{"ok":false,"error":...}`.

The response is returned via a caller-owned buffer with a length-out protocol —
the callee never allocates memory the caller must free:

1. Call with `out = NULL`, `cap = 0` to learn the response length `len`
   (excluding the terminating NUL).
2. Allocate `len + 1` bytes and call again; the response plus a NUL is written.

Whenever `len < cap`, the response is written on that call, so a
sufficiently-large buffer needs only one call.

A mutating command (`feed`, `feed_batch`, `set_spec`, `reset`) is executed
exactly once across those calls: the handle caches the response it has computed
but not yet delivered, and a repeated call with the same command bytes reuses
it instead of re-executing. Once the response has been written to a buffer the
cache is cleared, so the next identical command executes freshly. The two-call
idiom therefore never feeds an event twice.

Return codes:

| Return   | Meaning                                             |
|----------|-----------------------------------------------------|
| `>= 0`   | Response length in bytes (excluding the NUL).       |
| `-1`     | A required pointer (`handle` or `cmd_json`) is null. |
| `-2`     | `cmd_json` is not valid UTF-8.                       |
| `-3`     | A panic was caught at the boundary.                 |

Domain errors (a bad spec, an unknown command) are **not** negative — they come
back in-band as `{"ok":false,"error":...}` JSON in the buffer.

### C++

`include/wickra_radar.hpp` is a header-only C++17 hull over the same four
functions: `wickra::Radar` owns and frees the handle, `command` runs the
length-out protocol for you, and a negative return becomes a
`wickra::RadarError`. In-band refusals (`{"ok":false,...}`) are returned as
strings, not thrown. `examples/c/scan.cpp` builds against it.

### Header generation

`include/wickra_radar.h` is generated with [cbindgen] and committed; CI fails if
it drifts from the source. Regenerate after changing the ABI:

```sh
cbindgen --config cbindgen.toml --crate wickra-radar-c --output include/wickra_radar.h
```

## Benchmark

Every binding forwards to the same data-driven Rust core, so what this one adds is
the call overhead of the C ABI itself, not a different result. The core's throughput is
measured by the repository's benchmark suite and the nightly `bench.yml` run; the
numbers, the machine and how to reproduce them are in the repository
[BENCHMARKS.md](https://github.com/wickra-lib/wickra-radar/blob/main/BENCHMARKS.md).

## Documentation

The full guide, the spec reference and the API documentation live in the main
repository and the documentation site:

- **Repository:** <https://github.com/wickra-lib/wickra-radar>
- **Docs** (guides, spec reference, cookbook): <https://radar.wickra.org>
- **Runnable example:** [`examples/c/`](https://github.com/wickra-lib/wickra-radar/tree/main/examples/c)

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

[cbindgen]: https://github.com/mozilla/cbindgen
