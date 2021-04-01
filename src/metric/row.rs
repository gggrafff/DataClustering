use crate::record::RowRecord;
use crate::utils::Difference;

pub fn euclide_row_metric<C: Difference>(left: &RowRecord<C>, right: &RowRecord<C>) -> f64 {
    left.iter()
        .zip(right.iter())
        .map(|(l, r)| l.diff(r).powi(2))
        .sum::<f64>()
        .sqrt()
}

pub fn manhattan_row_metric<C: Difference>(left: &RowRecord<C>, right: &RowRecord<C>) -> f64 {
    left.iter()
        .zip(right.iter())
        .map(|(l, r)| l.diff(r).abs())
        .sum::<f64>()
        .sqrt()
}