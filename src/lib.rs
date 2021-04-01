pub mod algo;
pub mod record;

#[cfg(test)]
mod tests {
    use crate::algo::{Algorithm, SomeAlgo};
    use crate::record::row::{EucludeRowMetric, ManhattanRowMetric, RowRecord};

    #[test]
    fn test_works() {
        let cells = vec![1f64, 2f64, 3f64, 4f64, 5f64, 6f64];
        let euclid_metric = EucludeRowMetric::default();
        let mut records = vec![RowRecord::new(cells, euclid_metric); 6];

        for (i, r) in records.iter_mut().enumerate() {
            *r.cell_mut(i) += i as f64;
        }

        let mut algo = SomeAlgo {};
        let result = algo.eval(&records);
        println!("Euclide result: {:?}", result);

        let man_metric = ManhattanRowMetric::default();
        let records: Vec<_> = records
            .into_iter()
            .map(|r| r.with_metric(man_metric))
            .collect();
        let result = algo.eval(&records);
        println!("Manhatten result: {:?}", result);
    }
}
