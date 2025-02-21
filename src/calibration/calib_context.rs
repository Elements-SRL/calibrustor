use crate::{ranged_measurement::RangedMeasurement, sampling_rate::SamplingRate, uom::Uom};

#[derive(Clone, Copy, Debug)]
pub struct CalibContext<S: Uom, R: Uom> {
    stimulus_range: RangedMeasurement<S>,
    readout_range: RangedMeasurement<R>,
    sampling_rate: SamplingRate,
}

impl<S: Uom, R: Uom> CalibContext<S, R> {
    pub fn new(
        stimulus_range: RangedMeasurement<S>,
        readout_range: RangedMeasurement<R>,
        sampling_rate: SamplingRate,
    ) -> Self {
        Self {
            stimulus_range,
            readout_range,
            sampling_rate,
        }
    }
    pub fn get_stimulus_range(&self) -> RangedMeasurement<S> {
        self.stimulus_range
    }
    pub fn get_readout_range(&self) -> RangedMeasurement<R> {
        self.readout_range
    }
    pub fn get_sampling_rate(&self) -> SamplingRate {
        self.sampling_rate
    }
}
