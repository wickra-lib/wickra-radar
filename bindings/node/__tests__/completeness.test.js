"use strict";

// Parity guard: the Node binding must expose the full public surface of the
// radar, so an export dropped in a refactor fails loudly here (mirrors the
// completeness check in the main wickra repo).

const { test } = require("node:test");
const assert = require("node:assert");
const wickra = require("../index.js");

// The napi-rs loader adds its own diagnostics to the module object
// (`__napiBindingTarget` since @napi-rs/cli 3.10); they belong to the loader,
// not to the binding's surface, and are left out of the comparison.
const exported = () =>
  Object.keys(wickra)
    .filter((name) => !name.startsWith("__"))
    .sort();

test("module exposes Radar and version", () => {
  assert.strictEqual(typeof wickra.Radar, "function");
  assert.strictEqual(typeof wickra.version, "function");
});

test("Radar exposes command and version", () => {
  for (const name of ["command", "version"]) {
    assert.strictEqual(
      typeof wickra.Radar.prototype[name],
      "function",
      `Radar is missing ${name}`,
    );
  }
});

// Strict parity: pin the exact public surface so a stray export (or a dropped
// one) fails here, matching the exact-surface guards in the Python and R
// bindings.
test("module surface is exactly {Radar, version}", () => {
  assert.deepStrictEqual(exported(), ["Radar", "version"]);
});

test("Radar surface is exactly {command, version}", () => {
  const methods = Object.getOwnPropertyNames(wickra.Radar.prototype)
    .filter((name) => name !== "constructor")
    .sort();
  assert.deepStrictEqual(methods, ["command", "version"]);
});
