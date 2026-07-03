"""Wickra Radar — the data-driven liquidation-cascade radar core.

Build a :class:`Radar` from a spec JSON, drive it with command JSONs, and read
back alert reports. The same command protocol crosses every language binding, so
this Python front-end drives the exact same core as the native CLI.
"""

from ._wickra_radar import Radar, __version__

__all__ = ["Radar", "__version__"]
