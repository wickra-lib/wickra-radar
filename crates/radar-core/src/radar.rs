//! The `Radar` handle and the JSON `command_json` boundary — the single FFI
//! entry point exposed in every language binding (§6.9).

use std::collections::BTreeMap;

use serde_json::{json, Value};

use crate::error::{Error, Result};
use crate::event::Event;
use crate::scan::{report_from_states, scan, RadarReport};
use crate::spec::RadarSpec;
use crate::universe::Universe;

/// A radar over a fixed spec, holding a streaming universe of fed events.
pub struct Radar {
    spec: RadarSpec,
    universe: Universe,
}

impl Radar {
    /// Build a radar from a spec JSON string (validated).
    ///
    /// # Errors
    /// Returns an error if the spec fails to parse or validate.
    pub fn new(spec_json: &str) -> Result<Self> {
        let spec = RadarSpec::from_json(spec_json)?;
        Ok(Self {
            spec,
            universe: Universe::new(),
        })
    }

    /// The crate version string.
    #[must_use]
    pub fn version() -> &'static str {
        env!("CARGO_PKG_VERSION")
    }

    /// Replace the spec and clear the streaming universe (state sizing changes
    /// with the spec).
    pub fn set_spec(&mut self, spec: RadarSpec) {
        self.spec = spec;
        self.universe = Universe::new();
    }

    /// Feed one event for a symbol into the streaming universe.
    ///
    /// # Errors
    /// Currently infallible; returns `Result` for binding-surface consistency.
    pub fn feed(&mut self, symbol: &str, event: &Event) -> Result<()> {
        self.universe.ensure(symbol, &self.spec);
        self.universe.fold(symbol, event);
        Ok(())
    }

    /// Aggregate the current streaming universe into a report (same filter, sort
    /// and limit as [`scan`]).
    ///
    /// # Errors
    /// Returns an error if the spec fails validation.
    pub fn alerts(&self) -> Result<RadarReport> {
        self.spec.validate()?;
        Ok(report_from_states(
            self.universe
                .symbols
                .iter()
                .map(|(sym, state)| (sym.as_str(), state)),
            &self.spec,
            self.universe.scanned(),
        ))
    }

    /// Clear the streaming universe, keeping the spec.
    pub fn reset(&mut self) {
        self.universe = Universe::new();
    }

    /// The single JSON-in / JSON-out command boundary. Never returns `Err` for a
    /// well-formed call: internal errors come back as `{"ok":false,"error":...}`.
    ///
    /// # Errors
    /// Reserved for a serialization failure of the error envelope, which cannot
    /// occur for the fixed shapes used here.
    pub fn command_json(&mut self, cmd_json: &str) -> Result<String> {
        Ok(self
            .dispatch(cmd_json)
            .unwrap_or_else(|e| error_json(&e.to_string())))
    }

    fn dispatch(&mut self, cmd_json: &str) -> Result<String> {
        let value: Value = serde_json::from_str(cmd_json)?;
        let cmd = value
            .get("cmd")
            .and_then(Value::as_str)
            .ok_or_else(|| Error::BadSpec("missing \"cmd\"".into()))?;
        match cmd {
            "set_spec" => {
                let spec: RadarSpec = serde_json::from_value(field(&value, "spec")?)?;
                spec.validate()?;
                self.set_spec(spec);
                Ok(ok_json())
            }
            "feed" => {
                let symbol = str_field(&value, "symbol")?.to_string();
                let event: Event = serde_json::from_value(field(&value, "event")?)?;
                self.feed(&symbol, &event)?;
                Ok(ok_json())
            }
            "feed_batch" => {
                let symbol = str_field(&value, "symbol")?.to_string();
                let events: Vec<Event> = serde_json::from_value(field(&value, "events")?)?;
                for event in &events {
                    self.feed(&symbol, event)?;
                }
                Ok(ok_json())
            }
            "scan" => {
                let events: BTreeMap<String, Vec<Event>> =
                    serde_json::from_value(field(&value, "events")?)?;
                Ok(serde_json::to_string(&scan(&events, &self.spec)?)?)
            }
            "alerts" => Ok(serde_json::to_string(&self.alerts()?)?),
            "reset" => {
                self.reset();
                Ok(ok_json())
            }
            "version" => Ok(json!({ "version": Self::version() }).to_string()),
            other => Err(Error::BadSpec(format!("unknown cmd: {other}"))),
        }
    }
}

/// Clone a named field out of the envelope, erroring if absent.
fn field(value: &Value, name: &str) -> Result<Value> {
    value
        .get(name)
        .cloned()
        .ok_or_else(|| Error::BadSpec(format!("missing \"{name}\"")))
}

/// Read a named string field out of the envelope.
fn str_field<'a>(value: &'a Value, name: &str) -> Result<&'a str> {
    value
        .get(name)
        .and_then(Value::as_str)
        .ok_or_else(|| Error::BadSpec(format!("missing string \"{name}\"")))
}

fn ok_json() -> String {
    json!({ "ok": true }).to_string()
}

fn error_json(message: &str) -> String {
    json!({ "ok": false, "error": message }).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SPEC: &str = r#"{"symbols":["AAA","BBB"],"signals":[{"kind":"funding_flip","params":[0.0005],"weight":1.0}],"threshold":0.0}"#;

    fn deriv_json(ts: i64, funding: f64) -> String {
        format!(
            r#"{{"kind":"derivatives","ts":{ts},"open_interest":1.0,"funding_rate":{funding},"mark_price":50.0}}"#
        )
    }

    #[test]
    fn version_command() {
        let mut r = Radar::new(SPEC).unwrap();
        let out = r.command_json(r#"{"cmd":"version"}"#).unwrap();
        assert_eq!(out, format!(r#"{{"version":"{}"}}"#, Radar::version()));
    }

    #[test]
    fn unknown_cmd_returns_error_json() {
        let mut r = Radar::new(SPEC).unwrap();
        let out = r.command_json(r#"{"cmd":"nope"}"#).unwrap();
        assert!(out.contains(r#""ok":false"#));
    }

    #[test]
    fn malformed_json_returns_error_json() {
        let mut r = Radar::new(SPEC).unwrap();
        let out = r.command_json("not json").unwrap();
        assert!(out.contains(r#""ok":false"#));
    }

    #[test]
    fn feed_then_alerts_equals_scan() {
        let mut streaming = Radar::new(SPEC).unwrap();
        for (sym, ts, funding) in [
            ("AAA", 1, 0.0003),
            ("AAA", 2, -0.0004),
            ("BBB", 1, 0.0002),
            ("BBB", 2, 0.0003),
        ] {
            let cmd = format!(
                r#"{{"cmd":"feed","symbol":"{sym}","event":{}}}"#,
                deriv_json(ts, funding)
            );
            streaming.command_json(&cmd).unwrap();
        }
        let streamed = streaming.command_json(r#"{"cmd":"alerts"}"#).unwrap();

        let mut batch = Radar::new(SPEC).unwrap();
        let scan_cmd = format!(
            r#"{{"cmd":"scan","events":{{"AAA":[{},{}],"BBB":[{},{}]}}}}"#,
            deriv_json(1, 0.0003),
            deriv_json(2, -0.0004),
            deriv_json(1, 0.0002),
            deriv_json(2, 0.0003)
        );
        let scanned = batch.command_json(&scan_cmd).unwrap();

        assert_eq!(streamed, scanned);
    }

    #[test]
    fn feed_batch_matches_individual_feeds() {
        let mut a = Radar::new(SPEC).unwrap();
        for (ts, funding) in [(1, 0.0003), (2, -0.0004)] {
            let cmd = format!(
                r#"{{"cmd":"feed","symbol":"AAA","event":{}}}"#,
                deriv_json(ts, funding)
            );
            a.command_json(&cmd).unwrap();
        }
        let individual = a.command_json(r#"{"cmd":"alerts"}"#).unwrap();

        let mut b = Radar::new(SPEC).unwrap();
        let batch_cmd = format!(
            r#"{{"cmd":"feed_batch","symbol":"AAA","events":[{},{}]}}"#,
            deriv_json(1, 0.0003),
            deriv_json(2, -0.0004)
        );
        b.command_json(&batch_cmd).unwrap();
        let batched = b.command_json(r#"{"cmd":"alerts"}"#).unwrap();

        assert_eq!(individual, batched);
    }

    #[test]
    fn reset_clears_streaming_state() {
        let mut r = Radar::new(SPEC).unwrap();
        let cmd = format!(
            r#"{{"cmd":"feed","symbol":"AAA","event":{}}}"#,
            deriv_json(1, 0.0003)
        );
        r.command_json(&cmd).unwrap();
        r.command_json(r#"{"cmd":"reset"}"#).unwrap();
        let out = r.command_json(r#"{"cmd":"alerts"}"#).unwrap();
        assert!(out.contains(r#""scanned":0"#));
    }

    #[test]
    fn set_spec_command_reconfigures() {
        let mut r = Radar::new(SPEC).unwrap();
        let new_spec = r#"{"cmd":"set_spec","spec":{"symbols":["X"],"signals":[{"kind":"book_imbalance","params":[1.0]}],"threshold":0.0}}"#;
        let out = r.command_json(new_spec).unwrap();
        assert!(out.contains(r#""ok":true"#));
    }

    #[test]
    fn set_spec_with_wrong_arity_returns_error_json() {
        let mut r = Radar::new(SPEC).unwrap();
        // funding_flip takes exactly one parameter; two fails validation in-band.
        let bad =
            r#"{"cmd":"set_spec","spec":{"signals":[{"kind":"funding_flip","params":[0.1,0.2]}]}}"#;
        let out = r.command_json(bad).unwrap();
        assert!(out.contains(r#""ok":false"#));
        assert!(out.contains("parameter"));
    }

    #[test]
    fn missing_field_returns_error_json() {
        let mut r = Radar::new(SPEC).unwrap();
        // `feed` without an "event" field.
        let out = r.command_json(r#"{"cmd":"feed","symbol":"AAA"}"#).unwrap();
        assert!(out.contains(r#""ok":false"#));
    }

    #[test]
    fn new_rejects_a_duplicate_signal_key() {
        // Two identical signals would share a factors key.
        let dup = r#"{"signals":[{"kind":"funding_flip","params":[0.0005]},{"kind":"funding_flip","params":[0.0005]}]}"#;
        assert!(Radar::new(dup).is_err());
    }
}
