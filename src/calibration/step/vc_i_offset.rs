use std::collections::HashMap;

use crate::{
    calibration::{
        calib_context::CalibContext,
        calibration_result::CalibrationResult,
        calibration_values::CalibrationValues,
        setup::{Setup, SetupStatus},
        strategies::CalibrationStrategy,
        sub_step::{vc_i_offset::i_offset_std::VcIOffsetStdSubStep, SubStep},
    },
    device::Device,
    device_error::DeviceError,
    uom::{Ampere, Volt},
};

use super::Step;

#[derive(Clone)]
pub struct VcIOffsetStd {
    sub_steps: Vec<VcIOffsetStdSubStep>,
}
impl VcIOffsetStd {
    pub fn new(sub_steps: Vec<VcIOffsetStdSubStep>) -> Self {
        Self { sub_steps }
    }
}

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

impl Step<Volt, Ampere> for VcIOffsetStd {
    fn get_sub_steps(&self) -> Vec<impl SubStep<Volt, Ampere>> {
        self.sub_steps.to_vec()
    }
}

impl Setup<Volt, Ampere> for VcIOffsetStd {
    fn complete(self) -> Self
    where
        Self: Sized,
    {
        self
    }
    fn get_status(&self) -> SetupStatus {
        SetupStatus::NotInitialized
    }
    fn setup(
        &self,
        d: impl crate::device::Device<Volt, Ampere>,
    ) -> Result<Box<dyn Device<Volt, Ampere>>, DeviceError>
    where
        Self: Sized,
    {
        todo!()
    }
}
