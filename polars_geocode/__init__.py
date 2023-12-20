import polars as pl
from polars.utils.udfs import _get_shared_lib_location
from polars.type_aliases import IntoExpr

lib = _get_shared_lib_location(__file__)


@pl.api.register_expr_namespace("geocode")
class Geocoder:
    def __init__(self, expr: pl.Expr):
        self._expr = expr

    def reverse_geocode(self, longitude) -> pl.Expr:
        return self._expr.register_plugin(
            lib=lib,
            symbol="reverse_geocode",
            is_elementwise=True,
            args=[longitude],
        )
