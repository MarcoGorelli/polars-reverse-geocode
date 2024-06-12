import polars as pl

from polars_reverse_geocode import find_closest_city

df = pl.DataFrame({
    'lat': [37.7749, 51.01, 52.5],
    'lon': [-122.4194, -3.9, -.91]
})
print(df.with_columns(city = find_closest_city('lat', 'lon')))
