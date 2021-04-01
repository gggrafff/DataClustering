pub mod row;

pub trait Metric<R> {
    fn distance(&self, left: &R, right: &R) -> f64;
}

impl<R, F: Fn(&R, &R) -> f64> Metric<R> for F {
    fn distance(&self, left: &R, right: &R) -> f64 {
        self(left, right)
    }
}
