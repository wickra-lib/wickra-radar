# Contributing to wickra-radar

Thanks for your interest. Issues, bug reports, ideas and pull requests are all
welcome at <https://github.com/wickra-lib/wickra-radar>. For larger changes,
open an issue first so we can agree on the approach.

## Orientation

- The core — the `RadarSpec`, the perp-universe fold (open interest, funding,
  order-book and liquidation events) scored per signal and aggregated with
  weights into a `RadarAlert` — lives in `crates/radar-core`. Alerts are
  **data, not code**: a serde data-model, so the same alert crosses the C ABI
  and WASM unchanged, and stays byte-identical between the parallel (rayon) and
  sequential builds.
- The reference consumer is `crates/radar-cli` (the `wickra-radar` binary); an
  optional `crates/radar-tui` reference view renders the same alerts in a
  terminal.
- Every language binding lives under `bindings/<lang>/` and exposes the same
  data-driven surface: a `Radar` handle plus `command(json) -> json` and
  `version`. Bindings must preserve the **golden-parity invariant**: given the
  spec + events in `golden/{specs,data}/`, the same command produces the
  byte-identical alert in `golden/expected/`.

## The dev loop

Every change runs green locally before a commit:

```bash
cargo fmt --all
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo clippy --workspace --all-targets --no-default-features -- -D warnings   # WASM path (no rayon)
cargo test --workspace --all-features
cargo test -p radar-core --no-default-features                                # sequential == parallel
cargo deny check
```

`cargo fmt --all` and the `clippy -D warnings` gate are enforced in CI on three
operating systems, across both the default (rayon `parallel`) and
`--no-default-features` (sequential / WASM) feature sets — a scan must produce a
byte-identical alert either way.

## Conventions

- **Commits are signed** and follow Conventional Commits (`feat:`, `fix:`,
  `chore:`, `docs:`…). One logical change per commit. Open a PR against `main`;
  do not push to `main` directly.
- **All public artifacts are in English** — code, comments, commit messages, PR
  titles and bodies, issues and docs.
- **No secrets, ever** — not in code, tests, fixtures, logs, issues or PRs. The
  live perp feeds read only public market data and never use real keys in tests.
- **Production code only** — no mocks outside `#[cfg(test)]`, no TODO stubs, and
  no defensive branches that can never run (they fail coverage).

## Adding a signal

Signals are a serde enum, so extending the radar means adding a variant, not a
closure. A new signal is added to `crates/radar-core/src/signal.rs` and handled
in the weighted aggregation in `src/aggregate.rs`, with a serde round-trip test
and a golden fixture. Indicators themselves come from the
[Wickra](https://github.com/wickra-lib/wickra) core registry by name and
parameters — no indicator code lives here. See the guides under `docs/`.

## Developer Certificate of Origin

Contributions are accepted under the [DCO](DCO); sign off your commits with
`git commit -s`. By contributing you agree your work is dual-licensed under
`MIT OR Apache-2.0`.
