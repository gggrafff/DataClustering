pub mod row;

pub trait Metric {
    type Record;

    fn distance(&self, left: &Self::Record, right: &Self::Record) -> f64;
}
