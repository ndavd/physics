use physics::f64::*;
use physics::iter::*;
use physics::*;
use std::ops::Range;

const V: f64 = 1.0;
const MU: f64 = 0.0;
const SIGMA: f64 = 1.5;

const R: f64 = 3.5;
const X_RANGE: Range<f64> = -R..R;
const Y_RANGE: Range<f64> = -R..R;

fn hsp_prob_density(
    xs: Range<f64>,
    xps: Range<f64>,
    samples: (usize, usize),
    phi: impl Fn(f64) -> f64 + std::clone::Clone,
    psi_u: impl Fn(f64) -> f64 + std::clone::Clone + 'static,
    psi_r: impl Fn(f64) -> f64 + std::clone::Clone + 'static,
) -> impl Iterator<Item = (f64, f64, f64)> {
    let step = (
        (xs.end - xs.start) / samples.0 as f64,
        (xps.end - xps.start) / samples.1 as f64,
    );
    (0..(samples.0 * samples.1))
        .map(move |k| {
            let x = xs.start + step.0 * (k % samples.0) as f64;
            let xp = xps.start + step.1 * (k / samples.0) as f64;
            (
                x,
                xp,
                0.25 * ((psi_u(x).powi(2) * psi_r(xp).powi(2))
                    + (psi_r(x).powi(2) * psi_u(xp).powi(2)))
                    - (V * 0.5
                        * psi_u(x)
                        * psi_u(xp)
                        * psi_r(x)
                        * psi_r(xp)
                        * (phi(x) - phi(xp)).cos()),
            )
        })
        .norm()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (phi_str, phi) = ("x^2", |x: f64| x.powi(2));
    let psi_u = move |x: f64| x.gaussian(MU, SIGMA);
    let psi_r = psi_u;
    Heatplot2D {
        title: format!(
            "HSP Probability Density (V = {V}, Φ = {phi_str}, Ψu = Ψr = N({MU}, {}))",
            SIGMA.powi(2)
        )
        .as_str()
        .into(),
        out_file_name: "hsp.png",
        compute_fn: Box::new(
            move |xs: Range<f64>, ys: Range<f64>, samples: (usize, usize)| {
                Box::new(hsp_prob_density(xs, ys, samples, phi, psi_u, psi_r))
            },
        ),
        x_range: X_RANGE,
        y_range: Y_RANGE,
        ..Default::default()
    }
    .generate()
}
