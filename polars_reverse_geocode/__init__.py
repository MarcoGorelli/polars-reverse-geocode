from __future__ import annotations

from typing import TYPE_CHECKING

from pathlib import Path
import polars as pl

from polars.plugins import register_plugin_function

__version__ = "0.1.9"

if TYPE_CHECKING:
    from polars_reverse_geocode.typing import IntoExpr

LIB = Path(__file__).parent


def find_closest_city(lat: IntoExpr, long: IntoExpr) -> pl.Expr:
    return register_plugin_function(
        args=[lat, long],
        plugin_path=LIB,
        function_name="find_closest_city",
        is_elementwise=True,
    )


# alias for backwards compatibility
reverse_geocode = find_closest_city


def find_closest_state(lat: IntoExpr, long: IntoExpr) -> pl.Expr:
    return register_plugin_function(
        args=[lat, long],
        plugin_path=LIB,
        function_name="find_closest_state",
        is_elementwise=True,
    )


def find_closest_country(lat: IntoExpr, long: IntoExpr) -> pl.Expr:
    return register_plugin_function(
        args=[lat, long],
        plugin_path=LIB,
        function_name="find_closest_country",
        is_elementwise=True,
    )
