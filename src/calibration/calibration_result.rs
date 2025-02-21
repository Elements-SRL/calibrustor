use super::{calib_context::CalibContext, calibration_values::CalibrationValues};
use crate::uom::Uom;

#[derive(Debug)]
pub struct CalibrationResult<S: Uom, R: Uom> {
    calib_context: CalibContext<S, R>,
    values: CalibrationValues,
}

impl<S: Uom, R: Uom> CalibrationResult<S, R> {
    pub fn new(calib_context: CalibContext<S, R>, values: CalibrationValues) -> Self {
        Self {
            calib_context,
            values,
        }
    }

    pub fn get_calib_context(&self) -> CalibContext<S, R> {
        self.calib_context
    }

    pub fn get_values(&self) -> CalibrationValues {
        self.values.clone()
    }
}
