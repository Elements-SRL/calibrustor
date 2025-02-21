use std::sync::Arc;
use super::step::Step;
use crate::uom::{Ampere, Volt};

// pub trait VoltageCalmpCalibration {
//     fn get_steps(&self) -> Vec<Arc<dyn Step<Volt, Ampere>>>;
// }

// pub struct El03CProgram {
//     steps: Vec<Arc<dyn Step<Volt, Ampere>>>,
// }

// impl VoltageCalmpCalibration for El03CProgram {
//     fn get_strategy(&self) -> impl CalibrationStrategy<Volt, Ampere> {
//         self.steps.to_vec
//     }
// }

// impl El03CProgram {
//     pub fn new(steps: Vec<Arc<dyn Step<Volt, Ampere>>>) -> Self {
//         Self { steps }
//     }
// }

mod el03_ctest {

    use std::sync::Arc;

    use crate::{
        calibration::{
            calib_context::CalibContext,
            step::{
                vc_i_gain::{IVEstimationIGain, IVEstimationIGainSubStep},
                vc_i_offset::{self, VcIOffsetStd, VcIOffsetStdSubStep},
            },
            strategies::iv_estimation::IVEstimation,
        },
        prefix::UnitPfx,
        ranged_measurement::RangedMeasurement,
        resistors::{ModelCell, Resistors},
        sampling_rate::SamplingRate,
        uom::{Ampere, Volt},
    };
    use ndarray::array;


    #[test]
    fn fake_e4() {
        
    }
}
