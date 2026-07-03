# Wickra Radar — R

R bindings for the `wickra-radar` data-driven core, over its C ABI hub (`.Call`).
Build a radar from a spec JSON, drive it with command JSON, read back the report
— the same protocol as the CLI and every other binding.

## Usage

```r
library(wickraradar)

spec <- paste0(
  '{"symbols":["AAA"],"signals":[{"kind":"funding_flip",',
  '"params":[0.0005]}],"threshold":0.0}'
)

radar <- wkradar_new(spec)
report <- wkradar_command(radar, paste0(
  '{"cmd":"scan","events":{"AAA":[',
  '{"kind":"derivatives","ts":1,"open_interest":1.0,"funding_rate":0.0003,"mark_price":50.0},',
  '{"kind":"derivatives","ts":2,"open_interest":1.0,"funding_rate":-0.0004,"mark_price":50.0}]}}'
))
cat(report, "\n")
cat(wkradar_version(), "\n")
```

## Build and test from source

The package links the `wickra_radar` C ABI, located out-of-tree via two
environment variables:

```bash
# Build the C ABI shared library first.
cargo build -p wickra-radar-c --release

export WKRADAR_INC="$PWD/bindings/c/include"
export WKRADAR_LIB="$PWD/target/release"
# The loader must also find the shared library at run time:
export LD_LIBRARY_PATH="$WKRADAR_LIB:$LD_LIBRARY_PATH"   # PATH on Windows

R CMD INSTALL bindings/r
Rscript bindings/r/tests/run_tests.R
```

## API

| Function | Description |
|----------|-------------|
| `wkradar_new(spec_json)` | Build a radar from a spec JSON (errors on an invalid spec). |
| `wkradar_command(radar, cmd_json)` | Apply a command JSON, return the response JSON. |
| `wkradar_version()` | The library version. |

Domain errors (a bad spec, an unknown command) come back in-band as
`{"ok":false,"error":...}` JSON.

## License

`MIT OR Apache-2.0`.
