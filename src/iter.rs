use crate::f64::ExtendedMathsF64;

pub trait ExtendedMathsIter: Iterator<Item = (f64, f64, f64)> + Clone {
    fn norm(self) -> impl Iterator<Item = (f64, f64, f64)> {
        let min_max = self
            .clone()
            .fold((f64::INFINITY, 0_f64), |(pmin, pmax), (_, _, p)| {
                (pmin.min(p), pmax.max(p))
            });
        self.map(move |(x, xp, p)| (x, xp, p.norm(min_max)))
    }
}

impl<I> ExtendedMathsIter for I where I: Iterator<Item = (f64, f64, f64)> + Clone {}
