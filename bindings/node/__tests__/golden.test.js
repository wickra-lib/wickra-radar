"use strict";

// Cross-language golden parity: build the radar from each committed
// `golden/specs/*.json`, run `scan` over the shared `golden/events.json`, and
// assert the report equals `golden/expected/<spec>.json` byte-for-byte. Because
// every binding returns the core's compact `command_json` string verbatim, byte
// equality is the exact cross-language parity check. The fixtures arrive in a
// later phase; until then this test skips cleanly.

const { test } = require("node:test");
const assert = require("node:assert");
const fs = require("node:fs");
const path = require("node:path");
const { Radar } = require("../index.js");

function findGolden() {
  let dir = __dirname;
  for (let i = 0; i < 8; i++) {
    const g = path.join(dir, "golden");
    if (fs.existsSync(path.join(g, "specs"))) {
      return g;
    }
    dir = path.dirname(dir);
  }
  return null;
}

test("golden reports are byte-identical", (t) => {
  const golden = findGolden();
  if (!golden) {
    t.skip("golden fixtures not present yet");
    return;
  }
  const events = fs.readFileSync(path.join(golden, "events.json"), "utf8");
  const specDir = path.join(golden, "specs");
  for (const file of fs.readdirSync(specDir).filter((f) => f.endsWith(".json"))) {
    const spec = fs.readFileSync(path.join(specDir, file), "utf8");
    const expected = fs
      .readFileSync(path.join(golden, "expected", file), "utf8")
      .trim();
    const radar = new Radar(spec);
    const response = radar.command(
      JSON.stringify({ cmd: "scan", events: JSON.parse(events) }),
    );
    assert.strictEqual(response.trim(), expected, `mismatch for ${file}`);
  }
});
