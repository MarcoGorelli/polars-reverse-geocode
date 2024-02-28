#![allow(clippy::unused_unit)]
use polars::prelude::*;
use polars_arrow::array::MutablePlString;
use polars_core::utils::align_chunks_binary;
use pyo3_polars::derive::polars_expr;
use std::fmt::Write;
use polars::prelude::arity::binary_elementwise_values;

use reverse_geocoder::ReverseGeocoder;

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
            let mut buf = String::new();
            let mut mutarr = MutablePlString::with_capacity(lat_arr.len());

            for (lat_opt_val, lon_opt_val) in lat_arr.iter().zip(lon_arr.iter()) {
                match (lat_opt_val, lon_opt_val) {
                    (Some(lat_val), Some(lon_val)) => {
                        let res = &geocoder.search((*lat_val, *lon_val)).record.name;
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


#[polars_expr(output_type=String)]
fn geohash(inputs: &[Series]) -> PolarsResult<Series> {
    let lhs = inputs[0].f64()?;
    let rhs = inputs[1].f64()?;
    let (lhs, rhs) = align_chunks_binary(lat, lon);
    let chunks = lhs
        .downcast_iter()
        .zip(rhs.downcast_iter())
        .map(|(lat_arr, lon_arr)| {
            let mut buf = String::new();
            let mut mutarr = MutablePlString::with_capacity(lat_arr.len());

            for (lat_opt_val, lon_opt_val) in lat_arr.iter().zip(lon_arr.iter()) {
                match (lat_opt_val, lon_opt_val) {
                    (Some(lat_val), Some(lon_val)) => {
                        let res = &geocoder.search((*lat_val, *lon_val)).record.name;
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

// def encode(latitude, longitude, precision=6):
//     lat_interval, lon_interval = (-90.0, 90.0), (-180.0, 180.0)
//     geohash = []
//     bits = [ 16, 8, 4, 2, 1 ]
//     bit = 0
//     ch = 0
//     even = True
//     while len(geohash) < precision:
//         if even:
//             mid = (lon_interval[0] + lon_interval[1]) / 2
//             if longitude > mid:
//                 ch |= bits[bit]
//                 lon_interval = (mid, lon_interval[1])
//             else:
//                 lon_interval = (lon_interval[0], mid)
//         else:
//             mid = (lat_interval[0] + lat_interval[1]) / 2
//             if latitude > mid:
//                 ch |= bits[bit]
//                 lat_interval = (mid, lat_interval[1])
//             else:
//                 lat_interval = (lat_interval[0], mid)
//         even = not even
//         if bit < 4:
//             bit += 1
//         else:
//             geohash += __base32[ch]
//             bit = 0
//             ch = 0
//     return ''.join(geohash)
