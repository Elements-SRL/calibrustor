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
    resistors::Resistors,
    uom::{Ampere, Volt},
};

#[derive(Clone)]
pub struct IVEstimationIGainSubStep<R>
where
    R: Resistors,
{
    cc: CalibContext<Volt, Ampere>,
    cs: IVEstimation<R>,
}

impl<R> IVEstimationIGainSubStep<R>
where
    R: Resistors,
{
    pub fn new(cc: CalibContext<Volt, Ampere>, cs: IVEstimation<R>) -> Self {
        Self { cc, cs }
    }
}

impl<R, D> SubStep<Volt, Ampere, D> for IVEstimationIGainSubStep<R>
where
    D: Device<Volt, Ampere>,
    R: Resistors,
{
    fn get_strategy(&self) -> impl CalibrationStrategy<Volt, Ampere, D> {
        self.cs.clone()
    }
}

impl<R, D> CalibrationStrategy<Volt, Ampere, D> for IVEstimationIGainSubStep<R>
where
    D: Device<Volt, Ampere>,
    R: Resistors,
{
    fn calibrate(&self, d: &D, cc: CalibContext<Volt, Ampere>) -> CalibrationResult<Volt, Ampere> {
        self.get_strategy().calibrate(d, cc)
    }
}

impl<R, D> Setup<Volt, Ampere, D> for IVEstimationIGainSubStep<R>
where
    D: Device<Volt, Ampere>,
    R: Resistors,
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
