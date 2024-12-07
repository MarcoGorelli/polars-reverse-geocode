#![allow(clippy::unused_unit)]
use polars::prelude::*;
use polars_core::prelude::arity::binary_elementwise_into_string_amortized;
use pyo3_polars::derive::polars_expr;
use std::fmt::Write;

use reverse_geocoder::ReverseGeocoder;

#[derive(PartialEq, Eq)]
enum LocationLevel {
    City,
    State,
    Country,
}

fn reverse_geocode(inputs: &[Series], location_level: LocationLevel) -> PolarsResult<Series> {
    let lhs = inputs[0].f64()?;
    let rhs = inputs[1].f64()?;
    let geocoder = ReverseGeocoder::new();

    let out = binary_elementwise_into_string_amortized(lhs, rhs, |lat, lon, buf| {
        let record = geocoder.search((lat, lon)).record;
        let res: &String = match location_level {
            LocationLevel::City => &record.name,
            LocationLevel::State => &record.admin1,
            LocationLevel::Country => &record.cc,
        };
        write!(buf, "{}", res).unwrap();
    });
    Ok(out.into_series())
}

#[polars_expr(output_type=String)]
fn find_closest_city(inputs: &[Series]) -> PolarsResult<Series> {
    reverse_geocode(inputs, LocationLevel::City)
}

#[polars_expr(output_type=String)]
fn find_closest_state(inputs: &[Series]) -> PolarsResult<Series> {
    reverse_geocode(inputs, LocationLevel::State)
}

#[polars_expr(output_type=String)]
fn find_closest_country(inputs: &[Series]) -> PolarsResult<Series> {
    reverse_geocode(inputs, LocationLevel::Country)
}
