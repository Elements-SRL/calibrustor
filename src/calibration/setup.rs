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
pub trait Setup<S, R, D>: Sized
where
    S: Uom,
    R: Uom,
    D: Device<S, R>,
{
    fn setup(&self, d: D) -> Result<D, DeviceError>;
    fn complete(self) -> Self;
    fn get_status(&self) -> SetupStatus;
    fn test_setup(&self, d: &D) -> Result<(), SetupError> {
        todo!()
    }
}
