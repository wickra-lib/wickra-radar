# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Fixed

- **A mutating command through the C ABI executed twice.** `wickra_radar_command`
  ran the command on every call, and every consumer of the length-then-fill
  protocol -- Go, C#, Java, R and the C examples -- calls it twice (once for the
  length, once for the bytes). `feed` and `feed_batch` therefore fed each event
  twice through four of the ten bindings, and a report read through them
  disagreed with the same events through Rust, Python, Node or WASM. The handle
  now caches the response it has computed but not yet delivered and reuses it
  for a repeated call with the same command bytes, so a logical command runs
  exactly once however many buffer-sizing retries it takes (the contract gym
  already documents). A C ABI test pins it, and the operating-mode tests below
  fail against the old library.
- **Operating-mode equivalence is tested in every binding.** One-shot `scan`
  over a universe and streaming `feed` / `feed_batch` per symbol followed by
  `alerts` must return the same bytes; the core pinned it in Rust only. Each of
  the nine bindings now checks it across the whole golden corpus, and the C
  suite (`examples/c/golden_test.c`, wired into ctest with a CMake-globbed spec
  list) checks golden parity and both modes without a JSON library.
- **A C++ hull.** `bindings/c/include/wickra_radar.hpp` -- header-only C++17,
  owns the handle, runs the length protocol, turns a negative return into an
  exception -- ships beside the C header and in the release archive;
  `examples/c/scan.cpp` builds against it.
- **The Maven Central publish is idempotent, and waits as long as Central
  takes.** The release workflow skips a version already on Central, the plugin
  waits up to two hours (`waitMaxTime`), and the job has the budget for it;
  the jar it builds is uploaded for the provenance job to attest.
- **Every dependency comes from crates.io, and the sibling pin is exact.**
  `wickra-exchange` was a git dependency, which `cargo publish` refuses; it is
  the registry crate at `=0.1.4`, as the released siblings pin it, and
  `wickra-core` and `wickra-data` follow their 1.0 lines. The unused
  `wickra-backtest-core` git dependency is gone, and `deny.toml` no longer
  allows git sources.
- **The Python 3.9 CI row runs without pytest.** pytest 9.x requires 3.10, so
  that row could only pin 8.4.2, below the fix for GHSA-6w46-j5rx-g56g with no
  backport. The 3.9 lock carries maturin only, and the row runs the same test
  modules through `bindings/python/tests/run_without_pytest.py`; 3.10 and up
  run them under pytest as before.
- **The R package builds the family way.** `configure` downloads the C ABI
  release archive (or builds it from the tag's source on r-universe's
  WebAssembly image), `configure.win` picks the architecture from
  `R.version$arch`, the exported functions carry generated `man/` pages, a
  shipped smoke test runs inside the tarball, `.Rbuildignore` is
  regex-safe, the operating-mode test decodes events with jsonlite (installed
  by CI, declared in `Suggests`), and `DESCRIPTION` states the R floor.
- CI in the family shape: an Examples job that holds every example to the
  version line, a WASM demo page (`examples/wasm/scan.html`) whose module is
  parse-checked, CodeQL over C#, Java and C/C++ with a config that keeps
  generated code out, timeouts on every job, patch-level pins, Dependabot over
  every manifest (the fuzz crate and the Go example included), the five
  repository-check scripts, the detailed issue and PR templates, zizmor's
  `self-repository` policy, `criterion` aliased to CodSpeed, and docs.rs
  metadata on both crates.
- Licence texts travel with every published package (`LICENSES/`, copies in
  each crate and the Python and npm packages); the README states the toolchain
  floors the manifests declare; `SECURITY.md` names the first release.
- **The core crate carried a name the release could not upload.**
  `radar-core` is outside the org's crates.io token scope, which
  creates new crates under the `wickra-` prefix only; `cargo publish` on it
  returns 403 at upload while `--dry-run` passes, and because the publish jobs
  run in parallel the release would have landed on PyPI, npm, NuGet, Maven
  Central and the Go mirror without ever reaching crates.io. The core is now
  `wickra-radar-core`, the shape of every released sibling. The
  directory keeps its name; only the package and the
  `wickra_radar_core` path moved. The same audit ran across the family
  (xray paid for this with its first tag).

### Added

- The `wickra-radar-core` data-driven core: `RadarSpec` (JSON/TOML), the three input
  event kinds (derivatives, order-book, liquidation), the five cascade signals
  (open-interest delta, funding flip, book imbalance, liquidation cluster,
  OI/price divergence), the weighted-mean `severity` aggregation with a
  self-explaining `factors` map, `scan` over a perp universe, and the
  `Radar::command_json` JSON-over-C-ABI protocol. The parallel (rayon) and
  sequential builds are byte-for-byte identical.
- `wickra-radar` CLI: scan a universe from a spec + an event stream
  (`--stdin` / `--events`, `--threshold` / `--limit` overrides, `--format json`
  or a human-readable table).
- Ten-language surface: native Rust, Python (PyO3), Node.js (napi) and WASM
  (wasm-bindgen), plus a C ABI hub (cbindgen) backing C, C++, C#, Go, Java and R.
- Streaming: `feed` / `feed_batch` / `alerts` incremental scanning that returns
  the same bytes as a batch `scan`.
- A deterministic golden corpus (event universe, specs, byte-exact expected
  reports) and cross-language byte-equality tests across every binding.
- Test rigor: conformance, golden, streaming-equals-batch, property-based
  invariants, three cargo-fuzz targets, and the `radar-bench` criterion suite.
- One runnable "scan a universe" example per language and per-language guides
  under `docs/`.
- CI/CD: a multi-OS test matrix across ten languages, CodeQL, OpenSSF Scorecard,
  zizmor, link-check, benchmark and metadata-audit workflows, plus an authored
  (tag-gated) release workflow.
- Repository scaffolding: Cargo workspace, supply-chain configuration
  (`deny.toml`, `osv-scanner.toml`, `lychee.toml`), lint configuration
  (`clippy.toml`), `repo-metadata.toml`, and dual `MIT OR Apache-2.0` licensing.

[Unreleased]: https://github.com/wickra-lib/wickra-radar/commits/main
