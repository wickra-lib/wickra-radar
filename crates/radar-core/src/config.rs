//! The CLI config file — a [`RadarSpec`] wrapper.
//!
//! The CLI reads a JSON or TOML config with a top-level `spec` table and hands
//! the validated spec to the core. Parsing validates the spec, so a malformed
//! config is rejected at load time rather than mid-scan.

use serde::{Deserialize, Serialize};

use crate::error::Result;
use crate::spec::RadarSpec;

/// A parsed config file.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Config {
    /// The radar spec to run.
    pub spec: RadarSpec,
}

impl Config {
    /// Parse and validate a config from JSON.
    ///
    /// # Errors
    /// Returns an error if the config fails to parse or the spec fails to
    /// validate.
    pub fn from_json(s: &str) -> Result<Self> {
        let config: Config = serde_json::from_str(s)?;
        config.spec.validate()?;
        Ok(config)
    }

    /// Parse and validate a config from TOML.
    ///
    /// # Errors
    /// Returns an error if the config fails to parse or the spec fails to
    /// validate.
    pub fn from_toml(s: &str) -> Result<Self> {
        let config: Config = toml::from_str(s)?;
        config.spec.validate()?;
        Ok(config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_json_config() {
        let config = Config::from_json(
            r#"{ "spec": {
                "symbols": ["AAA"],
                "signals": [ { "kind": "funding_flip", "params": [0.0005] } ],
                "threshold": 0.3
            } }"#,
        )
        .unwrap();
        assert_eq!(config.spec.symbols, vec!["AAA"]);
        assert_eq!(config.spec.signals.len(), 1);
    }

    #[test]
    fn parses_toml_config() {
        let config = Config::from_toml(
            r#"
            [spec]
            symbols = ["AAA"]
            threshold = 0.3
            [[spec.signals]]
            kind = "funding_flip"
            params = [0.0005]
            "#,
        )
        .unwrap();
        assert_eq!(config.spec.symbols, vec!["AAA"]);
    }

    #[test]
    fn rejects_an_invalid_spec() {
        // Empty signals fails validation.
        let err = Config::from_json(r#"{ "spec": { "signals": [] } }"#);
        assert!(err.is_err());
    }
}
