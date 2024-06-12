from __future__ import annotations

from typing import TYPE_CHECKING

from pathlib import Path
import polars as pl

from polars_reverse_geocode.utils import parse_into_expr, register_plugin, parse_version

__version__ = "0.1.9"

if TYPE_CHECKING:
    from polars.type_aliases import IntoExpr

if parse_version(pl.__version__) < parse_version("0.20.16"):
    from polars.utils.udfs import _get_shared_lib_location  # type: ignore[import-not-found]

    lib: str | Path = _get_shared_lib_location(__file__)
else:
    lib = Path(__file__).parent


def find_closest_city(lat: IntoExpr, long: IntoExpr) -> pl.Expr:
    lat = parse_into_expr(lat)
    return register_plugin(
        lib=lib, symbol="reverse_geocode", is_elementwise=True, args=[lat, long]
    )


# alias for backwards compatibility
reverse_geocode = find_closest_city


def find_closest_state(lat: IntoExpr, long: IntoExpr) -> pl.Expr:
    lat = parse_into_expr(lat)
    return register_plugin(
        lib=lib, symbol="find_closest_state", is_elementwise=True, args=[lat, long]
    )
