use crate::{prefix::UnitPfx, uom::Hertz};

#[derive(Clone, Copy, Debug)]
pub struct SamplingRate {
    id: usize,
    value: f64,
    pfx: UnitPfx,
    uom: Hertz,
}

impl SamplingRate {
    pub fn new(id: usize, value: f64, pfx: UnitPfx) -> Self {
        Self {
            id,
            value,
            pfx,
            uom: Hertz,
        }
    }
}
