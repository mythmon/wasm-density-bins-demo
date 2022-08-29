#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![allow(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_precision_loss
)]

//! Based on <https://observablehq.com/@yurivish/zig-density-plot-prototype>

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[must_use]
pub fn density_1d_core(xs: &[f64], bin_count: usize) -> Vec<f64> {
    let mut bins = vec![0_f64; bin_count];
    let (min, max) = extent(xs).unwrap_or((0_f64, 1_f64));
    let scale = bin_scale(bin_count, min, max);
    for x in xs {
        bins[bin_index(*x, min, scale)] += 1_f64;
    }
    bins
}

fn extent<T>(xs: &[T]) -> Option<(T, T)>
where
    T: PartialOrd + Copy,
{
    if let Some(v) = xs.get(0) {
        let mut min = *v;
        let mut max = *v;
        for x in xs.iter().skip(1) {
            if *x < min {
                min = *x
            } else if *x > max {
                max = *x
            }
        }
        Some((min, max))
    } else {
        None
    }
}

/// Scale factor from `[min, max]` to `[0..bins.len - eps]`. The epsilon ensures
/// that maximum point gets mapped to the largest bin (instead of beyond it).
fn bin_scale(len: usize, min: f64, max: f64) -> f64 {
    const EPS: f64 = 1e-7_f64;
    (len as f64 - EPS) / (max - min)
}

/// Given a value, the min extent, and a bin scale factor, compute the bin index.
fn bin_index(value: f64, min: f64, scale: f64) -> usize {
    ((value - min) * scale) as usize
}

#[cfg(test)]
mod tests {
    use crate::{bin_index, bin_scale, extent};

    #[test]
    fn test_bin_index() {
        let (min, max) = (0_f64, 100_f64);
        let scale = bin_scale(100, min, max);
        let index = bin_index(50_f64, min, scale);
        assert_eq!(index, 49);
    }

    #[test]
    fn test_extent() {
        let (min, max) = extent(&vec![0, 3, 6, 9, 2, 5, 8, 1, 4, 7]).unwrap();
        assert_eq!(min, 0);
        assert_eq!(max, 9);
    }
}
