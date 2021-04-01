use std::marker::PhantomData;

use crate::record::{Difference, Record};

#[derive(Clone, Debug)]
pub struct RowRecord<M: RowMetric> {
    cells: Vec<M::Cell>,
    metric: M,
}

impl<M: RowMetric> RowRecord<M> {
    pub fn new(cells: Vec<M::Cell>, metric: M) -> Self {
        Self { cells, metric }
    }

    pub fn cell(&self, index: usize) -> &M::Cell {
        &self.cells[index]
    }

    pub fn cell_mut(&mut self, index: usize) -> &mut M::Cell {
        &mut self.cells[index]
    }

    pub fn metric(&self) -> &M {
        &self.metric
    }

    pub fn metric_mut(&mut self) -> &M {
        &mut self.metric
    }

    pub fn into_inner(self) -> (Vec<M::Cell>, M) {
        (self.cells, self.metric)
    }

    pub fn with_metric<NewM: RowMetric<Cell = M::Cell>>(self, new_metric: NewM) -> RowRecord<NewM> {
        let cells = self.cells;
        RowRecord {
            metric: new_metric,
            cells,
        }
    }
}

impl<M: RowMetric> Record for RowRecord<M> {
    fn distance(&self, other: &Self) -> f64 {
        self.metric.distance(&self.cells, &other.cells)
    }
}

pub trait RowMetric {
    type Cell;

    fn distance(&self, left: &[Self::Cell], right: &[Self::Cell]) -> f64;
}

#[derive(Default, Copy, Clone, Debug)]
pub struct EucludeRowMetric<T>(PhantomData<T>);

impl<C: Difference> RowMetric for EucludeRowMetric<C> {
    type Cell = C;

    fn distance(&self, left: &[Self::Cell], right: &[Self::Cell]) -> f64 {
        left.iter()
            .zip(right.iter())
            .map(|(l, r)| l.diff(r).powi(2))
            .sum::<f64>()
            .sqrt()
    }
}

#[derive(Default, Copy, Clone, Debug)]
pub struct ManhattanRowMetric<T>(PhantomData<T>);

impl<C: Difference> RowMetric for ManhattanRowMetric<C> {
    type Cell = C;

    fn distance(&self, left: &[Self::Cell], right: &[Self::Cell]) -> f64 {
        left.iter()
            .zip(right.iter())
            .map(|(l, r)| l.diff(r).abs())
            .sum::<f64>()
            .sqrt()
    }
}
