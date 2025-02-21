use crate::{device::Device, device_error::DeviceError, uom::Uom};
use super::calib_context::CalibContext;


pub trait Step<S: Uom, R: Uom>{
    fn setup<T1: Uom, T2: Uom>(&self, device: impl Device<T1, T2>) -> Result<impl Device<S, R>, DeviceError>;
    fn get_calibration_contexts() -> CalibContext<S, R>;
}
