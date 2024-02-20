import polars as pl

from polars_reverse_geocode import reverse_geocode

df = pl.DataFrame({
    'lat': [37.7749, 51, 37.7749],
    'lon': [-122.4194, -3, -122.4194]
})
print(df.with_columns(city=reverse_geocode('lat', 'lon')))
