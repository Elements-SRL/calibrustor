use std::sync::Arc;

use crate::uom::{Ampere, Volt};
use super::step::{
    vc_i_gain::IVEstimationIGain,
    vc_i_offset::VcIOffsetStd, Step,
};

pub trait VoltageCalmpCalibration {
    fn get_steps(&self) -> Vec<Arc<dyn Step<Volt, Ampere>>>;
}

pub struct El03CProgram {
    steps: Vec<Arc<dyn Step<Volt, Ampere>>>,
}

impl VoltageCalmpCalibration for El03CProgram {
    fn get_steps(&self) -> Vec<Arc<dyn Step<Volt, Ampere>>> {
        self.steps.to_vec()
    }
}

impl El03CProgram {
    pub fn new(steps: Vec<Arc<dyn Step<Volt, Ampere>>>) -> Self {
        Self { steps }
    }
}

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

    use super::{El03CProgram, VoltageCalmpCalibration};

    #[test]
    fn fake_e4() {
        let small_res =
            Resistors::ModifiedModelCell(ModelCell::new("ciccia", array![2.0, 2.0, 2.0, 2.0].into()));
        let big_res = Resistors::ModifiedModelCell(ModelCell::new(
            "culo",
            array![1.0, 1.0, 1.0, 1.0].into(),
        ));
        let big_stim = vec![2.0; 7];
        let small_stim = vec![2.0; 7];
        let big_res = IVEstimation::new(big_res, big_stim);
        let small_res = IVEstimation::new(small_res, small_stim);

        let stim_range = RangedMeasurement::<Volt>::new(0, -500.0, 500.0, 1.0, UnitPfx::Milli);

        let range_200n = RangedMeasurement::<Ampere>::new(0, -200.0, 200.0, 1.0, UnitPfx::Nano);
        let range_20n = RangedMeasurement::<Ampere>::new(1, -20.0, 20.0, 1.0, UnitPfx::Nano);

        let sr1 = SamplingRate::new(0, 1.25, UnitPfx::Kilo);
        let sr2 = SamplingRate::new(3, 50.0, UnitPfx::Kilo);

        let n200_slow = CalibContext::new(stim_range, range_200n, sr1);
        let n200_fast = CalibContext::new(stim_range, range_200n, sr2);
        let n20_slow = CalibContext::new(stim_range, range_20n, sr1);
        let n20_fast = CalibContext::new(stim_range, range_20n, sr2);

        let vc_i_gain = IVEstimationIGain::new(vec![
            Arc::new(IVEstimationIGainSubStep::new(n200_slow.clone(), big_res.clone())),
            Arc::new(IVEstimationIGainSubStep::new(n200_fast.clone(), big_res)),
            Arc::new(IVEstimationIGainSubStep::new(n20_slow.clone(), small_res.clone())),
            Arc::new(IVEstimationIGainSubStep::new(n20_fast.clone(), small_res)),
        ]);
        let vc_i_offset = VcIOffsetStd::new(vec![
            Arc::new(VcIOffsetStdSubStep::new(n200_slow)),
            Arc::new(VcIOffsetStdSubStep::new(n200_fast)),
            Arc::new(VcIOffsetStdSubStep::new(n20_slow)),
            Arc::new(VcIOffsetStdSubStep::new(n20_fast)),
        ]);
        let p = El03CProgram::new(vec![Arc::new(vc_i_gain), Arc::new(vc_i_offset)]);
    }
}
