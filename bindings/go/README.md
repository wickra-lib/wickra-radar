<p align="center">
  <a href="https://wickra.org"><img src="https://raw.githubusercontent.com/wickra-lib/.github/main/profile/wickra-banner.webp?v=514" alt="Wickra Radar — the liquidation-cascade early-warning radar for Go" width="100%"></a>
</p>

[![Built on Wickra](https://img.shields.io/badge/built%20on-wickra-3b82f6)](https://github.com/wickra-lib/wickra)
[![CI](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-radar/ci.svg)](https://github.com/wickra-lib/wickra-radar/actions/workflows/ci.yml)
[![codecov](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-radar/codecov.svg)](https://codecov.io/gh/wickra-lib/wickra-radar)
[![Go module](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/go.svg)](https://pkg.go.dev/github.com/wickra-lib/wickra-radar-go)
[![License: MIT OR Apache-2.0](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-radar/license.svg)](https://github.com/wickra-lib/wickra-radar#license)

# Wickra Radar — Go

---

**The liquidation-cascade early-warning radar core for Go, over the Wickra C ABI hub via cgo.**

[Wickra Radar](https://github.com/wickra-lib/wickra-radar) scans a universe of perpetuals for funding, open-interest and liquidation signals and aggregates them into an early-warning report. This package is the Go binding: it consumes the C ABI hub through cgo and exposes the `Radar` handle with the same JSON protocol as every other binding.

## Install

Use the published **`wickra-radar-go`** module, which bundles the prebuilt C ABI library
for every platform, so `go get` + `go build` works with no extra steps (a C
compiler is still required, as the binding uses cgo):

```bash
go get github.com/wickra-lib/wickra-radar-go
```

## Quick start

```go
package main

import (
	"encoding/json"
	"fmt"

	wickra "github.com/wickra-lib/wickra-radar-go"
)

func main() {
	spec := `{"symbols":["AAA"],"signals":[{"kind":"funding_flip","params":[0.0005]}],"threshold":0.0}`
	r, err := wickra.New(spec)
	if err != nil {
		panic(err)
	}
	defer r.Close()

	events := map[string]any{"AAA": []map[string]any{
		{"kind": "derivatives", "ts": 1, "open_interest": 1.0, "funding_rate": 0.0003, "mark_price": 50.0},
		{"kind": "derivatives", "ts": 2, "open_interest": 1.0, "funding_rate": -0.0004, "mark_price": 50.0},
	}}
	scan, _ := json.Marshal(map[string]any{"cmd": "scan", "events": events})

	report, _ := r.Command(string(scan))
	fmt.Println(report)
	fmt.Println(wickra.Version())
}
```


`wickra-radar-go` is generated from this directory by the release pipeline: it mirrors the
Go sources, the vendored C ABI header (`include/wickra_radar.h`) and the prebuilt
libraries under `lib/<goos>_<goarch>/`. On Windows the DLL must be discoverable at
run time (next to the executable or on `PATH`).

## Building from this repository (contributors)

This `bindings/go` directory is the development source. To build it directly,
compile the C ABI hub and stage the library into the per-platform directory cgo
links against:

```bash
cargo build -p wickra-radar-c --release
mkdir -p bindings/go/lib/linux_amd64                    # match your GOOS_GOARCH
cp target/release/libwickra_radar.so    bindings/go/lib/linux_amd64/   # Linux
cp target/release/libwickra_radar.dylib bindings/go/lib/darwin_arm64/  # macOS (arm64)
cp target/release/wickra_radar.dll      bindings/go/lib/windows_amd64/ # Windows
```

Then, with the library on the loader path, run `go test ./...` from this directory.

## License

Dual-licensed under [MIT](https://github.com/wickra-lib/wickra-radar/blob/main/LICENSE-MIT)
or [Apache-2.0](https://github.com/wickra-lib/wickra-radar/blob/main/LICENSE-APACHE), at your option.
