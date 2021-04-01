#[derive(Clone)]
pub struct Record {
    vals: Vec<f64>,
}

impl Record {
    pub fn new(vals: Vec<f64>) -> Self {
        Self { vals }
    }

    pub fn vals(&self) -> &[f64] {
        &self.vals
    }

    pub fn vals_mut(&mut self) -> &mut [f64] {
        &mut self.vals
    }
}
