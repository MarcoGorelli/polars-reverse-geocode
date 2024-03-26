import polars as pl

from polars_reverse_geocode import h3

df = pl.DataFrame({
    'lat': [37.7749, 51.01, 52.5],
    'lon': [-122.4194, -3.9, -.91]
})
print(df.with_columns(h3=h3('lat', 'lon')))
