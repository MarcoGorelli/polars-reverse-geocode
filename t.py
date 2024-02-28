import polars as pl

from polars_reverse_geocode import geohash

df = pl.DataFrame({
    'lat': [37.7749, 51.01, 52.5]*(19_260_379//3),
    'lon': [-122.4194, -3.9, -.91]*(19_260_379//3),
})
print(df.with_columns(
    # city=reverse_geocode('lat', 'lon'),
    geohash = geohash('lat', 'lon'),
))
