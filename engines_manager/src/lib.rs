#![allow(unused)] //TODO: remove later

mod engine;
use std::collections::HashMap;

use engine::Engine;
pub use engine::EngineError;

/// ## Description:
/// A struct that manages the engines of the system.
pub struct EnginesManager {
    engines: HashMap<String, Engine>,
}

impl EnginesManager {
    /// ## Description
    /// Initiates EnginesManager.
    /// Used by SherlockManager which holds the system's instance.
    /// ## Example
    /// **Basic usage:**
    /// ```
    /// let engines_manager = EnginesManager::init();
    /// ```
    pub fn init() -> EnginesManager {
        EnginesManager {
            engines: HashMap::new(),
        }
    }

    pub fn from_config(engines_folder:&str) -> EnginesManager{
        todo!()
    }
    /// ## Description
    /// Adds new engine
    /// Supposed to be called only when user manually add engine via the UI.
    /// ## Example
    /// **Basic usage:**
    /// ```
    ///# let mut engines_manager =  EnginesManager::init();
    /// engines_manager.add_engine("engine_name","path_to_engine",None,None)
    ///     .expect("engine exists already");
    /// ```
    pub fn add_engine(
        &mut self,
        name: &str,
        path: &str,
        prefix: Option<&str>,
        description: Option<&str>,
    ) -> Result<(), Error> {
        //check if the engine exists already
        if self.engines.contains_key(name) {
            return Err(Error::EngineExists);
        }

        // add the engine
        self.engines.insert(
            name.to_owned(),
            Engine::new(name, path, prefix, description),
        );
        Ok(())
    }

    /// ## Description
    /// Gets a list of the engine's commands names.
    /// ## Example
    /// **Basic usage:**
    /// ```
    /// # let mut engines_manager =  EnginesManager::init();
    /// engine_manager.list_engine_commands("engine_name")
    ///     .expect("unknown engine");
    /// ```
    pub fn list_engine_commands(&self, engine: &str) -> Result<Vec<String>, Error> {
        match self.engines.get(engine) {
            Some(engine) => {
                //if the engine exists, list its commands
                Ok(engine.list_commands())
            },
            None => {
                //unknown engine
                Err(Error::UnknownEngine)
            }
        }
    }
    /// ## Description
    /// Executes engine's command.
    // TODO: add an example
    pub fn execute(&self,engine:&str,command:&str,query:&str) -> Result<String,Error>{
        match self.engines.get(engine) {
            Some(engine) => {
                //if the engine exists, execute its command
                engine.execute(command, query).map_err(|_|Error::UnkownCommand) //replace error type
            },
            None => {
                //unknown engine
                Err(Error::UnknownEngine)
            }
        }
    }

    pub fn remove_engine(&mut self,engine:&str){
        self.engines.remove(engine);
    }

    pub fn list_engines(&self) -> Vec<String> {
        self.engines.keys().cloned().collect()
    }
}

pub enum Error {
    EngineExists,
    UnknownEngine,
    UnkownCommand,
    InvalidConfig(String),
}

#[cfg(test)]
mod tests {
//TODO: write tests
}
