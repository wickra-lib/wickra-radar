"""A runnable Python example: scan a perp universe through the binding.

    pip install wickra-radar
    python examples/python/scan.py
"""

import json

from wickra_radar import Radar

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


def main() -> None:
    radar = Radar(SPEC)
    response = radar.command(json.dumps({"cmd": "scan", "events": EVENTS}))
    report = json.loads(response)

    print(f"wickra-radar {Radar.version()}")
    print(response)
    print(f"  alerts: {len(report['alerts'])}")


if __name__ == "__main__":
    main()
