import polars as pl
from polars.testing import assert_frame_equal

from polars_reverse_geocode import reverse_geocode, h3


def test_main() -> None:
    df = pl.DataFrame({"lat": [37.7749, 51.01, 52.5], "lon": [-122.4194, -3.9, -0.91]})
    result = df.with_columns(city=reverse_geocode("lat", "lon"))
    expected = pl.DataFrame(
        {
            "lat": [37.7749, 51.01, 52.5],
            "lon": [-122.4194, -3.9, -0.91],
            "city": ["San Francisco", "South Molton", "Market Harborough"],
        }
    )
    assert_frame_equal(result, expected)


def test_h3() -> None:
    df = pl.DataFrame({"lat": [37.7749, 51.01, 52.5], "lon": [-122.4194, -3.9, -0.91]})
    result = df.with_columns(h3=h3("lat", "lon"))
    expected = pl.DataFrame(
        {
            "lat": [37.7749, 51.01, 52.5],
            "lon": [-122.4194, -3.9, -0.91],
            "h3": ["89283082803ffff", "89195b5b04fffff", "8919436a5d7ffff"],
        }
    )
    assert_frame_equal(result, expected)

    df = pl.DataFrame({"lat": [37.7749, 51.01, 52.5], "lon": [-122.4194, -3.9, -0.91]})
    result = df.with_columns(h3=h3("lat", "lon", resolution=6))
    expected = pl.DataFrame(
        {
            "lat": [37.7749, 51.01, 52.5],
            "lon": [-122.4194, -3.9, -0.91],
            "h3": ["86283082fffffff", "86195b5b7ffffff", "8619436a7ffffff"],
        }
    )
    assert_frame_equal(result, expected)
