# Wickra Radar examples — R

Runnable R examples for the [Wickra Radar R binding](../../bindings/r). The package compiles a thin
`.Call` glue layer against the C ABI library, so build the library and install
the package first (the CI examples job does exactly this):

```bash
cargo build -p wickra-radar-c --release
R CMD INSTALL bindings/r
```

## Run

```bash
Rscript examples/r/scan.R
```

## The examples

| Example | What it does |
|---------|--------------|
| `scan.R` | A runnable R example: scan a perp universe through the binding. |
