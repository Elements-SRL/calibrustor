use super::{calib_context::CalibContext, setup::Setup, strategies::CalibrationStrategy};
use crate::uom::Uom;

pub mod vc_i_gain;
pub mod vc_i_offset;

pub trait Step<S: Uom, R: Uom>: Setup<S, R> {
    fn get_calibration_contexts(&self) -> Vec<CalibContext<S, R>>;
    fn get_strategy(&self) -> impl CalibrationStrategy<S, R>;
}
