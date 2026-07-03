# Roadmap

`wickra-radar` is built out in phases, mirroring the proven structure of the
Wickra exchange, backtester, terminal, screener and X-ray repos. Each phase lands
as reviewed, CI-green pull requests. Status below is updated as phases complete.

## Phases

0. **Scaffold** — workspace, governance, supply-chain config, `.github`
   scaffolding. *In progress.*
1. **`radar-core`** — the `RadarSpec`, the perp-universe fold (open interest,
   funding, order-book and liquidation events), the five signals, the weighted
   aggregation into a `RadarAlert`, and both modes (batch `scan` + streaming
   `feed`), with near-total coverage via inline tests.
2. **`radar-cli`** — the reference `wickra-radar` binary: load a spec and an event
   stream, scan the universe, render the alert as text or JSON. An optional
   `radar-tui` reference view renders the same alert in a terminal.
3. **Bindings** — the C ABI hub first, then native Python, Node and WASM, then C,
   C++, C#, Go, Java and R over the hub; each exposes the `Radar` handle +
   `command` + `version`, with a completeness guard.
4. **Golden corpus** — a fixed deterministic perp event universe and canonical
   specs whose blessed alerts are the byte-exact, cross-language parity corpus.
5. **Test rigor** — conformance, golden, `streaming == batch`, property and fuzz
   tests, and a criterion benchmark suite.
6. **ABI harness + examples** — cbindgen header sync-check and one runnable
   example per language, with a C/C++ CMake harness.
7. **CI/CD** — the full workflow matrix (all languages), OpenSSF Scorecard, Best
   Practices, link check, and the release workflow.
8. **README, badges, docs** — the banner + badge treatment and the docs guides.

## Beyond 1.0

- Additional signals and richer per-signal parameters as the corpus grows.
- A live perp universe streamed from an exchange feed, still read-only.

## Non-goals

- **Indicator code in this repository.** Indicators come from the `wickra-core`
  registry; radar composes them, it does not reimplement them.
- **Closures or renderer commands as core output.** A signal is a serde
  `kind` + `params` + `weight` and an alert is a serde `RadarAlert` data-model,
  never draw calls or code, so it crosses the C ABI and WASM unchanged and every
  consumer computes the same alert.
- **A hosted service or stored credentials.** radar runs locally; it holds no
  order-secret material and places no orders.
