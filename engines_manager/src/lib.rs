//! ## Description
//! This componenet is incharge of the communication with
//! the different engines.
//! For more information check the wiki page:
#![allow(unused)] //TODO: remove later
const ENGINES_FOLDER: &str = "./mock_files/engines";

mod engine;
use engine::Engine;
pub use engine::EngineError;

pub struct EnginesManager {
    engines: Vec<Engine>,
}

impl EnginesManager {
    pub fn new() -> EnginesManager {
        todo!();
    }

    pub fn list_engines() -> Vec<String> {
        todo!();
    }
}
