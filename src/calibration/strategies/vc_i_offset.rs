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

fn calibrate<D: Device<Volt, Ampere>>(d: &D, cc: CalibContext<Volt, Ampere>) -> CalibrationResult<Volt, Ampere> {
    let a = d.acquire(1.0);
    let offsets = a.get_readout_means();
    let cv = CalibrationValues::new_float(HashMap::from_iter(offsets.into_iter().enumerate()));
    CalibrationResult::new(cc, cv)
}

#[derive(Default, Clone, Copy)]
pub struct IOffsetStd;

impl<D> CalibrationStrategy<Volt, Ampere, D> for IOffsetStd
where
    D: Device<Volt, Ampere>,
{
    fn calibrate(&self, d: &D, cc: CalibContext<Volt, Ampere>) -> CalibrationResult<Volt, Ampere> {
        calibrate(d, cc)
    }
}

#[derive(Default, Clone, Copy)]
pub struct IOffsetNoHands;
impl<D> CalibrationStrategy<Volt, Ampere, D> for IOffsetNoHands
where
    D: Device<Volt, Ampere>,
{
    fn calibrate(&self, d: &D, cc: CalibContext<Volt, Ampere>) -> CalibrationResult<Volt, Ampere> {
        calibrate(d, cc)
    }
}