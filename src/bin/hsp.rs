use num_traits::Inv;
use plotters::prelude::*;
use std::ops::Range;

const BG: &RGBColor = &BLACK;
const FG: &RGBColor = &WHITE;

const R: f64 = 3.5;
const V: f64 = 0.999;

const X_RANGE: Range<f64> = -R..R;
const Y_RANGE: Range<f64> = -R..R;

def_linear_colormap! {
    CustomMap,
    RGBColor,
    "Custom color map",
    BLACK,
    RED,
    YELLOW,
    WHITE
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_filename = format!("hsp-v{V}.png");
    println!("Generating HSP...");
    let root = BitMapBackend::new(&out_filename, (800, 800)).into_drawing_area();

    root.fill(&BG)?;

    let mut chart = ChartBuilder::on(&root)
        .caption(
            format!("Probability Density of HSP     v = {V}    Φ = x^2"),
            ("sans-serif", 24).into_font().color(&FG),
        )
        .margin(20)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(X_RANGE, Y_RANGE)?;

    chart
        .configure_mesh()
        .x_labels(5)
        .y_labels(5)
        .label_style(("sans-serif", 24).into_font().color(&FG))
        .axis_style(&FG)
        .disable_x_mesh()
        .disable_y_mesh()
        .draw()?;

    let plotting_area = chart.plotting_area();

    let range = plotting_area.get_pixel_range();

    let (pw, ph) = (range.0.end - range.0.start, range.1.end - range.1.start);

    for (x, y, c) in photon_prob_density(
        chart.x_range(),
        chart.y_range(),
        (pw as usize, ph as usize),
        |x: f64| x.powi(2),
        V,
    ) {
        plotting_area.draw_pixel((x, y), &CustomMap::get_color(c))?;
    }

    root.present().unwrap();
    println!("DONE. Saved to {}", out_filename);

    Ok(())
}

fn gaussian(x: f64) -> f64 {
    (2.0 * std::f64::consts::PI).sqrt().inv() * (-0.5 * x.powi(2)).exp()
}

fn normalize(x: f64, min: f64, max: f64) -> f64 {
    (x - min) / (max - min)
}

fn photon_prob_density(
    xs: Range<f64>,
    xps: Range<f64>,
    samples: (usize, usize),
    phi: impl Fn(f64) -> f64 + std::clone::Clone,
    v: f64,
) -> impl Iterator<Item = (f64, f64, f64)> {
    let step = (
        (xs.end - xs.start) / samples.0 as f64,
        (xps.end - xps.start) / samples.1 as f64,
    );
    let i = (0..(samples.0 * samples.1)).map(move |k| {
        let x = xs.start + step.0 * (k % samples.0) as f64;
        let xp = xps.start + step.1 * (k / samples.0) as f64;
        let psi_u = gaussian;
        let psi_r = gaussian;
        let psi_u_sq = |x: f64| psi_u(x).powi(2);
        let psi_r_sq = |x: f64| psi_r(x).powi(2);
        let p = 0.25 * ((psi_u_sq(x) * psi_r_sq(xp)) + (psi_r_sq(x) * psi_u_sq(xp)))
            - (v * 0.5 * psi_u(x) * psi_u(xp) * psi_r(x) * psi_r(xp) * (phi(x) - phi(xp)).cos());
        (x, xp, p)
    });

    let (min, max) = i
        .clone()
        .fold((f64::INFINITY, 0_f64), |(p_min, p_max), (_, _, pb)| {
            (p_min.min(pb), p_max.max(pb))
        });

    i.map(move |(x, xp, p)| (x, xp, normalize(p, min, max)))
}

#[test]
fn entry_point() {
    main().unwrap()
}
