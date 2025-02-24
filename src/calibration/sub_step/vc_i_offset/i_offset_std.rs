use crate::{
    calibration::{
        calib_context::CalibContext,
        calibration_result::CalibrationResult,
        setup::{Setup, SetupStatus},
        strategies::{vc_i_offset::IOffsetStd, CalibrationStrategy},
        sub_step::SubStep,
    },
    device::Device,
    device_error::DeviceError,
    uom::{Ampere, Volt},
};

#[derive(Clone)]
pub struct VcIOffsetStdSubStep {
    cc: CalibContext<Volt, Ampere>,
}

impl VcIOffsetStdSubStep {
    pub fn new(cc: CalibContext<Volt, Ampere>) -> Self {
        Self { cc }
    }
}

impl<D: Device<Volt, Ampere>> SubStep<Volt, Ampere, D> for VcIOffsetStdSubStep {
    fn get_strategy(&self) -> impl CalibrationStrategy<Volt, Ampere, D> {
        IOffsetStd
    }
}

impl<D: Device<Volt, Ampere>> CalibrationStrategy<Volt, Ampere, D> for VcIOffsetStdSubStep {
    fn calibrate(&self, d: &D, cc: CalibContext<Volt, Ampere>) -> CalibrationResult<Volt, Ampere> {
        let s = self.get_strategy();
        s.calibrate(d, cc)
    }
}
impl<D: Device<Volt, Ampere>> Setup<Volt, Ampere, D> for VcIOffsetStdSubStep {
    fn setup(&self, d: D) -> Result<D, DeviceError>
    where
        Self: Sized,
    {
        let d = d.set_calib_context(&self.cc)?;
        todo!()
    }
    fn complete(self) -> Self
    where
        Self: Sized,
    {
        self
    }
    fn get_status(&self) -> SetupStatus {
        SetupStatus::NotInitialized
    }
}
