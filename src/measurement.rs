use std::fmt::Debug;

use crate::{prefix::UnitPfx, uom::Uom};

#[derive(Clone, Copy, Debug)]
pub struct Measurement<T: Uom> {
    value: f64,
    pfx: UnitPfx,
    uom: T,
}
