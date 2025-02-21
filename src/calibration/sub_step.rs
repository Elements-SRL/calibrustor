use super::{setup::Setup, strategies::CalibrationStrategy};
use crate::uom::Uom;

pub trait SubStep<S: Uom, R: Uom>: Setup<S, R> + CalibrationStrategy<S, R> {
    fn get_strategy(&self) -> Box<dyn CalibrationStrategy<S, R>>;
}
