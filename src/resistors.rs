use ndarray::ArcArray1;

pub trait Resistors: Clone {
    fn get_name(&self) -> String;
    fn mean(&self) -> f64;
    fn get_values(&self) -> ArcArray1<f64>;
}

#[derive(Clone)]
pub struct RegularModelCell(ModelCell);

impl RegularModelCell {
    pub fn new(name: &str, values: ArcArray1<f64>) -> Self {
        Self(ModelCell::new(name, values))
    }
}

impl Resistors for RegularModelCell {
    fn get_name(&self) -> String {
        self.0.get_name()
    }
    fn mean(&self) -> f64 {
        self.0.mean()
    }
    fn get_values(&self) -> ArcArray1<f64> {
        self.0.get_values()
    }
}

#[derive(Clone)]
pub struct ModifiedModelCell(ModelCell);

impl ModifiedModelCell {
    pub fn new(name: &str, values: ArcArray1<f64>) -> Self {
        Self(ModelCell::new(name, values))
    }
}
impl Resistors for ModifiedModelCell {
    fn get_name(&self) -> String {
        self.0.get_name()
    }
    fn mean(&self) -> f64 {
        self.0.mean()
    }
    fn get_values(&self) -> ArcArray1<f64> {
        self.0.get_values()
    }
}

#[derive(Clone)]
pub struct Internal(ModelCell);

impl Internal {
    pub fn new(name: &str, values: ArcArray1<f64>) -> Self {
        Self(ModelCell::new(name, values))
    }
}
impl Resistors for Internal {
    fn get_name(&self) -> String {
        self.0.get_name()
    }
    fn mean(&self) -> f64 {
        self.0.mean()
    }
    fn get_values(&self) -> ArcArray1<f64> {
        self.0.get_values()
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
