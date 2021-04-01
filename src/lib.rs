pub mod algo;
pub mod metric;
pub mod record;

#[cfg(test)]
mod tests {
    use crate::algo::{Algorithm, SomeAlgo};
    use crate::metric::euclide;
    use crate::record::Record;

    #[test]
    fn it_works() {
        let mut rec = vec![Record::new(vec![1f64, 2f64, 3f64, 4f64, 5f64]); 10];

        for (i, r) in rec.iter_mut().enumerate() {
            r.vals_mut()[0] += i as f64;
        }

        let mut algo = SomeAlgo {};
        let result = algo.eval(&euclide::euclide, &rec);
        println!("Result: {:?}", result)
    }
}
