"""polars_reverse_geocode."""

from __future__ import annotations

from pathlib import Path
from typing import TYPE_CHECKING, Literal

from polars.plugins import register_plugin_function

__version__ = "0.1.9"

if TYPE_CHECKING:
    import polars as pl

    from polars_reverse_geocode.typing import IntoExpr

LIB = Path(__file__).parent

CacheMode = Literal["do_not_cache", "cache_forever"]


def find_closest_city(
    lat: IntoExpr, long: IntoExpr, *, cache_mode: CacheMode = "cache_forever"
) -> pl.Expr:
    """Reverse geocode a lat/long point, and return the city name."""
    return register_plugin_function(
        args=[lat, long],
        plugin_path=LIB,
        function_name="find_closest_city",
        is_elementwise=True,
        kwargs={"cache_mode": cache_mode},
    )


# alias for backwards compatibility
reverse_geocode = find_closest_city


def find_closest_state(
    lat: IntoExpr, long: IntoExpr, *, cache_mode: CacheMode = "cache_forever"
) -> pl.Expr:
    """Reverse geocode a lat/long point, and return the state/province name."""
    return register_plugin_function(
        args=[lat, long],
        plugin_path=LIB,
        function_name="find_closest_state",
        is_elementwise=True,
        kwargs={"cache_mode": cache_mode},
    )


def find_closest_country(
    lat: IntoExpr, long: IntoExpr, *, cache_mode: CacheMode = "cache_forever"
) -> pl.Expr:
    """Reverse geocode a lat/long point, and return the 2-character ISO country code."""
    return register_plugin_function(
        args=[lat, long],
        plugin_path=LIB,
        function_name="find_closest_country",
        is_elementwise=True,
        kwargs={"cache_mode": cache_mode},
    )
