use super::{setup::Setup, strategies::CalibrationStrategy};
use crate::{device::Device, uom::Uom};

pub mod vc_i_gain;
pub mod vc_i_offset;

pub trait SubStep<S, R, D>
where
    S: Uom,
    R: Uom,
    D: Device<S, R>,
    Self: Setup<S, R, D> + CalibrationStrategy<S, R, D>,
{
    fn get_strategy(&self) -> impl CalibrationStrategy<S, R, D>;
}
