use super::step::{
    vc_i_gain::IVEstimationIGain,
    vc_i_offset::VcIOffsetStd,
};

struct El03CProgram {
    vc_i_gain: IVEstimationIGain,
    vc_i_offset: VcIOffsetStd,
}

impl El03CProgram {
    pub fn new(vc_i_gain: IVEstimationIGain, vc_i_offset: VcIOffsetStd) -> Self {
        Self {
            vc_i_gain,
            vc_i_offset,
        }
    }
}

mod el03_ctest {

    use crate::{
        calibration::{
            calib_context::CalibContext,
            step::{
                vc_i_gain::{self, IVEstimationIGain, IVEstimationIGainSubStep},
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

    use super::El03CProgram;

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
            IVEstimationIGainSubStep::new(n200_slow.clone(), big_res.clone()),
            IVEstimationIGainSubStep::new(n200_fast.clone(), big_res),
            IVEstimationIGainSubStep::new(n20_slow.clone(), small_res.clone()),
            IVEstimationIGainSubStep::new(n20_fast.clone(), small_res),
        ]);
        let vc_i_offset = VcIOffsetStd::new(vec![
            VcIOffsetStdSubStep::new(n200_slow),
            VcIOffsetStdSubStep::new(n200_fast),
            VcIOffsetStdSubStep::new(n20_slow),
            VcIOffsetStdSubStep::new(n20_fast),
        ]);
        let p = El03CProgram::new(vc_i_gain, vc_i_offset);

    }
}
