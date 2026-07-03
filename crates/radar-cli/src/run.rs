//! The run pipeline: load a spec and an event stream, scan, render it (§2.3).

use std::collections::BTreeMap;
use std::error::Error;
use std::fs;
use std::io::Read;
use std::path::Path;

use radar_core::{scan, Event, RadarReport, RadarSpec};

use crate::args::{Args, Format};

/// Load the spec, apply any overrides, load the events, scan and render.
///
/// # Errors
/// Returns an error if a file cannot be read or parsed, no event source is
/// given, or the (overridden) spec fails validation.
pub fn run(args: &Args) -> Result<String, Box<dyn Error>> {
    let mut spec = load_spec(&args.spec)?;
    if let Some(limit) = args.limit {
        spec.limit = Some(limit);
    }
    if let Some(threshold) = args.threshold {
        spec.threshold = threshold;
    }

    let events = load_events(args)?;
    let report = scan(&events, &spec)?;

    Ok(match args.format {
        Format::Json => serde_json::to_string(&report)?,
        Format::Text => render_text(&report),
    })
}

/// Parse the spec file, choosing TOML or JSON by extension (JSON by default).
fn load_spec(path: &Path) -> Result<RadarSpec, Box<dyn Error>> {
    let text = fs::read_to_string(path)?;
    let is_toml = path.extension().and_then(|ext| ext.to_str()) == Some("toml");
    let spec = if is_toml {
        RadarSpec::from_toml(&text)?
    } else {
        RadarSpec::from_json(&text)?
    };
    Ok(spec)
}

/// Load the per-symbol events from stdin (`--stdin`) or a directory
/// (`--events`).
fn load_events(args: &Args) -> Result<BTreeMap<String, Vec<Event>>, Box<dyn Error>> {
    if args.stdin {
        let mut text = String::new();
        std::io::stdin().read_to_string(&mut text)?;
        Ok(serde_json::from_str(&text)?)
    } else if let Some(dir) = &args.events {
        load_events_dir(dir)
    } else {
        Err("either --events <dir> or --stdin is required".into())
    }
}

/// Assemble the universe from `<SYMBOL>.jsonl` files in `dir` (one JSON `Event`
/// per line). The `BTreeMap` orders symbols by key, so the scan is deterministic
/// regardless of directory-read order.
fn load_events_dir(dir: &Path) -> Result<BTreeMap<String, Vec<Event>>, Box<dyn Error>> {
    let mut universe = BTreeMap::new();
    for entry in fs::read_dir(dir)? {
        let path = entry?.path();
        if path.extension().and_then(|ext| ext.to_str()) != Some("jsonl") {
            continue;
        }
        let symbol = path
            .file_stem()
            .and_then(|stem| stem.to_str())
            .ok_or("event file has no valid name")?
            .to_owned();
        let text = fs::read_to_string(&path)?;
        let mut events = Vec::new();
        for line in text.lines() {
            let line = line.trim();
            if !line.is_empty() {
                events.push(serde_json::from_str::<Event>(line)?);
            }
        }
        universe.insert(symbol, events);
    }
    Ok(universe)
}

/// An aligned, human-readable table of alerts.
fn render_text(report: &RadarReport) -> String {
    if report.alerts.is_empty() {
        return format!("no alerts ({} scanned)", report.scanned);
    }
    let width = report
        .alerts
        .iter()
        .map(|a| a.symbol.len())
        .max()
        .unwrap_or(6)
        .max(6);
    let mut lines = vec![format!(
        "{:<width$}  severity              signal",
        "symbol"
    )];
    for alert in &report.alerts {
        lines.push(format!(
            "{:<width$}  {:>8.4} {}  {}",
            alert.symbol,
            alert.severity,
            severity_bar(alert.severity),
            top_factor(&alert.factors),
        ));
    }
    lines.push(format!(
        "{} alert(s), {} scanned",
        report.alerts.len(),
        report.scanned
    ));
    lines.join("\n")
}

/// A ten-cell severity bar, e.g. `[######    ]` for `0.6`.
fn severity_bar(severity: f64) -> String {
    let filled = ((severity * 10.0).round() as usize).min(10);
    format!("[{}{}]", "#".repeat(filled), " ".repeat(10 - filled))
}

/// The highest-scoring signal (excluding the `"severity"` entry), as
/// `key=score`.
fn top_factor(factors: &BTreeMap<String, f64>) -> String {
    factors
        .iter()
        .filter(|(key, _)| key.as_str() != "severity")
        .max_by(|a, b| a.1.total_cmp(b.1))
        .map_or_else(String::new, |(key, score)| format!("{key}={score:.4}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    const SPEC: &str = r#"{"symbols":["AAA"],"signals":[{"kind":"funding_flip","params":[0.0005]}],"threshold":0.0}"#;
    // AAA's funding flips 0.0003 -> -0.0004: severity 1.0.
    const AAA_JSONL: &str = concat!(
        r#"{"kind":"derivatives","ts":1,"open_interest":1.0,"funding_rate":0.0003,"mark_price":50.0}"#,
        "\n",
        r#"{"kind":"derivatives","ts":2,"open_interest":1.0,"funding_rate":-0.0004,"mark_price":50.0}"#,
        "\n",
    );

    fn scanned_report() -> RadarReport {
        let spec: RadarSpec = serde_json::from_str(SPEC).unwrap();
        let mut events = BTreeMap::new();
        events.insert(
            "AAA".to_owned(),
            vec![
                Event::Derivatives {
                    ts: 1,
                    open_interest: 1.0,
                    funding_rate: 0.0003,
                    mark_price: 50.0,
                },
                Event::Derivatives {
                    ts: 2,
                    open_interest: 1.0,
                    funding_rate: -0.0004,
                    mark_price: 50.0,
                },
            ],
        );
        scan(&events, &spec).unwrap()
    }

    #[test]
    fn render_text_lists_alerts() {
        let text = render_text(&scanned_report());
        assert!(text.contains("AAA"));
        assert!(text.contains("funding_flip(0.0005)"));
        assert!(text.contains("1 alert(s), 1 scanned"));
    }

    #[test]
    fn render_text_reports_no_alerts() {
        let empty = RadarReport {
            alerts: vec![],
            scanned: 3,
        };
        assert_eq!(render_text(&empty), "no alerts (3 scanned)");
    }

    #[test]
    fn run_loads_a_directory_and_matches_batch_json() {
        let base = std::env::temp_dir().join("wickra-radar-run-dir-test");
        let events = base.join("events");
        fs::create_dir_all(&events).unwrap();
        fs::write(base.join("spec.json"), SPEC).unwrap();
        fs::write(events.join("AAA.jsonl"), AAA_JSONL).unwrap();

        let args = Args {
            spec: base.join("spec.json"),
            events: Some(events.clone()),
            stdin: false,
            limit: None,
            threshold: None,
            format: Format::Json,
        };
        let out = run(&args).unwrap();
        let expected = serde_json::to_string(&scanned_report()).unwrap();
        assert_eq!(out, expected);

        fs::remove_dir_all(&base).unwrap();
    }

    #[test]
    fn threshold_override_drops_alerts() {
        let base = std::env::temp_dir().join("wickra-radar-run-override-test");
        let events = base.join("events");
        fs::create_dir_all(&events).unwrap();
        fs::write(base.join("spec.json"), SPEC).unwrap();
        fs::write(events.join("AAA.jsonl"), AAA_JSONL).unwrap();

        let args = Args {
            spec: base.join("spec.json"),
            events: Some(events.clone()),
            stdin: false,
            limit: None,
            threshold: Some(2.0),
            format: Format::Json,
        };
        let out = run(&args).unwrap();
        assert!(out.contains(r#""alerts":[]"#));

        fs::remove_dir_all(&base).unwrap();
    }

    #[test]
    fn missing_source_errors() {
        let base = std::env::temp_dir().join("wickra-radar-run-nosrc-test");
        fs::create_dir_all(&base).unwrap();
        let spec_path = base.join("spec.json");
        fs::write(&spec_path, SPEC).unwrap();

        let args = Args {
            spec: spec_path,
            events: None,
            stdin: false,
            limit: None,
            threshold: None,
            format: Format::Text,
        };
        let err = run(&args).unwrap_err();
        assert!(err.to_string().contains("--events"));

        fs::remove_dir_all(&base).unwrap();
    }
}
