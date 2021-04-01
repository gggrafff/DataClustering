use crate::record::Record;

pub trait Algorithm {
    fn eval<R: Record>(&mut self, data: &[R]) -> Vec<usize>;
}

pub struct SomeAlgo {}

impl Algorithm for SomeAlgo {
    fn eval<R: Record>(&mut self, data: &[R]) -> Vec<usize> {
        data.iter()
            .zip(data.iter().rev())
            .map(|(r1, r2)| r1.distance(r2) as usize)
            .collect::<Vec<usize>>()
    }
}
