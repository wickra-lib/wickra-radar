# Wickra Radar — Go

Go bindings for the `wickra-radar` data-driven core over its C ABI hub. Build a
`Radar` from a spec JSON, drive it with command JSON, read back the report — the
same protocol as every other binding.

## Install

```sh
go get github.com/wickra-lib/wickra-radar-go
```

The binding is cgo over the C ABI: it needs the prebuilt native library staged
under `lib/<goos>_<goarch>/` and the header under `include/` (both shipped in the
release module).

## Usage

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

## API

| Function | Description |
|----------|-------------|
| `New(specJSON string) (*Radar, error)` | Build a radar from a spec JSON (error on an invalid spec). |
| `(*Radar).Command(cmdJSON string) (string, error)` | Apply a command JSON, return the response JSON. |
| `(*Radar).Close()` | Free the handle (a finalizer also frees it). |
| `Version() string` | The library version. |

Domain errors (a bad spec, an unknown command) come back in-band as
`{"ok":false,"error":...}` JSON, not as a Go error.

## License

`MIT OR Apache-2.0`.
