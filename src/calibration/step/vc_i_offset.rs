use crate::{
    calibration::{
        calib_context::CalibContext,
        setup::{Setup, SetupStatus},
        strategies::{vc_i_offset::IOffsetStd, CalibrationStrategy},
    },
    device::Device,
    device_error::DeviceError,
    uom::{Ampere, Volt},
};

use super::Step;

pub struct VcIOffsetStd {
    calib_contexts: Vec<CalibContext<Volt, Ampere>>,
}

impl Step<Volt, Ampere> for VcIOffsetStd {
    fn get_calibration_contexts(&self) -> Vec<CalibContext<Volt, Ampere>> {
        self.calib_contexts.clone()
    }
    fn get_strategy(&self) -> impl CalibrationStrategy<Volt, Ampere> {
        IOffsetStd
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
        cc: Option<CalibContext<Volt, Ampere>>,
    ) -> Result<Box<dyn Device<Volt, Ampere>>, DeviceError>
    where
        Self: Sized,
    {
        Err(DeviceError::DeviceNotReady)
    }
}
