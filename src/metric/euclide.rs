use crate::record::Record;

pub fn euclide(r1: &Record, r2: &Record) -> f64 {
    let sum: f64 = r1
        .vals()
        .iter()
        .zip(r2.vals().iter())
        .map(|(v1, v2)| (v1 - v2) * (v1 - v2))
        .sum();
    sum.sqrt()
}
