use super::{setup::Setup, sub_step::SubStep};
use crate::{device::Device, uom::Uom};

pub mod vc_i_gain;
pub mod vc_i_offset;

pub trait Step<S: Uom, R: Uom, D: Device<S, R>>: Setup<S, R, D> {
    fn get_sub_steps(&self) -> Vec<impl SubStep<S, R, D>>;
}
