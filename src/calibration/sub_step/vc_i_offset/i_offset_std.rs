use crate::{
    calibration::{
        calib_context::CalibContext,
        calibration_result::CalibrationResult,
        setup::{Setup, SetupStatus},
        step::vc_i_offset::IOffsetStd,
        strategies::CalibrationStrategy,
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

impl SubStep<Volt, Ampere> for VcIOffsetStdSubStep {
    fn get_strategy(&self) -> impl CalibrationStrategy<Volt, Ampere> {
        IOffsetStd
    }
}

impl CalibrationStrategy<Volt, Ampere> for VcIOffsetStdSubStep {
    fn calibrate(
        &self,
        d: &dyn Device<Volt, Ampere>,
        cc: CalibContext<Volt, Ampere>,
    ) -> CalibrationResult<Volt, Ampere> {
        let s = self.get_strategy();
        s.calibrate(d, cc)
    }
}
impl Setup<Volt, Ampere> for VcIOffsetStdSubStep {
    fn setup(
        &self,
        d: impl Device<Volt, Ampere>,
    ) -> Result<Box<dyn Device<Volt, Ampere>>, DeviceError>
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
