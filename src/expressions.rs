#![allow(clippy::unused_unit)]
use polars::prelude::*;
use polars_core::prelude::arity::binary_elementwise_into_string_amortized;
use pyo3_polars::derive::polars_expr;
use serde::Deserialize;
use std::fmt::Write;
use std::sync::OnceLock;

use reverse_geocoder::ReverseGeocoder;

#[derive(PartialEq, Eq)]
enum LocationLevel {
    City,
    State,
    Country,
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
enum CacheMode {
    DoNotCache,
    CacheForever,
}

#[derive(Deserialize)]
struct ReverseGeocodeKwargs {
    cache_mode: CacheMode,
}

static GEOCODER: OnceLock<ReverseGeocoder> = OnceLock::new();

fn reverse_geocode(
    inputs: &[Series],
    location_level: LocationLevel,
    cache_mode: CacheMode,
) -> PolarsResult<Series> {
    let lhs = inputs[0].f64()?;
    let rhs = inputs[1].f64()?;

    let owned_geocoder;
    let geocoder = match cache_mode {
        CacheMode::CacheForever => GEOCODER.get_or_init(ReverseGeocoder::new),
        CacheMode::DoNotCache => match GEOCODER.get() {
            // Even if the cache_mode requests no caching, we'll check and use the
            // global cache if it's available from previous calls.
            Some(geocoder) => geocoder,
            None => {
                owned_geocoder = ReverseGeocoder::new();
                &owned_geocoder
            }
        },
    };

    let out = binary_elementwise_into_string_amortized(lhs, rhs, |lat, lon, buf| {
        let record = geocoder.search((lat, lon)).record;
        let res: &String = match location_level {
            LocationLevel::City => &record.name,
            LocationLevel::State => &record.admin1,
            LocationLevel::Country => &record.cc,
        };
        write!(buf, "{res}").unwrap();
    });
    Ok(out.into_series())
}

#[polars_expr(output_type=String)]
fn find_closest_city(inputs: &[Series], kwargs: ReverseGeocodeKwargs) -> PolarsResult<Series> {
    reverse_geocode(inputs, LocationLevel::City, kwargs.cache_mode)
}

#[polars_expr(output_type=String)]
fn find_closest_state(inputs: &[Series], kwargs: ReverseGeocodeKwargs) -> PolarsResult<Series> {
    reverse_geocode(inputs, LocationLevel::State, kwargs.cache_mode)
}

#[polars_expr(output_type=String)]
fn find_closest_country(inputs: &[Series], kwargs: ReverseGeocodeKwargs) -> PolarsResult<Series> {
    reverse_geocode(inputs, LocationLevel::Country, kwargs.cache_mode)
}
