pub mod algo;
pub mod metric;
pub mod record;
pub mod utils;

#[cfg(test)]
mod tests {
    use crate::algo::{Algorithm, SomeAlgo};
    use crate::metric::row;
    use crate::record::RowRecord;

    #[test]
    fn test_works() {
        let cells = vec![1f64, 2f64, 3f64, 4f64, 5f64, 6f64];
        let mut records = vec![RowRecord::new(cells); 6];

        for (i, r) in records.iter_mut().enumerate() {
            *r.cell_mut(i) += i as f64;
        }

        let mut algo = SomeAlgo {};

        let result = algo.eval(&records, row::euclide_row_metric);
        println!("Euclide result: {:?}", result);

        let result = algo.eval(&records, row::manhattan_row_metric);
        println!("Manhatten result: {:?}", result);
    }
}
