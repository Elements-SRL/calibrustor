use super::CalibrationStrategy;
use crate::{
    calibration::{
        calib_context::CalibContext, calibration_result::CalibrationResult,
        calibration_values::CalibrationValues,
    },
    device::Device,
    device_error::DeviceError,
    uom::{Ampere, Volt},
};
use std::collections::HashMap;

pub struct IOffsetStd {
    setup_complete: bool,
}

impl IOffsetStd {
    pub fn new() -> Self {
        Self {
            setup_complete: false,
        }
    }
}

impl<D: Device<Volt, Ampere>> CalibrationStrategy<Volt, Ampere, D> for IOffsetStd {
    fn calibrate(&self, d: &D, cc: CalibContext<Volt, Ampere>) -> CalibrationResult<Volt, Ampere> {
        let a = d.acquire(1.0);
        let offsets = a.get_readout_means();
        let cv = CalibrationValues::new_float(HashMap::from_iter(offsets.into_iter().enumerate()));
        CalibrationResult::new(cc, cv)
    }

    fn setup(self, d: D, cc: &CalibContext<Volt, Ampere>) -> Result<D, DeviceError> {
        d.set_calib_context(cc)
    }

    fn complete_setup(self) -> Self {
        Self {
            setup_complete: true,
            ..self
        }
    }

    fn is_setup_complete(&self) -> bool {
        self.setup_complete
    }
}
