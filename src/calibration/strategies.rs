use super::{calib_context::CalibContext, calibration_result::CalibrationResult};
use crate::{device::Device, uom::Uom};
pub mod iv_estimation;
pub mod vc_i_offset;

pub trait CalibrationStrategy<S, R, D>
where
    S: Uom,
    R: Uom,
    D: Device<S, R>,
{
    fn calibrate(&self, d: &D, cc: CalibContext<S, R>) -> CalibrationResult<S, R>;
}
