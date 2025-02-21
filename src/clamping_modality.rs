// Clamping modality
pub trait ClampingModality {}

pub struct VoltageClamp {}
impl ClampingModality for VoltageClamp {}

pub struct CurrentClamp {}
impl ClampingModality for CurrentClamp {}
