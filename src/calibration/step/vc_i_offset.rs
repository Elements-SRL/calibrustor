use crate::{
    calibration::{
        setup::{Setup, SetupStatus},
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
