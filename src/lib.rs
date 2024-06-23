use crate::iter::ExtendedMathsHeatplotIter;
use core::ops::Range;
use plotters::prelude::*;

pub mod f64;
pub mod iter;

def_linear_colormap! {
    WarmHeatMap,
    RGBColor,
    "Custom color map for warm heatmap plots",
    BLACK,
    RED,
    YELLOW,
    WHITE
}
def_linear_colormap! {
    ColdHeatMap,
    RGBColor,
    "Custom color map for cold heatmap plots",
    BLACK,
    BLUE,
    CYAN,
    WHITE
}

pub struct Heatplot2D<'a> {
    pub title: Option<&'a str>,
    pub out_file_name: &'a str,
    /// Fn that maps the normalized computed value to a RGBColor
    pub color_fn: Box<dyn Fn(f64) -> RGBColor>,
    /// Fn that takes in x range, y range and their samples
    /// Returns the iterator of the coordinates and normalized computed value
    pub compute_fn: Box<
        dyn Fn(Range<f64>, Range<f64>, (usize, usize)) -> Box<dyn Iterator<Item = (f64, f64, f64)>>,
    >,
    /// Resolution in px
    pub res: (u32, u32),
    pub x_range: Range<f64>,
    pub y_range: Range<f64>,
    pub margin: i32,
    pub x_label_area_size: i32,
    pub y_label_area_size: i32,
    pub text_style: TextStyle<'a>,
    pub bg: &'a RGBColor,
    pub fg: &'a RGBColor,
}

impl std::default::Default for Heatplot2D<'_> {
    fn default() -> Self {
        Self {
            title: None,
            out_file_name: "out.png",
            color_fn: Box::new(|x: f64| WarmHeatMap.get_color(x)),
            compute_fn: Box::new(|xs: Range<f64>, ys: Range<f64>, samples: (usize, usize)| {
                let step = (
                    (xs.end - xs.start) / samples.0 as f64,
                    (ys.end - ys.start) / samples.1 as f64,
                );
                Box::new(
                    (0..(samples.0 * samples.1))
                        .map(move |k| {
                            let x = xs.start + step.0 * (k % samples.0) as f64;
                            let y = ys.start + step.1 * (k / samples.0) as f64;
                            (x, y, x)
                        })
                        .norm(),
                )
            }),
            res: (800, 800),
            x_range: 0.0..10.0,
            y_range: 0.0..10.0,
            margin: 20,
            x_label_area_size: 30,
            y_label_area_size: 30,
            text_style: ("sans-serif", 24).into_font().color(&WHITE),
            bg: &BLACK,
            fg: &WHITE,
        }
    }
}

impl Heatplot2D<'_> {
    pub fn generate(&self) -> Result<(), Box<dyn std::error::Error>> {
        let Self {
            title,
            out_file_name,
            color_fn,
            compute_fn,
            res,
            x_range,
            y_range,
            margin,
            x_label_area_size,
            y_label_area_size,
            text_style,
            bg,
            fg,
        } = self;

        println!("GENERATING HEATPLOT {title:?}...");
        let root = BitMapBackend::new(out_file_name, *res).into_drawing_area();

        root.fill(bg)?;

        let mut builder = ChartBuilder::on(&root);

        let mut chart = if let Some(title) = title {
            builder
                .caption(title, text_style.clone())
                .margin(*margin)
                .x_label_area_size(*x_label_area_size)
                .y_label_area_size(*y_label_area_size)
        } else {
            builder
                .margin(*margin)
                .x_label_area_size(*x_label_area_size)
                .y_label_area_size(*y_label_area_size)
        }
        .build_cartesian_2d(x_range.clone(), y_range.clone())?;

        chart
            .configure_mesh()
            .x_labels(5)
            .y_labels(5)
            .label_style(text_style.clone())
            .axis_style(fg)
            .disable_x_mesh()
            .disable_y_mesh()
            .draw()?;

        let plotting_area = chart.plotting_area();
        let range = plotting_area.get_pixel_range();
        let (pw, ph) = (range.0.end - range.0.start, range.1.end - range.1.start);

        for (x, y, c) in compute_fn(chart.x_range(), chart.y_range(), (pw as usize, ph as usize)) {
            plotting_area.draw_pixel((x, y), &color_fn(c))?;
        }

        root.present()?;
        println!("DONE. Saved to {}", out_file_name);

        Ok(())
    }
}
