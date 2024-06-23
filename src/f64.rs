pub trait ExtendedMathsF64
where
    Self: Sized,
{
    fn inv(&self) -> Self;
    fn gaussian(&self, mu: Self, sigma: Self) -> Self;
    fn gaussian_std(&self) -> Self;
    fn norm(&self, min_max: (Self, Self)) -> Self;
}

impl ExtendedMathsF64 for f64 {
    fn inv(&self) -> Self {
        1.0 / self
    }
    fn gaussian(&self, mu: Self, sigma: Self) -> Self {
        (2.0 * std::f64::consts::PI * sigma.powi(2)).sqrt().inv()
            * (-(self - mu).powi(2) / (2.0 * sigma.powi(2))).exp()
    }
    fn gaussian_std(&self) -> Self {
        self.gaussian(0.0, 1.0)
    }
    fn norm(&self, min_max: (Self, Self)) -> Self {
        (self - min_max.0) / (min_max.1 - min_max.0)
    }
}
