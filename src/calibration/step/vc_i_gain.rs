use crate::{
    calibration::{
        calib_context::CalibContext,
        setup::{Setup, SetupStatus},
        strategies::{iv_estimation::IVEstimation, CalibrationStrategy},
    },
    device::Device,
    device_error::DeviceError,
    resistors::Resistors,
    uom::{Ampere, Volt},
};

use super::Step;

pub struct IVEstimationIGain {
    calib_contexts: Vec<CalibContext<Volt, Ampere>>,
    resistors: Resistors,
    stimuli: Vec<f64>,
}

impl Step<Volt, Ampere> for IVEstimationIGain {
    fn get_calibration_contexts(&self) -> Vec<CalibContext<Volt, Ampere>> {
        self.calib_contexts.clone()
    }

    fn get_strategy(&self) -> impl CalibrationStrategy<Volt, Ampere> {
        IVEstimation::new(self.resistors.clone(), self.stimuli.clone())
    }
}

impl Setup<Volt, Ampere> for IVEstimationIGain {
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
        cc: Option<CalibContext<Volt, Ampere>>,
    ) -> Result<Box<dyn Device<Volt, Ampere>>, DeviceError>
    where
        Self: Sized,
    {
        Err(DeviceError::DeviceNotReady)
    }
}
