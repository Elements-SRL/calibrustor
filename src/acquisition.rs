use ndarray::{ArcArray2, Array1, Axis};

#[derive(Debug)]
pub struct Acquisition {
    stimulus_channels: Vec<usize>,
    readout_channels: Vec<usize>,
    stimuli: ArcArray2<f64>,
    readout: ArcArray2<f64>,
}

impl Acquisition {
    pub fn new(
        stimulus_channels: Vec<usize>,
        readout_channels: Vec<usize>,
        stimuli: ArcArray2<f64>,
        readout: ArcArray2<f64>,
    ) -> Self {
        Self {
            stimulus_channels,
            readout_channels,
            stimuli,
            readout,
        }
    }
    pub fn get_stimuli_means(&self) -> Array1<f64> {
        self.stimuli.mean_axis(Axis(1)).unwrap()
    }
    pub fn get_readout_means(&self) -> Array1<f64> {
        self.readout.mean_axis(Axis(1)).unwrap()
    }
}
