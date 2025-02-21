use super::CalibrationStrategy;
use crate::{
    calibration::{
        calib_context::CalibContext, calibration_result::CalibrationResult,
        calibration_values::CalibrationValues,
    },
    device::Device,
    uom::{Ampere, Volt},
};
use std::collections::HashMap;

#[derive(Default, Clone, Copy)]
pub struct IOffsetStd;

impl CalibrationStrategy<Volt, Ampere> for IOffsetStd {
    fn calibrate(
        &self,
        d: &dyn Device<Volt, Ampere>,
        cc: CalibContext<Volt, Ampere>,
    ) -> CalibrationResult<Volt, Ampere> {
        let a = d.acquire(1.0);
        let offsets = a.get_readout_means();
        let cv = CalibrationValues::new_float(HashMap::from_iter(offsets.into_iter().enumerate()));
        CalibrationResult::new(cc, cv)
    }
}
