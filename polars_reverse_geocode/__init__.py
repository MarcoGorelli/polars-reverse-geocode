from __future__ import annotations

from typing import TYPE_CHECKING

import polars as pl
from polars.utils.udfs import _get_shared_lib_location

from polars_reverse_geocode.utils import parse_into_expr

__version__ = "0.1.11"

if TYPE_CHECKING:
    from polars.type_aliases import IntoExpr

lib = _get_shared_lib_location(__file__)


def reverse_geocode(lat: IntoExpr, lon: IntoExpr) -> pl.Expr:
    lat = parse_into_expr(lat)
    return lat.register_plugin(
        lib=lib, symbol="reverse_geocode", is_elementwise=True, args=[lon]
    )


def geohash(lat: IntoExpr, lon: IntoExpr, *, precision: int = 6) -> pl.Expr:
    lat = parse_into_expr(lat)
    return lat.register_plugin(
        lib=lib,
        symbol="geohash",
        is_elementwise=True,
        args=[lon],
        kwargs={"precision": precision},
    )
