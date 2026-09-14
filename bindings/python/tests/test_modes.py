"""Operating-mode equivalence through the binding: the radar has two ways to
consume a universe -- one-shot ``scan`` over every symbol, or streaming
``feed`` / ``feed_batch`` per symbol followed by ``alerts`` -- and both must
return the same report bytes for every golden spec. The core pins this in
Rust (streaming_eq_batch.rs); this checks the boundary the Python binding
crosses. Plain functions with plain asserts: the module also runs on the 3.9
row through run_without_pytest.py."""

import json
import pathlib

from wickra_radar import Radar

ROOT = pathlib.Path(__file__).resolve().parents[3]
GOLDEN = ROOT / "golden"


def _specs() -> list[pathlib.Path]:
    return sorted((GOLDEN / "specs").glob("*.json"))


def _scan(spec: str, events: dict) -> str:
    return Radar(spec).command(json.dumps({"cmd": "scan", "events": events}))


def _stream(spec: str, events: dict) -> str:
    radar = Radar(spec)
    for symbol, evs in events.items():
        for event in evs:
            radar.command(json.dumps({"cmd": "feed", "symbol": symbol, "event": event}))
    return radar.command(json.dumps({"cmd": "alerts"}))


def _stream_batched(spec: str, events: dict) -> str:
    radar = Radar(spec)
    for symbol, evs in events.items():
        radar.command(json.dumps({"cmd": "feed_batch", "symbol": symbol, "events": evs}))
    return radar.command(json.dumps({"cmd": "alerts"}))


def test_streaming_equals_batch_for_every_golden_spec() -> None:
    specs = _specs()
    assert specs, "golden corpus not found"
    events = json.loads((GOLDEN / "events.json").read_text(encoding="utf-8"))
    for spec_path in specs:
        spec = spec_path.read_text(encoding="utf-8")
        expected = (GOLDEN / "expected" / spec_path.name).read_text(encoding="utf-8").strip()
        batch = _scan(spec, events)
        assert batch == expected, spec_path.stem
        assert _stream(spec, events) == batch, f"{spec_path.stem}: feed vs scan"
        assert _stream_batched(spec, events) == batch, f"{spec_path.stem}: feed_batch vs scan"
