use crate::metric::Metric;

pub trait Algorithm {
    fn eval<R>(&mut self, metric: &impl Metric<R>, data: &[R]) -> Vec<usize>;
}

pub struct SomeAlgo {}

impl Algorithm for SomeAlgo {
    fn eval<R>(&mut self, metric: &impl Metric<R>, data: &[R]) -> Vec<usize> {
        data.iter()
            .zip(data.iter().rev())
            .map(|(r1, r2)| metric.dist(r1, r2) as usize)
            .collect::<Vec<usize>>()
    }
}


