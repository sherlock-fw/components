use engine::Engine;
pub use engine::EngineError;
use serde_valid::json::FromJsonReader;
use std::{cell::RefCell, collections::HashMap, fs};

mod engine;

/// ## Description:
/// A struct that manages the engines of the system.
pub struct EnginesManager {
    engines: RefCell<HashMap<String, Engine>>,
}

impl EnginesManager {
    /// ## Description
    /// Initiates EnginesManager.
    ///
    /// Used by SherlockManager which holds the system's instance.
    /// ## Example
    /// **Basic usage:**
    /// ```
    /// let engines_manager = EnginesManager::init();
    /// ```
    pub fn init() -> EnginesManager {
        EnginesManager {
            engines: RefCell::new(HashMap::new()),
        }
    }

    /// ## Description
    /// Adds new engine from the engine's json config file.
    /// ## Example
    /// **Basic usage:**
    /// ```
    /// # use engine_manager::EnginesManager;
    /// let manager = EnginesManager::init();
    /// manager.add_engine_from_config("engine.json").unwrap();
    /// ```
    // TODO: add examples and tests.
    pub fn add_engine_from_config(&self, config_file: &str) -> Result<(), Error> {
        //open the config file
        match fs::File::open(config_file) {
            Ok(fd) => {
                // create new engine from the config file
                match Engine::from_json_reader(fd) {
                    Ok(engine) => {
                        //check if the engine exists already
                        if self.engines.borrow().contains_key(engine.get_name()) {
                            return Err(Error::EngineExists);
                        }
                        self.engines.borrow_mut().insert(engine.get_name().into(), engine);
                        Ok(())
                    }
                    Err(error) => Err(Error::InvalidConfig(error.to_string())), //convert error
                }
            }
            Err(error) => Err(Error::InvalidConfig(error.to_string())), //convert error
        }
    }

    /// ## Description
    /// Adds new engine
    /// Supposed to be called only when user manually add engine via the UI.
    /// ## Example
    /// **Basic usage:**
    /// ```
    ///# let engines_manager =  EnginesManager::init();
    /// engines_manager.add_engine("engine_name","path_to_engine",None,None)
    ///     .expect("engine exists already");
    /// ```
    // TODO: add commands
    pub fn add_engine(
        &self,
        name: &str,
        path: &str,
        prefix: Option<&str>,
        description: Option<&str>,
    ) -> Result<(), Error> {
        //check if the engine exists already
        if self.engines.borrow().contains_key(name) {
            return Err(Error::EngineExists);
        }

        // add the engine
        self.engines.borrow_mut().insert(
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
    /// # let engines_manager =  EnginesManager::init();
    /// engine_manager.list_engine_commands("engine_name")
    ///     .expect("unknown engine");
    /// ```
    pub fn list_engine_commands(&self, engine: &str) -> Result<HashMap<String,Option<String>>, Error> {
        match self.engines.borrow().get(engine) {
            Some(engine) => {
                //if the engine exists, list its commands
                Ok(engine.list_commands())
            }
            None => {
                //unknown engine
                Err(Error::UnknownEngine)
            }
        }
    }
    /// ## Description
    /// Executes engine's command.
    /// ## Example
    /// **Basic usage:**
    /// ```
    /// # let manager = EnginesManager::init();
    /// ```
    // TODO: add an example
    pub fn execute(&self, engine: &str, command: &str, query: &str) -> Result<String, Error> {
        match self.engines.borrow().get(engine) {
            Some(engine) => {
                //if the engine exists, execute its command
                engine
                    .execute(command, query)
                    .map_err(|_| Error::UnkownCommand) //replace error type
            }
            None => {
                //unknown engine
                Err(Error::UnknownEngine)
            }
        }
    }

    /// ## Description
    /// Removes an engine from the engines hashmap.
    /// ## Example
    /// **Basic usage:**
    /// ```
    /// # let manager = EnginesManager::init();
    /// manager.remove_engine("engine_name");
    /// ```
    // TODO: add test
    pub fn remove_engine(&self, engine_name: &str) {
        self.engines.borrow_mut().remove(engine_name);
    }

    /// ## Description
    /// Gets a list of the engines names.
    /// ## Example
    /// **Basic usage:**
    /// ```
    /// # let manager = EnginesManager::init();
    /// let engines: Vec<String> = manager.list_engines()
    /// ```
    // TODO: add an example
    pub fn list_engines(&self) -> Vec<String> {
        self.engines.borrow().keys().cloned().collect()
    }

    /// ## Description
    /// Gets engine's description.
    // TODO: add an example
    pub fn get_engine_description(&self, engine: &str) -> Result<Option<String>, Error> {
        //get the engine
        match self.engines.borrow().get(engine) {
            Some(engine) => Ok(engine.get_description().cloned()),
            None => Err(Error::UnknownEngine),
        }
    }

    /// ## Description
    /// Gets engine's command description.
    // TODO: add an example
    pub fn get_command_description(
        &self,
        engine: &str,
        command: &str,
    ) -> Result<Option<String>, Error> {
        todo!()
    }
}

#[derive(Debug)]
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
