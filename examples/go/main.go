// A runnable Go example: scan a perp universe through the binding.
//
//	cargo build --release -p wickra-radar-c
//	# stage the library under bindings/go/lib/<goos>_<goarch>/ (CI does this)
//	cd examples/go && go run .
package main

import (
	"fmt"

	wickra "github.com/wickra-lib/wickra-radar-go"
)

const spec = `{"symbols":["AAA"],"signals":[` +
	`{"kind":"funding_flip","params":[0.0005]}],"threshold":0.0}`

const scan = `{"cmd":"scan","events":{"AAA":[` +
	`{"kind":"derivatives","ts":1,"open_interest":1.0,"funding_rate":0.0003,"mark_price":50.0},` +
	`{"kind":"derivatives","ts":2,"open_interest":1.0,"funding_rate":-0.0004,"mark_price":50.0}]}}`

func main() {
	radar, err := wickra.New(spec)
	if err != nil {
		panic(err)
	}
	defer radar.Close()

	report, err := radar.Command(scan)
	if err != nil {
		panic(err)
	}

	fmt.Println("wickra-radar", wickra.Version())
	fmt.Println(report)
}
