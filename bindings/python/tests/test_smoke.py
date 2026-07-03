"""Smoke test: construct a radar, scan events, parse the report."""

import json

from wickra_radar import Radar, __version__

SPEC = json.dumps(
    {
        "symbols": ["AAA"],
        "signals": [{"kind": "funding_flip", "params": [0.0005]}],
        "threshold": 0.0,
    }
)

EVENTS = {
    "AAA": [
        {"kind": "derivatives", "ts": 1, "open_interest": 1.0, "funding_rate": 0.0003, "mark_price": 50.0},
        {"kind": "derivatives", "ts": 2, "open_interest": 1.0, "funding_rate": -0.0004, "mark_price": 50.0},
    ]
}


def test_scan_roundtrip() -> None:
    radar = Radar(SPEC)
    report = json.loads(radar.command(json.dumps({"cmd": "scan", "events": EVENTS})))
    assert report["scanned"] == 1
    assert report["alerts"][0]["symbol"] == "AAA"
    # A funding flip clamps the severity to 1.0.
    assert abs(report["alerts"][0]["severity"] - 1.0) < 1e-9


def test_version_matches_module() -> None:
    assert Radar.version() == __version__


def test_bad_spec_raises() -> None:
    try:
        Radar("not json")
    except ValueError:
        return
    raise AssertionError("expected ValueError for a malformed spec")
