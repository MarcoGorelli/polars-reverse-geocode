from __future__ import annotations

from typing import TYPE_CHECKING

from pathlib import Path
import polars as pl
from polars.utils.udfs import _get_shared_lib_location

from polars_reverse_geocode.utils import parse_into_expr, register_plugin, parse_version

__version__ = "0.1.9"

if TYPE_CHECKING:
    from polars.type_aliases import IntoExpr

if parse_version(pl.__version__) < parse_version("0.20.16"):
    from polars.utils.udfs import _get_shared_lib_location

    lib: str | Path = _get_shared_lib_location(__file__)
else:
    lib = Path(__file__).parent


def reverse_geocode(lat: IntoExpr, long: IntoExpr) -> pl.Expr:
    lat = parse_into_expr(lat)
    return register_plugin(
        lib=lib, symbol="reverse_geocode", is_elementwise=True, args=[lat, long]
    )

def h3(lat: IntoExpr, long: IntoExpr, *, resolution: int = 9) -> pl.Expr:
    lat = parse_into_expr(lat)
    return register_plugin(
        lib=lib, symbol="h3", is_elementwise=True, args=[lat, long], kwargs={"resolution": resolution},
    )
