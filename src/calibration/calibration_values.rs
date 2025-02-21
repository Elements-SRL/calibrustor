use std::collections::HashMap;

#[derive(Clone, Debug)]
pub enum CalibrationValues {
    Int(HashMap<usize, i32>),
    Float(HashMap<usize, f64>),
}

impl CalibrationValues {
    pub fn new_float(m: HashMap<usize, f64>) -> Self {
        Self::Float(m)
    }
}
