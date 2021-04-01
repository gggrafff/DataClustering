use crate::record::Record;
pub mod euclide;

pub trait Metric<R> {
    fn dist(&self, r1: &R, r2: &R) -> f64;
}

impl<F: Fn(&Record, &Record) -> f64> Metric<Record> for F {
    fn dist(&self, r1: &Record, r2: &Record) -> f64 {
        self(r1, r2)
    }
}
