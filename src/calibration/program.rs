use crate::resistors::{Internal, ModifiedModelCell};

use super::step::{vc_i_gain::IVEstimationIGain, vc_i_offset::VcIOffsetStd};

pub struct ProgramEl03c {
    // vc_v_gain: Option<bool>,
    vc_i_gain: Option<IVEstimationIGain<ModifiedModelCell>>,
    vc_i_offset: Option<VcIOffsetStd>,
    // vc_v_offset: Option<bool>,
}

impl ProgramEl03c {
    pub fn new(
        // vc_v_gain: Option<bool>,
        vc_i_gain: Option<IVEstimationIGain<ModifiedModelCell>>,
        vc_i_offset: Option<VcIOffsetStd>,
        // vc_v_offset: Option<bool>
    ) -> Self {
        Self {
            // vc_v_gain,
            vc_i_gain,
            vc_i_offset,
            // vc_v_offset
        }
    }
}

struct ProgramEl07cd {
    // vc_g_offset: Option<bool>,
    vc_i_gain: Option<IVEstimationIGain<Internal>>,
    vc_i_offset: Option<VcIOffsetStd>,
    // vc_v_offset: Option<bool>,
    // vc_v_rs_correction: Option<bool>,
    // cc_v_gain: Option<bool>,
    // cc_v_offset: Option<bool>,
    // cc_i_offset: Option<bool>,
    // cc_i_gain: Option<bool>,
}
