#![warn(missing_docs)]

//! Holochain Test Conductor
//!
//! TODO

use holochain::ConductorHandle;
use shrinkwraprs::Shrinkwrap;

/// A wrapper for a ConductorHandle designed for use in integration tests
#[derive(Shrinkwrap)]
pub struct TestConductor {
    /// The handle
    #[shrinkwrap(main_field)]
    handle: ConductorHandle,
}

impl TestConductor {
    pub fn from_config(config: TestConductorConfig) -> Self {
        todo!()
    }
}

pub struct TestConductorConfig;
