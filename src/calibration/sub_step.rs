use super::{setup::Setup, strategies::CalibrationStrategy};
use crate::uom::Uom;

pub mod vc_i_gain;
pub mod vc_i_offset;

pub trait SubStep<S: Uom, R: Uom>: Setup<S, R> + CalibrationStrategy<S, R> + Sized {
    fn get_strategy(&self) -> impl CalibrationStrategy<S, R>;
}
