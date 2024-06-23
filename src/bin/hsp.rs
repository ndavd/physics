use physics::f64::*;
use physics::iter::*;
use physics::*;
use plotters::prelude::*;
use std::ops::Range;

const R: f64 = 3.5;
const V: f64 = 1.0;
const MU: f64 = 0.0;
const SIGMA: f64 = 1.5;

const X_RANGE: Range<f64> = -R..R;
const Y_RANGE: Range<f64> = -R..R;

const RES: (u32, u32) = (800, 800);
const OUT_FILENAME: &str = "hsp.png";

fn hsp_prob_density(
    xs: Range<f64>,
    xps: Range<f64>,
    samples: (usize, usize),
    phi: impl Fn(f64) -> f64 + std::clone::Clone,
) -> impl Iterator<Item = (f64, f64, f64)> {
    let step = (
        (xs.end - xs.start) / samples.0 as f64,
        (xps.end - xps.start) / samples.1 as f64,
    );
    let psi_u = move |x: f64| x.gaussian(MU, SIGMA);
    let psi_r = psi_u;
    let psi_u_sq = move |x: f64| psi_u(x).powi(2);
    let psi_r_sq = move |x: f64| psi_r(x).powi(2);
    (0..(samples.0 * samples.1))
        .map(move |k| {
            let x = xs.start + step.0 * (k % samples.0) as f64;
            let xp = xps.start + step.1 * (k / samples.0) as f64;
            let p = 0.25 * ((psi_u_sq(x) * psi_r_sq(xp)) + (psi_r_sq(x) * psi_u_sq(xp)))
                - (V * 0.5
                    * psi_u(x)
                    * psi_u(xp)
                    * psi_r(x)
                    * psi_r(xp)
                    * (phi(x) - phi(xp)).cos());
            (x, xp, p)
        })
        .norm()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    Heatplot2D::new(
        format!(
            "HSP Probability Density (V = {V}, Φ = x^2, Ψu = Ψr = N({MU}, {}))",
            SIGMA.powi(2)
        ),
        OUT_FILENAME.to_string(),
        |x: f64| WarmHeatMap.get_color(x),
        |xs: Range<f64>, xps: Range<f64>, samples: (usize, usize)| {
            Box::new(hsp_prob_density(xs, xps, samples, |x| x * x))
        },
        RES,
        X_RANGE,
        Y_RANGE,
    )
    .generate()
}
