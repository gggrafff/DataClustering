pub trait Difference {
    fn diff(&self, other: &Self) -> f64;
}

impl Difference for i8 {
    fn diff(&self, to: &Self) -> f64 {
        (self - to) as f64
    }
}

impl Difference for i16 {
    fn diff(&self, to: &Self) -> f64 {
        (self - to) as f64
    }
}

impl Difference for i32 {
    fn diff(&self, to: &Self) -> f64 {
        (self - to) as f64
    }
}

impl Difference for i64 {
    fn diff(&self, to: &Self) -> f64 {
        (self - to) as f64
    }
}

impl Difference for f32 {
    fn diff(&self, to: &Self) -> f64 {
        (self - to) as f64
    }
}

impl Difference for f64 {
    fn diff(&self, to: &Self) -> f64 {
        self - to
    }
}
