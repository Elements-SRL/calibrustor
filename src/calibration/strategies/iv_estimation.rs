use super::CalibrationStrategy;
use crate::{
    calibration::{
        calib_context::CalibContext, calibration_result::CalibrationResult,
        calibration_values::CalibrationValues,
    },
    device::Device,
    device_error::DeviceError,
    resistors::Resistors,
    uom::{Ampere, Volt},
};
use ndarray::{Array1, Array2, Axis};
use std::collections::HashMap;

pub struct IVEstimation {
    setup_complete: bool,
    resistors: Resistors,
    stimuli: Vec<f64>,
}

impl IVEstimation {
    pub fn new(resistors: Resistors, stimuli: Vec<f64>) -> Self {
        Self {
            resistors,
            stimuli,
            setup_complete: false,
        }
    }
}

impl<D: Device<Volt, Ampere>> CalibrationStrategy<Volt, Ampere, D> for IVEstimation {
    fn calibrate(&self, d: &D, cc: CalibContext<Volt, Ampere>) -> CalibrationResult<Volt, Ampere> {
        let resistors = self.resistors.get_values();
        let active_channels_num = d.get_readout_channels_num();
        assert_eq!(resistors.len(), active_channels_num);
        let active_channels = d.get_active_readout_channels();
        let resistors_iter = resistors.into_iter().enumerate().filter_map(|(i, v)| {
            if active_channels.contains(&i) {
                Some(v)
            } else {
                None
            }
        });
        let resistors = Array1::from_iter(resistors_iter);
        let gains = self
            .stimuli
            .clone()
            .into_iter()
            .flat_map(|v| {
                d.set_stim_on_all_channels(v);
                let a = d.acquire(1.0);
                let i = a.get_readout_means();
                let v = Array1::from_elem(i.dim(), v);
                v / i
            })
            .collect();
        let resistors_matrix =
            Array2::from_shape_vec((self.stimuli.len(), active_channels_num), gains).unwrap();
        // columns contain channels
        let x = resistors_matrix.mean_axis(Axis(0)).unwrap();
        let y = resistors;
        let gains = x / y;
        let cv = CalibrationValues::new_float(HashMap::from_iter(gains.into_iter().enumerate()));
        CalibrationResult::new(cc, cv)
    }

    fn setup(self, d: D, cc: &CalibContext<Volt, Ampere>) -> Result<D, DeviceError> {
        let d = d.set_calib_context(cc)?;
        let nominal_value = &self.resistors.get_name();
        match &self.resistors {
            Resistors::ModelCell(_) => {
                println!("insert the model cell with {}", nominal_value);
                Err(DeviceError::DeviceNotReady)
            }
            Resistors::ModifiedModelCell(_) => {
                println!("insert the model cell with {}", nominal_value);
                println!("set the switch to internal");
                Err(DeviceError::DeviceNotReady)
            }
            Resistors::Internal(_) => {
                println!("setting automatically");
                Ok(d)
            }
        }
    }

    fn complete_setup(self) -> Self {
        Self {
            setup_complete: true,
            ..self
        }
    }

    fn is_setup_complete(&self) -> bool {
        self.setup_complete
    }
}
