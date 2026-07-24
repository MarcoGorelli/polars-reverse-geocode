# Polars-Reverse-Geocode

Polars plugin based on https://github.com/gx0r/rrgeo.

> rrgeo takes a latitude and longitude as input and returns the closest city, country, latitude, and longitude, using a k-d tree to efficiently find the nearest neighbour based on a known list of locations. This can be useful if you need to reverse geocode a large number of coordinates quickly, or just need the rough location of coordinates but don't want the expense or complication of an online reverse geocoder.

## Installation

```
pip install polars-reverse-geocode
```

## Usage example

```python
import polars as pl

from polars_reverse_geocode import find_closest_city, find_closest_state, find_closest_country

df = pl.DataFrame({
    'lat': [37.7749, 51.01, 52.5],
    'lon': [-122.4194, -3.9, -.91]
})

print(
    df.with_columns(
        city = find_closest_city('lat', 'lon'),
        state = find_closest_state('lat', 'lon'),
        country_code = find_closest_country('lat', 'lon')
    )
)
```

```
shape: (3, 5)
┌─────────┬───────────┬───────────────────┬────────────┬──────────────┐
│ lat     ┆ lon       ┆ city              ┆ state      ┆ country_code │
│ ---     ┆ ---       ┆ ---               ┆ ---        ┆ ---          │
│ f64     ┆ f64       ┆ str               ┆ str        ┆ str          │
╞═════════╪═══════════╪═══════════════════╪════════════╪══════════════╡
│ 37.7749 ┆ -122.4194 ┆ San Francisco     ┆ California ┆ US           │
│ 51.01   ┆ -3.9      ┆ South Molton      ┆ England    ┆ GB           │
│ 52.5    ┆ -0.91     ┆ Market Harborough ┆ England    ┆ GB           │
└─────────┴───────────┴───────────────────┴────────────┴──────────────┘
```

## Caching

Loading the geocoding data set has a fixed cost of roughly 150ms, which is
noticeable when doing repeated, one-off lookups (e.g. sequential/chained
calls in a UI). Each `find_closest_*` function accepts a `cache_mode`
keyword argument to control this caching behaviour:

- `"cache_forever"` (default): loads the data set once per process and
  keeps it in memory (~2.5GB RAM) for the lifetime of the process, so
  subsequent calls are fast.
- `"do_not_cache"`: does not populate the shared, process-wide cache.
  If the cache has already been populated (e.g. by an earlier
  `"cache_forever"` call), it's reused; otherwise, the data set is loaded
  fresh on every call, and discarded afterwards, avoiding the memory cost.

```python
df.with_columns(
    city=find_closest_city("lat", "lon", cache_mode="do_not_cache"),
)
```
