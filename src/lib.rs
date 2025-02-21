pub mod acquisition;
pub mod calibration;
pub mod clamping_modality;
pub mod device;
pub mod device_error;
pub mod devices;
pub mod measurement;
pub mod prefix;
pub mod ranged_measurement;
pub mod resistors;
pub mod sampling_rate;
pub mod uom;

// pub struct AbstractCalib {
//     multimeter: usize,
//     current_range_idxs: Vec<usize>,
//     voltage_range_idxs: Vec<usize>,
//     channel_indexes: Vec<usize>,
//     sampling_modes: Vec<usize>,
//     calibrations: Vec<usize>,
//     skip_check_contacting_routine: bool,
// }
