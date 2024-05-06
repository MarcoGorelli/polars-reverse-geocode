import polars as pl
from polars.testing import assert_frame_equal

from polars_reverse_geocode import reverse_geocode, find_closest_state


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


def test_find_closest_state() -> None:
    df = pl.DataFrame({"lat": [37.7749, 51.01, 52.5], "lon": [-122.4194, -3.9, -0.91]})
    result = df.with_columns(city=find_closest_state("lat", "lon"))
    expected = pl.DataFrame(
        {
            "lat": [37.7749, 51.01, 52.5],
            "lon": [-122.4194, -3.9, -0.91],
            "city": ["California", "England", "England"],
        }
    )
    assert_frame_equal(result, expected)
