
use polars::prelude::*;
use pyo3_polars::derive::{polars_expr};
use polars::prelude::arity::binary_elementwise;
use std::fmt::Write;

use reverse_geocoder::{ReverseGeocoder, SearchResult};
use std::time::{Duration, Instant};

#[polars_expr(output_type=Utf8)]
fn reverse_geocode(inputs: &[Series]) -> PolarsResult<Series> {
    let left = inputs[0].utf8()?;
    let right = inputs[1].utf8()?;
    let geocoder = ReverseGeocoder::new();
    let out: Utf8Chunked = binary_elementwise(left, right, |left: Option<&str>, right: Option<&str>|
        {
            match (left, right) {
                (Some(left), Some(right)) => {
                    let left: f64 = left.parse().unwrap();
                    let right: f64 = right.parse().unwrap();
                    let res = &geocoder.search((left, right)).record.name;
                    // todo: don't clone, push to a buffer and clear it each time
                    // but this is still "good enough" for now, and beats iterating over
                    // rows in Python
                    Some(res.clone())
                },
                _ => None
            }
        }
    );
    Ok(out.into_series())
}
