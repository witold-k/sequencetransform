// SPDX-License-Identifier: Apache-2.0
//! Shared direct one-sided DFT for trailing spectral features.
use std::f64::consts::PI;

#[derive(Copy, Clone)]
pub(crate) enum Feature { Entropy, Centroid, Change }

pub(crate) fn spectrum(window: &[f64]) -> Option<Vec<f64>> {
    let n = window.len();
    if n < 4 { return None; }
    // Center first; Hann taper suppresses spectral leakage.
    let mean = window.iter().sum::<f64>() / n as f64;
    let mut tapered = Vec::with_capacity(n);
    for (i, &x) in window.iter().enumerate() {
        let w = 0.5 - 0.5 * (2.0 * PI * i as f64 / (n - 1) as f64).cos();
        tapered.push((x - mean) * w);
    }
    let mut power = Vec::with_capacity(n / 2);
    // DC excluded; positive bins through Nyquist (if even).
    for k in 1..=n / 2 {
        let mut re = 0.0;
        let mut im = 0.0;
        for (i, &x) in tapered.iter().enumerate() {
            let angle = 2.0 * PI * (k as f64) * (i as f64) / n as f64;
            re += x * angle.cos();
            im -= x * angle.sin();
        }
        // One-sided energy: double interior bins, not Nyquist.
        let weight = if n.is_multiple_of(2) && k == n / 2 { 1.0 } else { 2.0 };
        power.push(weight * (re * re + im * im));
    }
    let total: f64 = power.iter().sum();
    if !total.is_finite() || total <= 0.0 { return None; }
    for p in &mut power { *p /= total; }
    Some(power)
}

pub(crate) fn feature(window: &[f64], kind: Feature) -> Option<f64> {
    let n = window.len();
    match kind {
        Feature::Entropy => {
            let p = spectrum(window)?;
            if p.len() < 2 { return None; }
            let h: f64 = p.iter().filter(|&&v| v > 0.0).map(|&v| -v * v.ln()).sum();
            Some((h / (p.len() as f64).ln()).clamp(0.0, 1.0))
        }
        Feature::Centroid => {
            let p = spectrum(window)?;
            Some(p.iter().enumerate().map(|(i, &v)| {
                v * (i + 1) as f64 / (n as f64 / 2.0)
            }).sum::<f64>().clamp(0.0, 1.0))
        }
        Feature::Change => {
            if !n.is_multiple_of(2) || n < 8 { return None; }
            let half = n / 2;
            let a = spectrum(&window[..half])?;
            let b = spectrum(&window[half..])?;
            Some((0.5 * a.iter().zip(&b).map(|(x,y)| (x-y).abs()).sum::<f64>()).clamp(0.0, 1.0))
        }
    }
}
