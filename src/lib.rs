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

const BG: &RGBColor = &BLACK;
const FG: &RGBColor = &WHITE;

pub struct Heatplot2D {
    title: String,
    out_file_name: String,
    color_fn: Box<dyn Fn(f64) -> RGBColor>,
    /// Fn that takes in xs, ys and samples and returns the iterator of the coordinates and computed value
    compute_fn: Box<
        dyn Fn(Range<f64>, Range<f64>, (usize, usize)) -> Box<dyn Iterator<Item = (f64, f64, f64)>>,
    >,
    res: (u32, u32),
    xr: Range<f64>,
    yr: Range<f64>,
}

impl Heatplot2D {
    pub fn new(
        title: String,
        out_file_name: String,
        color_fn: impl Fn(f64) -> RGBColor + 'static,
        compute_fn: impl Fn(Range<f64>, Range<f64>, (usize, usize)) -> Box<dyn Iterator<Item = (f64, f64, f64)>>
            + 'static,
        res: (u32, u32),
        xr: Range<f64>,
        yr: Range<f64>,
    ) -> Self {
        Self {
            title,
            out_file_name,
            color_fn: Box::new(color_fn),
            compute_fn: Box::new(compute_fn),
            res,
            xr,
            yr,
        }
    }
    pub fn generate(&self) -> Result<(), Box<dyn std::error::Error>> {
        let Self {
            title,
            out_file_name,
            color_fn,
            compute_fn,
            res,
            xr,
            yr,
        } = self;

        println!("GENERATING HEATPLOT {title:?}...");
        let root = BitMapBackend::new(out_file_name, *res).into_drawing_area();

        root.fill(&BG)?;

        let mut chart = ChartBuilder::on(&root)
            .caption(title, ("sans-serif", 24).into_font().color(&FG))
            .margin(20)
            .x_label_area_size(30)
            .y_label_area_size(30)
            .build_cartesian_2d(xr.clone(), yr.clone())?;

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

        for (x, y, c) in compute_fn(chart.x_range(), chart.y_range(), (pw as usize, ph as usize)) {
            plotting_area.draw_pixel((x, y), &color_fn(c))?;
        }

        root.present()?;
        println!("DONE. Saved to {}", out_file_name);

        Ok(())
    }
}
