//! Platform abstraction layer for Trusted Execution Environments

use std::fmt::Debug;
use std::future::Future;
use std::path::Path;

pub trait ErrorType: Debug + Into<anyhow::Error> {}
impl<T: Debug + Into<anyhow::Error>> ErrorType for T {}

pub trait Sealing {
    type SealError: ErrorType;
    type UnsealError: ErrorType;

    fn seal_data(&self, path: impl AsRef<Path>, data: &[u8]) -> Result<(), Self::SealError>;
    fn unseal_data(&self, path: impl AsRef<Path>) -> Result<Option<Vec<u8>>, Self::UnsealError>;
}

pub trait RA {
    type Error: ErrorType;
    fn create_attestation_report(&self, data: &[u8]) -> Result<(String, String, String), Self::Error>;
}

pub trait Machine {
    fn machine_id(&self) -> Vec<u8>;
    fn cpu_core_num(&self) -> u32;
    fn cpu_feature_level(&self) -> u32;
}

pub trait Platform: Sealing + RA + Machine + Clone {}
impl<T: Sealing + RA + Machine + Clone> Platform for T {}
