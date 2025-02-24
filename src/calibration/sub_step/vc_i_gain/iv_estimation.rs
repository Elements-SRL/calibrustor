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

impl<D> SubStep<Volt, Ampere, D> for IVEstimationIGainSubStep
where
    D: Device<Volt, Ampere>,
{
    fn get_strategy(&self) -> impl CalibrationStrategy<Volt, Ampere, D> {
        self.cs.clone()
    }
}

impl<D> CalibrationStrategy<Volt, Ampere, D> for IVEstimationIGainSubStep
where
    D: Device<Volt, Ampere>,
{
    fn calibrate(&self, d: &D, cc: CalibContext<Volt, Ampere>) -> CalibrationResult<Volt, Ampere> {
        self.get_strategy().calibrate(d, cc)
    }
}

impl<D> Setup<Volt, Ampere, D> for IVEstimationIGainSubStep
where
    D: Device<Volt, Ampere>,
{
    fn complete(self) -> Self
    where
        Self: Sized,
    {
        todo!()
    }
    fn get_status(&self) -> SetupStatus {
        todo!()
    }
    fn setup(&self, d: D) -> Result<D, DeviceError>
    where
        Self: Sized,
    {
        d.set_calib_context(&self.cc)?;
        todo!()
    }
}
