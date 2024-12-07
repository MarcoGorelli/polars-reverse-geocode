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
