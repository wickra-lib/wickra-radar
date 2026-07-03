// A runnable Node.js example: scan a perp universe through the binding.
//
//   ( cd bindings/node && npm install && npm run build )
//   ( cd examples/node && npm install && node scan.js )

"use strict";

const { Radar, version } = require("wickra-radar");

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

const radar = new Radar(SPEC);
const response = radar.command(JSON.stringify({ cmd: "scan", events: EVENTS }));
const report = JSON.parse(response);

console.log("wickra-radar", version());
console.log(response);
console.log(`  alerts: ${report.alerts.length}`);
