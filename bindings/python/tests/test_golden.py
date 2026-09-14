"""Cross-language golden: every binding must produce byte-identical report JSON.

The fixtures live in the repository-root ``golden/`` directory (specs + a shared
event universe + expected responses). They are added in a later phase; until then
this test skips cleanly.
"""

import json
import pathlib

from wickra_radar import Radar

ROOT = pathlib.Path(__file__).resolve().parents[3]
GOLDEN = ROOT / "golden"


def _spec_files() -> list[pathlib.Path]:
    specs = GOLDEN / "specs"
    if not specs.exists():
        return []
    return sorted(specs.glob("*.json"))


def test_golden_reports_are_byte_identical() -> None:
    # One function over every case rather than a parametrized test: this
    # module also runs on the Python 3.9 row, which has no test framework
    # installed (see run_without_pytest.py). A missing corpus is a failure,
    # not a skip.
    specs = _spec_files()
    assert specs, "golden corpus not found"
    for spec_path in specs:
        _check_case(spec_path)


def _check_case(spec_path: pathlib.Path) -> None:
    events = json.loads((GOLDEN / "events.json").read_text(encoding="utf-8"))
    expected = (GOLDEN / "expected" / f"{spec_path.stem}.json").read_text(
        encoding="utf-8"
    )
    radar = Radar(spec_path.read_text(encoding="utf-8"))
    response = radar.command(json.dumps({"cmd": "scan", "events": events}))
    assert response == expected.strip(), spec_path.stem
