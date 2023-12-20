import polars as pl
from polars.testing import assert_series_equal
import polars_geocode  # noqa: F401

def test_reverse_geocoder() -> None:
    df = pl.DataFrame({
        'latitude': [10, 20, 30],
        'longitude': [20, 30, 40],
    }).select(pl.all().cast(pl.Utf8))
    result = df.select(city=pl.col('latitude').geocode.reverse_geocode(pl.col('longitude')))['city']
    expected = pl.Series('city', ['Am Timan', 'Karmah an Nuzul', 'Sakaka'])
    assert_series_equal(result, expected)
