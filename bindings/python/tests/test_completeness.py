"""Pin the public surface of the wickra_radar module and Radar class.

The class-surface guard mirrors the Node/R completeness checks; the module guard
pins the package's exported names (via ``__all__``) so a stray export — or a
dropped one — fails loudly, matching the exact-surface guard in the Node binding.
"""

import wickra_radar
from wickra_radar import Radar

EXPECTED_METHODS = {"command", "version"}
EXPECTED_EXPORTS = ["Radar", "__version__"]


def test_expected_methods_present() -> None:
    for name in EXPECTED_METHODS:
        assert hasattr(Radar, name), f"missing method: {name}"


def test_no_unexpected_public_methods() -> None:
    public = {name for name in dir(Radar) if not name.startswith("_")}
    assert public == EXPECTED_METHODS


def test_module_all_is_exact() -> None:
    assert wickra_radar.__all__ == EXPECTED_EXPORTS


def test_module_exposes_radar_and_version() -> None:
    assert isinstance(wickra_radar.Radar, type)
    assert isinstance(wickra_radar.__version__, str)
    assert wickra_radar.__version__ == Radar.version()
