use super::{calib_context::CalibContext, calibration_result::CalibrationResult};
use crate::{device::Device, uom::Uom};
pub mod iv_estimation;
pub mod vc_i_offset;

pub trait CalibrationStrategy<S, R>
where
    S: Uom,
    R: Uom,
{
    fn calibrate(&self, d: &dyn Device<S, R>, cc: CalibContext<S, R>) -> CalibrationResult<S, R>;
}
