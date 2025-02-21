use std::marker::PhantomData;

use crate::{prefix::UnitPfx, uom::Uom};

#[derive(Clone, Copy, Debug)]
pub struct RangedMeasurement<T: Uom> {
    id: usize,
    min: f64,
    max: f64,
    step: f64,
    pfx: UnitPfx,
    t: PhantomData<T>,
}

impl<T: Uom> RangedMeasurement<T> {
    pub fn new(id: usize, min: f64, max: f64, step: f64, pfx: UnitPfx) -> Self {
        Self {
            id,
            min,
            max,
            step,
            pfx,
            t: PhantomData,
        }
    }
}
