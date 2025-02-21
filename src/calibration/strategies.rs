use super::{calib_context::CalibContext, calibration_result::CalibrationResult};
use crate::{device::Device, device_error::DeviceError, uom::Uom};
pub mod iv_estimation;
pub mod vc_i_offset;

pub trait CalibrationStrategy<S, R, D>
where
    S: Uom,
    R: Uom,
    D: Device<S, R>,
{
    fn is_setup_complete(&self) -> bool;
    fn setup(self, d: D, cc: &CalibContext<S, R>) -> Result<D, DeviceError>;
    fn complete_setup(self) -> Self where Self: Sized;
    fn calibrate(&self, d: &D, cc: CalibContext<S, R>) -> CalibrationResult<S, R>;
}
