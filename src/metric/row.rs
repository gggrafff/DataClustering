use std::marker::PhantomData;

use crate::metric::Metric;
use crate::record::RowRecord;
use crate::utils::Difference;

#[derive(Default, Copy, Clone, Debug)]
pub struct EucludeRowMetric<T>(PhantomData<T>);

impl<C: Difference> Metric for EucludeRowMetric<C> {
    type Record = RowRecord<C>;

    fn distance(&self, left: &Self::Record, right: &Self::Record) -> f64 {
            left.iter()
            .zip(right.iter())
            .map(|(l, r)| l.diff(r).powi(2))
            .sum::<f64>()
            .sqrt()
    }
}

#[derive(Default, Copy, Clone, Debug)]
pub struct ManhattanRowMetric<T>(PhantomData<T>);

impl<C: Difference> Metric for ManhattanRowMetric<C> {
    type Record = RowRecord<C>;

    fn distance(&self, left: &Self::Record, right: &Self::Record) -> f64 {
        left.iter()
            .zip(right.iter())
            .map(|(l, r)| l.diff(r).abs())
            .sum::<f64>()
            .sqrt()
    }
}
