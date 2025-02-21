use super::{setup::Setup, strategies::CalibrationStrategy, sub_step::SubStep};
use crate::uom::Uom;

pub mod vc_i_gain;
pub mod vc_i_offset;

pub trait Step<S: Uom, R: Uom>: Setup<S, R> {
    fn get_sub_steps(&self) -> Vec<impl SubStep<S, R>>;
}
