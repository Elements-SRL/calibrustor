use crate::{device::Device, device_error::DeviceError, uom::Uom};

pub enum SetupStatus {
    NotInitialized,
    Ready,
    Pending,
}

pub enum SetupError {
    Saturating(Vec<usize>),
    NotContacting(Vec<usize>),
}

impl Default for SetupStatus {
    fn default() -> Self {
        Self::NotInitialized
    }
}

// todo add a method to test if the setup is ok (equivalent to check_for_contact_routine)
// it could also call
pub trait Setup<S: Uom, R: Uom> {
    fn setup(&self, d: impl Device<S, R>) -> Result<Box<dyn Device<S, R>>, DeviceError>
    where
        Self: Sized;
    fn complete(self) -> Self
    where
        Self: Sized;
    fn get_status(&self) -> SetupStatus;
    // fn test_setup(&self, d: &impl Device<S, R>) -> Result<(), SetupError> {
    //     todo!()
    // }
}
