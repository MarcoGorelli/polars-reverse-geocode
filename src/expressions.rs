#![allow(clippy::unused_unit)]
use polars::prelude::*;
use polars_arrow::array::MutablePlString;
use polars_core::utils::align_chunks_binary;
use pyo3_polars::derive::polars_expr;
use serde::Deserialize;
use std::fmt::Write;

use reverse_geocoder::ReverseGeocoder;

#[polars_expr(output_type=String)]
fn reverse_geocode(inputs: &[Series]) -> PolarsResult<Series> {
    let lhs = inputs[0].f64()?;
    let rhs = inputs[1].f64()?;
    let geocoder = ReverseGeocoder::new();

    let (lhs, rhs) = align_chunks_binary(lhs, rhs);
    let chunks = lhs
        .downcast_iter()
        .zip(rhs.downcast_iter())
        .map(|(lhs_arr, rhs_arr)| {
            let mut buf = String::new();
            let mut mutarr = MutablePlString::with_capacity(lhs_arr.len());

            for (lhs_opt_val, rhs_opt_val) in lhs_arr.iter().zip(rhs_arr.iter()) {
                match (lhs_opt_val, rhs_opt_val) {
                    (Some(lhs_val), Some(rhs_val)) => {
                        let res = &geocoder.search((*lhs_val, *rhs_val)).record.name;
                        buf.clear();
                        write!(buf, "{res}").unwrap();
                        mutarr.push(Some(&buf))
                    }
                    _ => mutarr.push_null(),
                }
            }

            mutarr.freeze().boxed()
        })
        .collect();
    let out: StringChunked = unsafe { ChunkedArray::from_chunks("placeholder", chunks) };
    Ok(out.into_series())
}

#[derive(Deserialize)]
struct H3Kwargs {
    resolution: u8,
}

#[polars_expr(output_type=String)]
fn h3(inputs: &[Series], kwargs: H3Kwargs) -> PolarsResult<Series> {
    let lhs = inputs[0].f64()?;
    let rhs = inputs[1].f64()?;

    use h3o::{LatLng, Resolution};

    let resolution = match kwargs.resolution {
        1 => Resolution::One,
        2 => Resolution::Two,
        3 => Resolution::Three,
        4 => Resolution::Four,
        5 => Resolution::Five,
        6 => Resolution::Six,
        7 => Resolution::Seven,
        8 => Resolution::Eight,
        9 => Resolution::Nine,
        _ => {
            polars_bail!(InvalidOperation: "expected resolution between 1 and 9, got {}", kwargs.resolution)
        }
    };

    let (lhs, rhs) = align_chunks_binary(lhs, rhs);
    let chunks = lhs
        .downcast_iter()
        .zip(rhs.downcast_iter())
        .map(|(lhs_arr, rhs_arr)| {
            let mut buf = String::new();
            let mut mutarr = MutablePlString::with_capacity(lhs_arr.len());

            for (lhs_opt_val, rhs_opt_val) in lhs_arr.iter().zip(rhs_arr.iter()) {
                match (lhs_opt_val, rhs_opt_val) {
                    (Some(lhs_val), Some(rhs_val)) => {
                        let coord = LatLng::new(*lhs_val, *rhs_val).expect("valid coord");
                        let cell = coord.to_cell(resolution);
                        let res = cell.to_string();
                        buf.clear();
                        write!(buf, "{res}").unwrap();
                        mutarr.push(Some(&buf))
                    }
                    _ => mutarr.push_null(),
                }
            }

            mutarr.freeze().boxed()
        })
        .collect();
    let out: StringChunked = unsafe { ChunkedArray::from_chunks("placeholder", chunks) };
    Ok(out.into_series())
}
