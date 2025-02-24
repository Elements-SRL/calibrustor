use crate::{
    calibration::{
        setup::{Setup, SetupStatus},
        sub_step::{vc_i_gain::iv_estimation::IVEstimationIGainSubStep, SubStep},
    },
    device::Device,
    device_error::DeviceError,
    uom::{Ampere, Volt},
};

use super::Step;

#[derive(Clone)]
pub struct IVEstimationIGain {
    sub_steps: Vec<IVEstimationIGainSubStep>,
}

impl IVEstimationIGain {
    pub fn new(sub_steps: Vec<IVEstimationIGainSubStep>) -> Self {
        Self { sub_steps }
    }
}

impl Step<Volt, Ampere> for IVEstimationIGain {
    fn get_sub_steps(&self) -> Vec<impl SubStep<Volt, Ampere>> {
        self.sub_steps.to_vec()
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
        d: impl Device<Volt, Ampere>,
    ) -> Result<Box<dyn Device<Volt, Ampere>>, DeviceError>
    where
        Self: Sized,
    {
        Err(DeviceError::DeviceNotReady)
    }
}
