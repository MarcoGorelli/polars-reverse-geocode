import polars as pl
from polars.testing import assert_frame_equal

from polars_reverse_geocode import reverse_geocode, geohash


def test_main() -> None:
    df = pl.DataFrame({"lat": [37.7749, 51.01, 52.5], "lon": [-122.4194, -3.9, -0.91]})
    result = df.with_columns(
        city=reverse_geocode("lat", "lon"),
        geohash_3=geohash("lat", "lon", precision=3),
        geohash_6=geohash("lat", "lon", precision=6),
    )
    expected = pl.DataFrame(
        {
            "lat": [37.7749, 51.01, 52.5],
            "lon": [-122.4194, -3.9, -0.91],
            "city": ["San Francisco", "South Molton", "Market Harborough"],
            "geohash_3": ["9q8", "gcj", "gcr"],
            "geohash_6": ["9q8yyk", "gcj4pq", "gcr6em"],
        }
    )
    assert_frame_equal(result, expected)
