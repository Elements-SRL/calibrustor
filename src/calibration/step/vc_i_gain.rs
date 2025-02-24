use crate::{
    calibration::{
        setup::{Setup, SetupStatus},
        sub_step::{vc_i_gain::iv_estimation::IVEstimationIGainSubStep, SubStep},
    },
    device::Device,
    device_error::DeviceError,
    resistors::{Internal, ModelCell, ModifiedModelCell, RegularModelCell, Resistors},
    uom::{Ampere, Volt},
};

use super::Step;

#[derive(Clone)]
pub struct IVEstimationIGain<R>
where
    R: Resistors,
{
    sub_steps: Vec<IVEstimationIGainSubStep<R>>,
}

impl<R> IVEstimationIGain<R>
where
    R: Resistors,
{
    pub fn new(sub_steps: Vec<IVEstimationIGainSubStep<R>>) -> Self {
        Self { sub_steps }
    }
}

impl<R, D> Step<Volt, Ampere, D> for IVEstimationIGain<R>
where
    D: Device<Volt, Ampere>,
    R: Resistors,
    IVEstimationIGain<R>: Setup<Volt, Ampere, D>,
{
    fn get_sub_steps(&self) -> Vec<impl SubStep<Volt, Ampere, D>> {
        self.sub_steps.to_vec()
    }
}

// here we'll define different implementation, one for
// each kind of resitance
impl<D> Setup<Volt, Ampere, D> for IVEstimationIGain<Internal>
where
    D: Device<Volt, Ampere>,
{
    fn complete(self) -> Self
    where
        Self: Sized,
    {
        self
    }
    fn get_status(&self) -> SetupStatus {
        SetupStatus::NotInitialized
    }
    fn setup(&self, d: D) -> Result<D, DeviceError>
    where
        Self: Sized,
    {
        Err(DeviceError::DeviceNotReady)
    }
}

impl<D> Setup<Volt, Ampere, D> for IVEstimationIGain<RegularModelCell>
where
    D: Device<Volt, Ampere>,
{
    fn complete(self) -> Self
    where
        Self: Sized,
    {
        self
    }
    fn get_status(&self) -> SetupStatus {
        SetupStatus::NotInitialized
    }
    fn setup(&self, d: D) -> Result<D, DeviceError>
    where
        Self: Sized,
    {
        Err(DeviceError::DeviceNotReady)
    }
}

impl<D> Setup<Volt, Ampere, D> for IVEstimationIGain<ModifiedModelCell>
where
    D: Device<Volt, Ampere>,
{
    fn complete(self) -> Self
    where
        Self: Sized,
    {
        self
    }
    fn get_status(&self) -> SetupStatus {
        SetupStatus::NotInitialized
    }
    fn setup(&self, d: D) -> Result<D, DeviceError>
    where
        Self: Sized,
    {
        Err(DeviceError::DeviceNotReady)
    }
}
