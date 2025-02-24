use crate::{
    calibration::{
        calib_context::CalibContext,
        calibration_result::CalibrationResult,
        setup::{Setup, SetupStatus},
        strategies::{iv_estimation::IVEstimation, CalibrationStrategy},
        sub_step::SubStep,
    },
    device::Device,
    device_error::DeviceError,
    uom::{Ampere, Volt},
};

#[derive(Clone)]
pub struct IVEstimationIGainSubStep {
    cc: CalibContext<Volt, Ampere>,
    cs: IVEstimation,
}

impl IVEstimationIGainSubStep {
    pub fn new(cc: CalibContext<Volt, Ampere>, cs: IVEstimation) -> Self {
        Self { cc, cs }
    }
}

impl SubStep<Volt, Ampere> for IVEstimationIGainSubStep {
    fn get_strategy(&self) -> impl CalibrationStrategy<Volt, Ampere> {
        self.cs.clone()
    }
}

impl CalibrationStrategy<Volt, Ampere> for IVEstimationIGainSubStep {
    fn calibrate(
        &self,
        d: &dyn Device<Volt, Ampere>,
        cc: CalibContext<Volt, Ampere>,
    ) -> CalibrationResult<Volt, Ampere> {
        self.get_strategy().calibrate(d, cc)
    }
}

impl Setup<Volt, Ampere> for IVEstimationIGainSubStep {
    fn complete(self) -> Self
    where
        Self: Sized,
    {
        todo!()
    }
    fn get_status(&self) -> SetupStatus {
        todo!()
    }
    fn setup(
        &self,
        d: impl Device<Volt, Ampere>,
    ) -> Result<Box<dyn Device<Volt, Ampere>>, DeviceError>
    where
        Self: Sized,
    {
        d.set_calib_context(&self.cc)?;
        todo!()
    }
}
