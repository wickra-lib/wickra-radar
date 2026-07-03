#![no_main]
//! Fuzz the JSON command boundary: a radar built from a fixed valid spec is
//! driven with arbitrary bytes as a command. `command_json` must never panic —
//! any malformed or unknown command comes back as an in-band
//! `{"ok":false,...}` reply, never a crash.

use libfuzzer_sys::fuzz_target;
use radar_core::Radar;

const SPEC: &str = r#"{"signals":[{"kind":"funding_flip","params":[0.0005]},{"kind":"oi_delta","params":[3,0.1]}]}"#;

fuzz_target!(|data: &[u8]| {
    let Ok(text) = std::str::from_utf8(data) else {
        return;
    };
    // A radar from a known-good spec always builds; the fuzz surface is the
    // command string.
    let mut radar = Radar::new(SPEC).expect("the fixed spec is valid");
    let _ = radar.command_json(text);
});
