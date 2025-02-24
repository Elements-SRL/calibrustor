use crate::{
    acquisition::Acquisition,
    calibration::{calibration_error::CalibrationError, calibration_result::CalibrationResult},
    device::Device,
    device_error::DeviceError,
    prefix::UnitPfx,
    ranged_measurement::RangedMeasurement,
    sampling_rate::SamplingRate,
    uom::{Ampere, Uom, Volt},
};
use ndarray::ArcArray2;
use std::{collections::HashMap, marker::PhantomData};

struct Dummy<S: Uom, R: Uom> {
    counter: usize,
    channels: usize,
    stimulus: PhantomData<S>,
    readout: PhantomData<R>,
}

impl<S: Uom, R: Uom> Dummy<S, R> {
    fn new(counter: usize, channels: usize) -> Self {
        Self {
            counter,
            channels,
            stimulus: PhantomData,
            readout: PhantomData,
        }
    }
    fn inc(self) -> Self {
        Self {
            counter: self.counter + 1,
            ..self
        }
    }
}

impl Device<Volt, Ampere> for Dummy<Volt, Ampere> {
    // getters
    fn get_active_stimuli_channels(&self) -> Vec<usize> {
        (0..self.channels).collect()
    }
    fn get_active_readout_channels(&self) -> Vec<usize> {
        (0..self.channels).collect()
    }
    fn get_stimuli_channels_num(&self) -> usize {
        self.channels
    }
    fn get_readout_channels_num(&self) -> usize {
        self.channels
    }
    fn get_stimulus_range(self) -> RangedMeasurement<Volt> {
        RangedMeasurement::new(0, -500.0, 500.0, 1.0, UnitPfx::Milli)
    }
    fn get_readout_range(self) -> RangedMeasurement<Ampere> {
        RangedMeasurement::new(0, -200.0, 200.0, 1.0, UnitPfx::Nano)
    }

    // setters
    // when setting the stim, be sure to check that the stim is being set
    // so after the application wait for data from the device until the stm
    // equals set value (within a certain range) or wait for some millis.
    fn set_stimuli(&self, map: HashMap<usize, f64>) {
        println!("setting: {:?}", map);
    }
    fn set_stimulus_range(self, rm: RangedMeasurement<Volt>) -> Result<Self, DeviceError> {
        println!("setting: {:?}", rm);
        Ok(self)
    }
    fn set_readout_range(self, rm: RangedMeasurement<Ampere>) -> Result<Self, DeviceError> {
        println!("setting: {:?}", rm);
        Ok(self)
    }
    fn set_sampling_rate(self, sr: SamplingRate) -> Result<Self, DeviceError> {
        println!("setting: {:?}", sr);
        Ok(self)
    }
    fn acquire(&self, t: f64) -> Acquisition {
        let t = (t * 100.0) as usize;
        let s = ArcArray2::from_elem((self.channels, t), self.counter as f64);
        let r = ArcArray2::from_elem((self.channels, t), self.counter as f64);
        let active_readout_channels = self.get_active_readout_channels();
        let active_stimulus_channels = self.get_active_stimuli_channels();
        Acquisition::new(active_readout_channels, active_stimulus_channels, s, r)
    }
    fn set_calibration(self, cr: CalibrationResult<Volt, Ampere>) -> Result<Self, CalibrationError>
    where
        Self: Sized,
    {
        println!("applying calibration {:?}", cr);
        Ok(self)
    }
}

impl Device<Ampere, Volt> for Dummy<Ampere, Volt> {
    // getters
    fn get_active_stimuli_channels(&self) -> Vec<usize> {
        (0..self.channels).collect()
    }
    fn get_active_readout_channels(&self) -> Vec<usize> {
        (0..self.channels).collect()
    }
    fn get_stimuli_channels_num(&self) -> usize {
        self.channels
    }
    fn get_readout_channels_num(&self) -> usize {
        self.channels
    }
    fn get_stimulus_range(self) -> RangedMeasurement<Ampere> {
        RangedMeasurement::new(0, -250.0, 250.0, 1.0, UnitPfx::Pico)
    }
    fn get_readout_range(self) -> RangedMeasurement<Volt> {
        RangedMeasurement::new(0, -2000.0, 2000.0, 1.0, UnitPfx::Milli)
    }

    // setters
    // when setting the stim, be sure to check that the stim is being set
    // so after the application wait for data from the device until the stm
    // equals set value (within a certain range) or wait for some millis.
    fn set_stimuli(&self, map: HashMap<usize, f64>) {
        println!("setting: {:?}", map);
    }
    fn set_stimulus_range(self, rm: RangedMeasurement<Ampere>) -> Result<Self, DeviceError> {
        println!("setting: {:?}", rm);
        Ok(self)
    }
    fn set_readout_range(self, rm: RangedMeasurement<Volt>) -> Result<Self, DeviceError> {
        println!("setting: {:?}", rm);
        Ok(self)
    }
    fn set_sampling_rate(self, sr: SamplingRate) -> Result<Self, DeviceError> {
        println!("setting: {:?}", sr);
        Ok(self)
    }
    fn acquire(&self, t: f64) -> Acquisition {
        let t = (t * 100.0) as usize;
        let s = ArcArray2::from_elem((self.channels, t), self.counter as f64);
        let r = ArcArray2::from_elem((self.channels, t), self.counter as f64);
        let active_readout_channels = self.get_active_readout_channels();
        let active_stimulus_channels = self.get_active_stimuli_channels();
        Acquisition::new(active_readout_channels, active_stimulus_channels, s, r)
    }
    fn set_calibration(self, cr: CalibrationResult<Ampere, Volt>) -> Result<Self, CalibrationError>
    where
        Self: Sized,
    {
        println!("applying calibration {:?}", cr);
        Ok(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        calibration::{
            calib_context::CalibContext, program::ProgramEl03c, step::{vc_i_gain::IVEstimationIGain, vc_i_offset::VcIOffsetStd}, strategies::{iv_estimation::IVEstimation, vc_i_offset::IOffsetStd}, sub_step::{
                vc_i_gain::iv_estimation::IVEstimationIGainSubStep,
                vc_i_offset::i_offset_std::VcIOffsetStdSubStep,
            }
        },
        resistors::{ModelCell, Resistors},
    };
    use ndarray::array;

    #[test]
    fn dummy_dev() {
        let small_resistors = Resistors::ModifiedModelCell(ModelCell::new(
            "5M",
            array![5.0e6, 5.0e6, 5.0e6, 5.0e6,].into(),
        ));
        let big_resistors = Resistors::ModifiedModelCell(ModelCell::new(
            "500M",
            array![500.0e6, 500.0e6, 500.0e6, 500.0e6,].into(),
        ));
        let stim_small_resistors = vec![-0.04, -0.02, -0.01, 0.0, 0.01, 0.02, 0.04];
        let stim_big_resistors = vec![-0.4, -0.2, -0.1, 0.0, 0.1, 0.2, 0.4];

        let stim_range = RangedMeasurement::<Volt>::new(0, -500.0, 500.0, 1.0, UnitPfx::Milli);

        let range_200n = RangedMeasurement::<Ampere>::new(0, -200.0, 200.0, 1.0, UnitPfx::Nano);
        let range_20n = RangedMeasurement::<Ampere>::new(0, -20.0, 20.0, 1.0, UnitPfx::Nano);

        let sr1 = SamplingRate::new(0, 1.25, UnitPfx::Kilo);
        let sr2 = SamplingRate::new(3, 50.0, UnitPfx::Kilo);

        let n200_slow = CalibContext::new(stim_range, range_200n, sr1);
        let n200_fast = CalibContext::new(stim_range, range_200n, sr2);
        let n20_slow = CalibContext::new(stim_range, range_20n, sr1);
        let n20_fast = CalibContext::new(stim_range, range_20n, sr2);

        let offset = IOffsetStd;

        let big_res = IVEstimation::new(big_resistors, stim_big_resistors);
        let small_res = IVEstimation::new(small_resistors, stim_small_resistors);

        let vc_i_gain = IVEstimationIGain::new(vec![
            IVEstimationIGainSubStep::new(n200_slow.clone(), big_res.clone()),
            IVEstimationIGainSubStep::new(n200_fast.clone(), big_res.clone()),
            IVEstimationIGainSubStep::new(n20_slow.clone(), small_res.clone()),
            IVEstimationIGainSubStep::new(n20_fast.clone(), small_res),
        ]);
        let vc_i_offset = VcIOffsetStd::new(vec![
            VcIOffsetStdSubStep::new(n200_slow),
            VcIOffsetStdSubStep::new(n200_fast),
            VcIOffsetStdSubStep::new(n20_slow),
            VcIOffsetStdSubStep::new(n20_fast),
        ]);
        let dev = Dummy::<Volt, Ampere>::new(1, 6);
        let p = ProgramEl03c::new(Some(vc_i_gain), Some(vc_i_offset));
        // p.get_steps().into_iter().map(|s| {
        //     let s = s.setup(&dev);
        //     s
        // })
    }

    #[test]
    fn reset_calibrations() {
        let a = array![1.0, 2.0, 3.0];
        println!("{:?}", (a / array![0.0, 1.0, 0.0]).mean());
    }
}
