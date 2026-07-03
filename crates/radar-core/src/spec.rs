//! The radar specification — a serde `RadarSpec` and its validation.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::error::{Error, Result};
use crate::signal::Signal;

/// The specification a radar scan is driven by: the perp universe, the weighted
/// signals, the alert threshold and an optional top-N limit.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct RadarSpec {
    /// Exchange IDs (documentary + optional live filter); empty means all.
    #[serde(default)]
    pub venues: Vec<String>,
    /// The perp universe; empty means all fed symbols.
    #[serde(default)]
    pub symbols: Vec<String>,
    /// The weighted signals; at least one is required.
    pub signals: Vec<Signal>,
    /// The severity threshold an alert must reach; defaults to `0.0` (all perps).
    #[serde(default)]
    pub threshold: f64,
    /// Optional top-N cap by severity.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<usize>,
}

impl RadarSpec {
    /// Parse a spec from JSON and validate it.
    pub fn from_json(s: &str) -> Result<Self> {
        let spec: RadarSpec = serde_json::from_str(s)?;
        spec.validate()?;
        Ok(spec)
    }

    /// Parse a spec from TOML and validate it.
    pub fn from_toml(s: &str) -> Result<Self> {
        let spec: RadarSpec = toml::from_str(s)?;
        spec.validate()?;
        Ok(spec)
    }

    /// Check the spec is structurally sound: at least one signal, each with the
    /// right parameter arity and a finite non-negative weight, a finite
    /// threshold, a positive limit if present, and no two signals sharing a
    /// `factors` key.
    pub(crate) fn validate(&self) -> Result<()> {
        if self.signals.is_empty() {
            return Err(Error::BadSpec("at least one signal is required".into()));
        }
        if !self.threshold.is_finite() {
            return Err(Error::BadSpec("threshold must be finite".into()));
        }
        if self.limit == Some(0) {
            return Err(Error::BadSpec("limit must be greater than zero".into()));
        }
        let mut keys = BTreeSet::new();
        for signal in &self.signals {
            let arity = signal.kind.arity();
            if signal.params.len() != arity {
                return Err(Error::BadSpec(format!(
                    "{} takes {arity} parameter(s), got {}",
                    signal.kind.as_str(),
                    signal.params.len()
                )));
            }
            if !signal.weight.is_finite() || signal.weight < 0.0 {
                return Err(Error::BadSpec(format!(
                    "{} weight must be finite and non-negative",
                    signal.kind.as_str()
                )));
            }
            let key = signal.key();
            if !keys.insert(key.clone()) {
                return Err(Error::BadSpec(format!("duplicate signal: {key}")));
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::signal::SignalKind;

    fn sig(kind: SignalKind, params: Vec<f64>, weight: f64) -> Signal {
        Signal { kind, params, weight }
    }

    fn spec_with(signals: Vec<Signal>, threshold: f64, limit: Option<usize>) -> RadarSpec {
        RadarSpec { venues: vec![], symbols: vec![], signals, threshold, limit }
    }

    #[test]
    fn valid_spec_passes() {
        let spec = spec_with(vec![sig(SignalKind::OiDelta, vec![20.0, 0.1], 2.0)], 0.4, Some(50));
        assert!(spec.validate().is_ok());
    }

    #[test]
    fn empty_signals_rejected() {
        assert!(spec_with(vec![], 0.0, None).validate().is_err());
    }

    #[test]
    fn wrong_arity_rejected() {
        let spec = spec_with(vec![sig(SignalKind::OiDelta, vec![20.0], 1.0)], 0.0, None);
        assert!(spec.validate().is_err());
    }

    #[test]
    fn negative_or_nonfinite_weight_rejected() {
        let neg = spec_with(vec![sig(SignalKind::FundingFlip, vec![0.0005], -1.0)], 0.0, None);
        assert!(neg.validate().is_err());
        let nan = spec_with(vec![sig(SignalKind::FundingFlip, vec![0.0005], f64::NAN)], 0.0, None);
        assert!(nan.validate().is_err());
    }

    #[test]
    fn nonfinite_threshold_rejected() {
        let spec = spec_with(vec![sig(SignalKind::FundingFlip, vec![0.0005], 1.0)], f64::INFINITY, None);
        assert!(spec.validate().is_err());
    }

    #[test]
    fn zero_limit_rejected() {
        let spec = spec_with(vec![sig(SignalKind::FundingFlip, vec![0.0005], 1.0)], 0.0, Some(0));
        assert!(spec.validate().is_err());
    }

    #[test]
    fn duplicate_signal_key_rejected() {
        let spec = spec_with(
            vec![
                sig(SignalKind::OiDelta, vec![20.0, 0.1], 1.0),
                sig(SignalKind::OiDelta, vec![20.0, 0.1], 2.0),
            ],
            0.0,
            None,
        );
        assert!(spec.validate().is_err());
    }

    #[test]
    fn from_json_parses_defaults_and_validates() {
        let json = r#"{"symbols":["BTCUSDT"],"signals":[{"kind":"oi_delta","params":[20,0.1],"weight":2.0}],"threshold":0.4}"#;
        let spec = RadarSpec::from_json(json).unwrap();
        assert_eq!(spec.signals.len(), 1);
        assert!(spec.venues.is_empty());
        assert!(spec.limit.is_none());
    }
}
