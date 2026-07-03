"use strict";

const { test } = require("node:test");
const assert = require("node:assert");
const { Radar, version } = require("../index.js");

const SPEC = JSON.stringify({
  symbols: ["AAA"],
  signals: [{ kind: "funding_flip", params: [0.0005] }],
  threshold: 0.0,
});

const EVENTS = {
  AAA: [
    { kind: "derivatives", ts: 1, open_interest: 1.0, funding_rate: 0.0003, mark_price: 50.0 },
    { kind: "derivatives", ts: 2, open_interest: 1.0, funding_rate: -0.0004, mark_price: 50.0 },
  ],
};

test("scan roundtrip returns the report", () => {
  const radar = new Radar(SPEC);
  const report = JSON.parse(radar.command(JSON.stringify({ cmd: "scan", events: EVENTS })));
  assert.strictEqual(report.scanned, 1);
  assert.strictEqual(report.alerts[0].symbol, "AAA");
  assert.ok(Math.abs(report.alerts[0].severity - 1.0) < 1e-9);
});

test("version matches the module-level function", () => {
  const radar = new Radar(SPEC);
  assert.strictEqual(radar.version(), version());
});

test("a malformed spec throws", () => {
  assert.throws(() => new Radar("not json"));
});
