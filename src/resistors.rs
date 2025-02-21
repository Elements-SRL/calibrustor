use ndarray::ArcArray1;

#[derive(Clone)]
pub enum Resistors {
    ModelCell(ModelCell),
    ModifiedModelCell(ModelCell),
    Internal(ModelCell),
}

impl Resistors {
    pub fn get_name(&self) -> String {
        match self {
            Resistors::ModelCell(mc) => mc.get_name(),
            Resistors::ModifiedModelCell(mc) => mc.get_name(),
            Resistors::Internal(mc) => mc.get_name(),
        }
    }
    pub fn mean(&self) -> f64 {
        match self {
            Resistors::ModelCell(mc) => mc.mean(),
            Resistors::ModifiedModelCell(mc) => mc.mean(),
            Resistors::Internal(mc) => mc.mean(),
        }
    }
    pub fn get_values(&self) -> ArcArray1<f64> {
        match self {
            Resistors::ModelCell(mc) => mc.get_values(),
            Resistors::ModifiedModelCell(mc) => mc.get_values(),
            Resistors::Internal(mc) => mc.get_values(),
        }
    }
}

#[derive(Clone)]
pub struct ModelCell {
    name: String,
    values: ArcArray1<f64>,
}

impl ModelCell {
    pub fn new(name: &str, values: ArcArray1<f64>) -> Self {
        Self {
            name: name.to_string(),
            values,
        }
    }
    pub fn get_name(&self) -> String {
        self.name.clone()
    }
    pub fn mean(&self) -> f64 {
        self.values.mean().unwrap()
    }
    pub fn get_values(&self) -> ArcArray1<f64> {
        self.values.clone()
    }
}
