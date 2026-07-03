# Threat Model

`wickra-radar` is analysis software. It folds perp market data into early-warning
signals and places no orders, opens no authenticated connections, and holds no
secret key material — its feeds read only public market data. The attack surface
is correspondingly narrow: it is dominated by the parsing of **untrusted input** —
a `RadarSpec` and an event stream supplied by the caller — as it crosses the C ABI
and WASM boundary.

## Assets

- The **`RadarSpec` and event stream** a caller supplies. These are inputs, not
  secrets, but a malformed or hostile one must never crash or corrupt the host.
- The **integrity and determinism** of the `RadarAlert`: the same spec and events
  must always produce the same alert, in every language, and identically between
  the parallel and sequential builds.
- The **host process** embedding a binding. Scanning must not be able to take it
  down (panic across FFI, unbounded allocation) or read memory it should not.

There is intentionally **no secret asset** — no API keys, no credentials, no order
flow.

## Trust boundaries

- **Caller → core.** Everything arriving through `Radar::command` (spec, events,
  command) is untrusted and validated before use.
- **Binding → C ABI hub.** The hub is the one place `unsafe` is allowed. It wraps
  every call in `catch_unwind`, guards null pointers, and uses a length-out
  buffer protocol so no panic or invalid pointer crosses into C / Go / C# / Java
  / R.
- **Exchange feed → events.** The perp derivative, order-book and liquidation
  streams sourced through `wickra-exchange` are public market data; they add a
  network read but no credentials or orders, and their contents are validated
  like any other untrusted input.

## Guarantees the code is held to

- `unsafe_code = "forbid"` workspace-wide; only `bindings/c` re-allows it locally.
- No panic crosses the FFI boundary; errors are returned as JSON, never as an
  abort.
- Parsing is bounded and total — a hostile spec or event stream yields an error,
  not an unbounded allocation or a hang.
- The scan is deterministic: the same events and spec always yield the same
  `RadarAlert`, and because each binding returns the core's response verbatim,
  that alert is byte-identical in every language.

## Out of scope

- Incorrect signal mathematics — a functional bug, handled through normal issues
  and tests, not a vulnerability.
- Vulnerabilities in third-party crates, which are tracked and triaged through
  `deny.toml` and `osv-scanner.toml`.
- Resource exhaustion a caller inflicts on **their own** process by deliberately
  feeding an enormous event stream; the core bounds its own allocations but cannot
  bound the caller's data volume.
