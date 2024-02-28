#![allow(clippy::unused_unit)]
use serde::Deserialize;
use polars::prelude::*;
use polars_arrow::array::MutablePlString;
use polars_core::utils::align_chunks_binary;
use pyo3_polars::derive::polars_expr;

use reverse_geocoder::ReverseGeocoder;

const BASE_32: &'static str = "0123456789bcdefghjkmnpqrstuvwxyz";

#[polars_expr(output_type=String)]
fn reverse_geocode(inputs: &[Series]) -> PolarsResult<Series> {
    let lat = inputs[0].f64()?;
    let lon = inputs[1].f64()?;
    let geocoder = ReverseGeocoder::new();

    let (lhs, rhs) = align_chunks_binary(lat, lon);
    let chunks = lhs
        .downcast_iter()
        .zip(rhs.downcast_iter())
        .map(|(lat_arr, lon_arr)| {
            let mut mutarr = MutablePlString::with_capacity(lat_arr.len());

            for (lat_opt_val, lon_opt_val) in lat_arr.iter().zip(lon_arr.iter()) {
                match (lat_opt_val, lon_opt_val) {
                    (Some(lat_val), Some(lon_val)) => {
                        let res = &geocoder.search((*lat_val, *lon_val)).record.name;
                        mutarr.push(Some(res))
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
struct GeoHashKwargs {
    precision: usize,
}

#[polars_expr(output_type=String)]
fn geohash(inputs: &[Series], kwargs: GeoHashKwargs) -> PolarsResult<Series> {
    let lat = inputs[0].f64()?;
    let lon = inputs[1].f64()?;
    let precision = kwargs.precision;
    let (lhs, rhs) = align_chunks_binary(lat, lon);
    let chunks = lhs
        .downcast_iter()
        .zip(rhs.downcast_iter())
        .map(|(lat_arr, lon_arr)| {
            let mut buf = String::with_capacity(precision);
            let mut mutarr = MutablePlString::with_capacity(lat_arr.len());

            for (lat_opt_val, lon_opt_val) in lat_arr.iter().zip(lon_arr.iter()) {
                match (lat_opt_val, lon_opt_val) {
                    (Some(lat_val), Some(lon_val)) => {
                        buf.clear();
                        impl_geohash(*lat_val, *lon_val, precision, &mut buf);
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

fn impl_geohash(latitude: f64, longitude: f64, precision: usize, buffer: &mut String) {
    let mut lat_interval = (-90.0, 90.0);
    let mut lon_interval = (-180.0, 180.0);
    let bits = [16, 8, 4, 2, 1];
    let mut bit = 0;
    let mut ch = 0;
    let mut even = true;
    while buffer.len() < precision {
        if even {
            let mid = (lon_interval.0 + lon_interval.1) / 2.0;
            if longitude > mid {
                ch |= bits[bit];
                lon_interval = (mid, lon_interval.1);
            } else {
                lon_interval = (lon_interval.0, mid);
            }
        } else {
            let mid = (lat_interval.0 + lat_interval.1) / 2.0;
            if latitude > mid {
                ch |= bits[bit];
                lat_interval = (mid, lat_interval.1);
            } else {
                lat_interval = (lat_interval.0, mid);
            }
        }
        even = !even;
        if bit < 4 {
            bit += 1;
        } else {
            buffer.push_str(&BASE_32[ch as usize..(ch + 1) as usize]);
            bit = 0;
            ch = 0;
        }
    }
}
