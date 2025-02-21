use std::sync::Arc;

use super::{setup::Setup, sub_step::SubStep};
use crate::uom::Uom;

pub mod vc_i_gain;
pub mod vc_i_offset;

pub trait Step<S: Uom, R: Uom>: Setup<S, R> {
    fn get_sub_steps(&self) -> Vec<Arc<dyn SubStep<S, R>>>;
}
