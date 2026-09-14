"use strict";
// Operating-mode equivalence through the binding: one-shot `scan` over the
// universe and streaming `feed` / `feed_batch` per symbol followed by `alerts`
// must return the same report bytes for every golden spec. The core pins this
// in Rust (streaming_eq_batch.rs); this checks the boundary the WebAssembly core
// crosses. A missing corpus is a failure, not a skip.
const { test } = require("node:test");
const assert = require("node:assert");
const fs = require("node:fs");
const path = require("node:path");
const { Radar } = require("../pkg-node/wickra_radar_wasm.js");

const GOLDEN = path.resolve(__dirname, "..", "..", "..", "golden");

function scan(spec, events) {
  return new Radar(spec).command(JSON.stringify({ cmd: "scan", events }));
}

function stream(spec, events) {
  const radar = new Radar(spec);
  for (const [symbol, evs] of Object.entries(events)) {
    for (const event of evs) {
      radar.command(JSON.stringify({ cmd: "feed", symbol, event }));
    }
  }
  return radar.command(JSON.stringify({ cmd: "alerts" }));
}

function streamBatched(spec, events) {
  const radar = new Radar(spec);
  for (const [symbol, evs] of Object.entries(events)) {
    radar.command(JSON.stringify({ cmd: "feed_batch", symbol, events: evs }));
  }
  return radar.command(JSON.stringify({ cmd: "alerts" }));
}

test("streaming equals batch for every golden spec", () => {
  const specDir = path.join(GOLDEN, "specs");
  const specs = fs.readdirSync(specDir).filter((f) => f.endsWith(".json"));
  assert.ok(specs.length > 0, "golden corpus not found");
  const events = JSON.parse(fs.readFileSync(path.join(GOLDEN, "events.json"), "utf8"));
  for (const file of specs) {
    const spec = fs.readFileSync(path.join(specDir, file), "utf8");
    const expected = fs.readFileSync(path.join(GOLDEN, "expected", file), "utf8").trim();
    const batch = scan(spec, events);
    assert.strictEqual(batch.trim(), expected, file);
    assert.strictEqual(stream(spec, events), batch, `${file}: feed vs scan`);
    assert.strictEqual(streamBatched(spec, events), batch, `${file}: feed_batch vs scan`);
  }
});
