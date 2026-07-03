# Wickra Radar — Node.js

Node.js bindings for
[`radar-core`](https://github.com/wickra-lib/wickra-radar), built with
[napi-rs]. The surface mirrors every other Wickra binding: build a `Radar` from
a spec JSON, drive it with command JSONs, and read back the report.

## Install

```sh
npm install wickra-radar
```

The right prebuilt native binary is pulled in automatically as an optional
dependency for your platform.

## Usage

```js
const { Radar } = require("wickra-radar");

const spec = JSON.stringify({
  symbols: ["AAA"],
  signals: [{ kind: "funding_flip", params: [0.0005] }],
  threshold: 0.0,
});

const radar = new Radar(spec);
const report = JSON.parse(radar.command(JSON.stringify({ cmd: "scan", events: {
  AAA: [
    { kind: "derivatives", ts: 1, open_interest: 1.0, funding_rate: 0.0003, mark_price: 50.0 },
    { kind: "derivatives", ts: 2, open_interest: 1.0, funding_rate: -0.0004, mark_price: 50.0 },
  ],
}})));
console.log(report.scanned, report.alerts[0].symbol); // 1 AAA
```

## Surface

- **`new Radar(spec_json)`** builds a radar from a spec JSON (`""` or `"{}"` for
  an empty handle whose spec is set later). Throws on a malformed spec.
- **`radar.command(cmd_json)`** applies a command JSON (`set_spec`, `feed`,
  `feed_batch`, `scan`, `alerts`, `reset`, `version`) and returns the response
  JSON — a `RadarReport` for `scan`/`alerts`. Domain errors come back in-band as
  `{"ok":false,"error":...}`.
- **`radar.version()` / `version()`** return the library version.

## Build from source

```sh
npm install
npm run build
npm test
```

[napi-rs]: https://napi.rs
