use crate::metric::Metric;

pub trait Algorithm {
    fn eval<R, M: Metric<Record = R>>(&mut self, data: &[R], metric: M) -> Vec<usize>;
}

pub struct SomeAlgo {}

impl Algorithm for SomeAlgo {
    fn eval<R, M: Metric<Record = R>>(&mut self, data: &[R], metric: M) -> Vec<usize> {
        data.iter()
            .zip(data.iter().rev())
            .map(|(r1, r2)| metric.distance(r1, r2) as usize)
            .collect::<Vec<usize>>()
    }
}
