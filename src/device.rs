use crate::{
    acquisition::Acquisition,
    calibration::{
        calib_context::CalibContext, calibration_error::CalibrationError,
        calibration_result::CalibrationResult,
    },
    device_error::DeviceError,
    ranged_measurement::RangedMeasurement,
    sampling_rate::SamplingRate,
    uom::Uom,
};
use std::collections::HashMap;

pub mod dummy;

pub trait Device<S: Uom, R: Uom> {
    // getters
    fn get_active_stimuli_channels(&self) -> Vec<usize>;
    fn get_active_readout_channels(&self) -> Vec<usize>;
    fn get_stimuli_channels_num(&self) -> usize;
    fn get_readout_channels_num(&self) -> usize;
    fn get_stimulus_range(self) -> RangedMeasurement<S>;
    fn get_readout_range(self) -> RangedMeasurement<R>;

    // setters
    // when setting the stim, be sure to check that the stim is being set
    // so after the application wait for data from the device until the stm
    // equals set value (within a certain range) or wait for some millis.
    fn set_stim_on_all_channels(&self, s: f64) {
        let active_channels = self.get_active_stimuli_channels();
        self.set_stimuli(HashMap::from_iter(
            active_channels.into_iter().map(|ch| (ch, s)),
        ));
    }
    fn set_stimuli(&self, map: HashMap<usize, f64>);
    fn set_stimulus_range(self, rm: RangedMeasurement<S>) -> Result<Self, DeviceError>
    where
        Self: Sized;
    fn set_readout_range(self, rm: RangedMeasurement<R>) -> Result<Self, DeviceError>
    where
        Self: Sized;
    fn set_sampling_rate(self, sr: SamplingRate) -> Result<Self, DeviceError>
    where
        Self: Sized;
    fn set_calib_context(self, cc: &CalibContext<S, R>) -> Result<Self, DeviceError>
    where
        Self: Sized,
    {
        self.set_stimulus_range(cc.get_stimulus_range())?
            .set_readout_range(cc.get_readout_range())?
            .set_sampling_rate(cc.get_sampling_rate())
    }
    fn acquire(&self, t: f64) -> Acquisition;
    fn set_calibration(self, cr: CalibrationResult<S, R>) -> Result<Self, CalibrationError>
    where
        Self: Sized;
}

pub trait EepromDevice<S, R>: Device<S, R>
where
    S: Uom,
    R: Uom,
{
    fn read_eeprom(&self) -> Vec<u8>;
    fn write_eeprom(&self) -> Result<(), DeviceError>;
}

pub trait InputSwitchDevice<S, R>: Device<S, R>
where
    S: Uom,
    R: Uom,
{
    fn set_switches(self, map: HashMap<usize, bool>) -> Result<Self, DeviceError>
    where
        Self: Sized;
}
