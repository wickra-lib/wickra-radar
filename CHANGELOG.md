# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.3] - 2026-11-27

A release of the build and release pipeline; the library code is unchanged.

### Changed

- **The family pins follow the owners' releases.** `wickra-exchange` =0.2.2 ->
  =0.2.3 -- the exact pins this repository keeps on its siblings move to the
  versions those repositories release in the same train, and every tracked
  lockfile follows.
- **The Maven sources jar no longer carries the native libraries.** The release
  workflow stages every platform's library into the resources before the
  deploy, and the sources jar packed them a second time. A release's size on
  Maven Central roughly halves (measured on wickra-impact 0.2.2: the sources jar
  drops from 3.6 MB to 7 KB, the jar is unchanged).
- **The GitHub Release no longer waits on Maven Central.** Central can refuse a
  release over its monthly publishing limits after every other registry has
  published it, and the release page then never appeared. It now does, and the
  jar is attested in a job of its own once Central has taken it.
- **PyPI uploads go through PyPA's publishing action** instead of
  `maturin upload`, which maturin has deprecated. maturin still builds every
  wheel; token, files and the skip of versions already on PyPI are unchanged.
- **CI retries Maven Central's throttling** (429 and 5xx, eight times with a
  back-off of up to five minutes), and the link check skips r-universe's pages,
  which answer every non-browser client with a Cloudflare challenge.
- **Draft pull requests start no CI.** Every job of the workflows that run
  on pull requests skips a draft, and marking it ready for review starts
  the full run.
- **CI caches the Maven repository with restore-keys.** setup-java's own
  cache keyed on the pom alone, so every version bump started empty and
  fetched every plugin from Maven Central again; in a release train that ran
  into Central's 429 throttling. A changed pom now restores the previous
  cache and downloads only what is new.
- **A draft pull request cannot be merged by accident.** The ci-ok gate
  now runs on a draft and fails there; skipped, it counted as passed and let
  auto-merge merge a draft the moment it was marked ready, before its CI ran.

## [0.2.2] - 2026-11-26

A maintenance release: it follows wickra-core 2.2.0, builds on Rust 1.100 and
Go 1.27, and ships the refreshed dependency tree.

### Changed

- **Built on wickra-core 2.2.0** (from 2.1.0). The lock takes the indicator
  core's release; the caret requirement already admitted it. Saturated `Rsi` and
  `StochRsi` values are now exactly 100 (and 0), and `TtmSqueeze` detrends its
  momentum per bar, as the reference does, with a warmup of `2 * period - 1`;
  anything here that runs those indicators can report different values with it.
- **The family pins follow the owners' releases.** `wickra-exchange` =0.2.1 ->
  =0.2.2 -- the exact pins this repository keeps on its siblings move to the
  versions those repositories release in the same train, and every tracked
  lockfile follows.
- **Third-party dependencies refreshed.** `Cargo.lock` takes 99 crates at their
  latest releases within the declared ranges. The npm and Python dev locks are
  re-resolved the same way. The refresh itself changes no manifest.
- **Dependabot's monthly updates** (GitHub Actions, Maven, npm): 4 pull
  requests, each checked against the versions the rest of the family holds.
- **CI and release workflow:** point osv-scanner at the Python lockfiles it was
  silently skipping; install the Node binding with npm ci now that the lock
  names every platform package; use setup-java's *-env-var inputs.

### Fixed

- **Builds on Rust 1.100.** Its `clippy::used_underscore_items` fires on the
  code `#[pymodule]` generates for an underscore-named module function. The
  function has a plain name now and `#[pyo3(name = "_...")]` keeps the extension
  module's import name unchanged.
- **The Go binding links on Windows with Go 1.27,** which rejects
  `-l:<name>.dll` in `#cgo LDFLAGS` as an invalid flag. It is `-l<name>` now;
  the MinGW linker resolves that to the same DLL.

### Security

- **url 2.5.9** in the lockfile, for RUSTSEC-2026-0592: a stack overflow in
  `Url::origin()` on nested `blob:` URLs.
- **rustix 1.1.6** in the lockfile, for RUSTSEC-2026-0486: xattr functions
  marked unwritten bytes as initialised when given a zero-length buffer.
- **quinn-proto 0.11.19** in the lockfile, for RUSTSEC-2026-0523 and
  RUSTSEC-2026-0524: a `MAX_STREAM_DATA` frame could open streams beyond the
  limit, and a long Retry token could panic or exhaust memory in a QUIC client.
- **quinn-udp 0.5.16** in the lockfile, for RUSTSEC-2026-0538: the receive loop
  could spin after a truncated datagram.

## [0.2.1] - 2026-10-28

A maintenance release: the radar, its signals and its bindings are unchanged.
It ships the security fixes in the dependency tree.

### Changed

- **Built on wickra-core 2.1.0.** The lock takes the indicator core's minor
  release, reached through the family crates; the `2.0` requirement already
  admitted it. It adds `RunningDrawdown` and updates `MaxDrawdown` in amortised
  O(1) with the same values; nothing here computes differently.
- **The family pins follow the owners' releases.** `wickra-exchange` =0.2.0 ->
  =0.2.1 -- the exact pins this repository keeps on its siblings move to the
  versions those repositories release in the same train, and every tracked
  lockfile follows.
- **MSRV 1.87** (was 1.86) for the workspace; the Node binding stays at 1.88.
  wickra-exchange reaches `crypto-bigint` and `ctutils` through `p256`, and the
  releases that fix RUSTSEC-2026-0387 and RUSTSEC-2026-0385 require Rust 1.87.
  Every earlier crypto-bigint 0.7 release is yanked, so a fresh resolve could no
  longer build on 1.86.
- **`wickra-core` is no longer declared in the workspace.** No crate used the
  declaration: no signal resolves an indicator yet, and the core reaches the
  graph only through wickra-exchange. The lockfile does not change.
- **napi 3.15.0 with napi-derive 3.6.13** for the Node binding, moved together
  with napi-derive-backend 6.1.6, napi-build 2.6.0 and napi-sys 3.4.1; the pair
  Dependabot proposed on its own did not compile. `@napi-rs/cli` 3.10.8 builds
  it. All five keep the binding's Rust 1.88 floor.
- **pyo3 0.30.1** for the Python binding, and serde_json 1.0.154. pyo3 0.30
  drops Python 3.8, which the wheels never supported: they still need Python 3.9
  or later.
- **central-publishing-maven-plugin 0.12.0** publishes the Java binding.
- **uv 0.13.5** for the lockfile bootstrap in `scripts/update-lockfiles.sh`.
- **The documentation describes Radar as it is.** The README banners,
  `ARCHITECTURE.md`, `CITATION.cff` and the workspace manifest claimed Radar ran
  over 514 streaming indicators; its five signals are window reductions of the
  raw perpetual-futures event streams, and `IndicatorSet` ships no resolver
  yet. They now name what the radar scans.

### Security

- **crypto-bigint 0.7.9, ctutils 0.4.6 and tokio 1.53.3** in the lockfile, for
  RUSTSEC-2026-0387, RUSTSEC-2026-0385 and RUSTSEC-2026-0386.
- **rustls 0.23.46** in the lockfile, for RUSTSEC-2026-0346 to RUSTSEC-2026-0349:
  TLS secrets in the `Debug` output of client session values, application data
  sent to the outer server after an ECH rejection, Brotli certificate
  decompression allocating before authentication, and a ServerHello downgrade to
  classical key exchange.
- **rustls-webpki 0.103.16 and webpki-roots 1.0.10** in the lockfile, for
  RUSTSEC-2026-0382 and RUSTSEC-2026-0381: certificates chaining through the
  distrusted emSign intermediates are refused.

## [0.2.0] - 2026-10-06

A follow-up release on the wickra 2.0 family.

### Changed

- **Built on wickra 2.0 and wickra-exchange 0.2.0.** `wickra-core` moves from
  1.0 to 2.0, the formula-audit release of the indicator core; the exact pin on
  `wickra-exchange` moves from =0.1.8 to =0.2.0; every tracked lockfile
  follows. Indicators the audit corrected return the values of their published
  definitions; wickra's changelog lists them, with the warmup changes and the
  new defaults.

## [0.1.4] - 2026-09-24

A follow-up release: the scanner and its bindings are unchanged. It pins
wickra-exchange 0.1.8, the release that makes exchange's R package build on
r-universe's WebAssembly target and its release pipeline re-runnable.

### Changed

- **Built on wickra-exchange 0.1.8.** The exact pin on `wickra-exchange` moves
  from =0.1.7 to =0.1.8, and every tracked lockfile follows. Nothing in
  exchange's Rust API changed between the two; 0.1.8 fixes its R package's
  WebAssembly build and its Maven Central step.

## [0.1.3] - 2026-09-23

A maintenance release: the scanner and its bindings are unchanged. It publishes
the refreshed dependency tree and toolchain pins.

### Added

- **The Node binding reports which artifact it loaded.** The loader generated
  by `@napi-rs/cli` 3.10.4 exports `__napiBindingTarget` -- `'native'` for the
  native addon, otherwise the WASI flavor it resolved -- and follows a
  `NAPI_RS_NATIVE_LIBRARY_PATH` override to a WASI loader instead of
  misreporting it as native. Typed in `index.d.ts`.

### Changed

- **Built on wickra-core 1.0.6.** The lock takes the indicator core's latest
  release; the `1.0` requirement already admitted it.
- **The family pins follow the owners' releases.** `wickra-exchange` =0.1.6 ->
  =0.1.7 -- the exact pins this repository keeps on its siblings move to the
  versions those repositories release in the same train, and every tracked
  lockfile follows.
- **Third-party dependencies refreshed.** `Cargo.lock` takes 46 crates to their
  newest versions compatible with the Rust floor (the lock now resolves
  MSRV-aware, see below), run across the family in one pass so every repository
  resolves the same day's versions. The refresh itself changes no manifest.
- **The lockfile resolves for the Rust floor.** `.cargo/config.toml` sets
  `incompatible-rust-versions = "fallback"`, so `cargo update` takes the newest
  version the workspace's `rust-version` can build rather than the newest
  release -- the setting compile, copilot and shazam already carried, now
  family-wide. Without it, a routine refresh elsewhere in the family raised the
  icu crates to 2.3.0, which declares Rust 1.88, above a 1.86 floor. Re-resolved
  under it, the lock steps back to the newest versions the floor can build for
  `icu_collections`, `icu_locale_core`, `icu_normalizer`, `icu_normalizer_data`,
  `icu_properties`, `icu_properties_data`, `icu_provider`, `wasip2`,
  `wit-bindgen`.
- **`@napi-rs/cli` 3.10.4** for the Node binding, the family's line.
- **uv 0.12.18** for the lockfile bootstrap in `scripts/update-lockfiles.sh`,
  with all four platform checksums moved together.
- **The README's static badges are served by the organization** rather than
  hot-linked from shields.io, so they no longer break when shields is down.

## [0.1.2] - 2026-09-18

### Fixed

- **The Java binding loads the library it ships.** The jar carries the native
  library under `native/<os>-<arch>/` -- the release pipeline stages every
  platform there -- but the loader only ever looked at `-Dnative.lib.dir` and
  the working directory, so a Maven Central consumer got a jar it could not
  load without pointing the JVM at a library it had to build itself. The loader
  now resolves in wickra's order: `-Dnative.lib.dir` when set, the bundled copy
  extracted to a temporary file, every `target/release` or `target/debug` up
  the tree from the working directory and the class's own location, then the
  bare name.

### Changed

- **Family pins follow the owners' releases:** wickra-exchange =0.1.5 -> =0.1.6. No code of this repository changes; the engine it links is the one those releases ship.
- **Every README follows wickra's shape.** A cross-repo scan compared the
  heading skeleton of each README against wickra's and this repository's
  differed throughout. The root README opens as wickra's does (banner, badges,
  the one-liner, the live-demo and ecosystem lines, no separate H1), the
  License section carries wickra's wording and its `### Contribution` clause,
  and the shared sections run in wickra's order. Each binding README is
  `Install`, `Quick start`, `Benchmark`, `Documentation`, `Security`,
  `Disclaimer`, `License` with the product's own surface and protocol notes
  as subsections; the registry pages that render them now say how to report a
  vulnerability and under which licence the package ships.
  `examples/README.md` lists every language the way wickra's does, with the
  commands the CI examples job runs; the per-language example READMEs,
  `fuzz/README.md` and the `## Editing the docs` section of
  `docs/README.md` exist as they do in wickra.

### Changed

- **wickra-exchange 0.1.5.** The pins move to the releases the family is on; the lock follows.
  A cross-repo scan lined the 24 wickra-lib repositories up, and the rest is
  what this one spelled differently: the fuzz job runs the family's pinned
  `nightly-2026-07-01` rather than a rolling nightly, and the example job's
  `dotnet-version` reads `8.0.x`.

### Changed

- **wickra-exchange 0.1.5.** The pins move to the releases the family is on; the lock follows.
  A cross-repo scan lined the 24 wickra-lib repositories up, and the rest is
  what this one spelled differently: the fuzz job runs the family's pinned
  `nightly-2026-07-01` rather than a rolling nightly, and the example job's
  `dotnet-version` reads `8.0.x`.

### Changed

- **uv 0.12.15 for the lockfile script.** `scripts/update-lockfiles.sh`
  bootstraps 0.12.15 (was 0.12.13); the pin and all four release
  checksums move together, taken from the release's `.sha256` files.

## [0.1.1] - 2026-09-14

### Security

- **rustls 0.23.45.** RUSTSEC-2026-0285: rustls accepted TLS 1.3 handshake
  messages sent at the wrong encryption level. The lock moves to the
  patched release; nothing in the code changes.

### Fixed

- **The Go module mirror ships.** The v0.1.0 release built every artefact and
  published crates.io, npm, NuGet and Maven, but the job that assembles
  `wickra-radar-go` failed its own smoke test before pushing: the test came
  from a sibling whose constructor takes no arguments, while `New` here
  parses a spec. It now passes the smallest spec the core accepts, so this
  release is the first one the Go mirror and the GitHub Release carry.

## [0.1.0] - 2026-09-14

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

[Unreleased]: https://github.com/wickra-lib/wickra-radar/compare/v0.2.3...HEAD
[0.2.3]: https://github.com/wickra-lib/wickra-radar/compare/v0.2.2...v0.2.3
[0.2.2]: https://github.com/wickra-lib/wickra-radar/compare/v0.2.1...v0.2.2
[0.2.1]: https://github.com/wickra-lib/wickra-radar/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/wickra-lib/wickra-radar/compare/v0.1.4...v0.2.0
[0.1.4]: https://github.com/wickra-lib/wickra-radar/compare/v0.1.3...v0.1.4
[0.1.3]: https://github.com/wickra-lib/wickra-radar/compare/v0.1.2...v0.1.3
[0.1.2]: https://github.com/wickra-lib/wickra-radar/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/wickra-lib/wickra-radar/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/wickra-lib/wickra-radar/releases/tag/v0.1.0
