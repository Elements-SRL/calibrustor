use super::step::{vc_i_gain::IVEstimationIGain, vc_i_offset::VcIOffsetStd};

struct El03CProgram {
    vc_v_gain: IVEstimationIGain,
    vc_i_gain: VcIOffsetStd,
}
