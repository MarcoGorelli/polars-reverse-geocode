#![allow(clippy::unused_unit)]
use polars::prelude::*;
use polars_arrow::array::MutablePlString;
use polars_core::prelude::arity::binary_elementwise_into_string_amortized;
use polars_core::utils::align_chunks_binary;
use pyo3_polars::derive::polars_expr;
use std::fmt::Write;

use reverse_geocoder::ReverseGeocoder;

#[polars_expr(output_type=String)]
fn reverse_geocode(inputs: &[Series]) -> PolarsResult<Series> {
    let lhs = inputs[0].f64()?;
    let rhs = inputs[1].f64()?;
    let geocoder = ReverseGeocoder::new();
    let out = binary_elementwise_into_string_amortized(lhs, rhs, |lat, lon, buf| {
        write!(buf, "{}", geocoder.search((lat, lon)).record.name).unwrap();
    });
    Ok(out.into_series())
}

fn reverse_geocode_chunks(inputs: &[Series], return_type: String) -> PolarsResult<Series> {
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
                        let record = &geocoder.search((*lhs_val, *rhs_val)).record;
                        let res: &String;
                        if return_type == "city" {
                            res = &record.name;
                        } else if return_type == "state" {
                            res = &record.admin1;
                        } else if return_type == "country" {
                            res = &record.cc;
                        } else {
                            res = &record.name;
                        }
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
    let out: StringChunked = unsafe { ChunkedArray::from_chunks(PlSmallStr::EMPTY, chunks) };
    Ok(out.into_series())
}

#[polars_expr(output_type=String)]
fn find_closest_state(inputs: &[Series]) -> PolarsResult<Series> {
    reverse_geocode_chunks(inputs, "state".to_string())
}

#[polars_expr(output_type=String)]
fn find_closest_country(inputs: &[Series]) -> PolarsResult<Series> {
    reverse_geocode_chunks(inputs, "country".to_string())
}
