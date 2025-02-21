use super::calib_info::CalibInfo;

pub trait EepromCalibration {
    fn get_reset_info(&self) -> Vec<CalibInfo>;
    fn write_to_eeprom(&self, ci: Vec<CalibInfo>);
}
