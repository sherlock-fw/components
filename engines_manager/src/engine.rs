use serde::Deserialize;
use serde_valid::Validate;
use std::{collections::HashMap, process};
// ----------------------------------------- Engine Struct ----------------------------------------

/// ## Description
/// A struct that's incharge of holding an engine information
/// and operate it.
/// The struct instance is created from the engine's `config.json` file
/// or manually by the user through the UI.
#[derive(Deserialize, Validate, Debug)]
pub struct Engine {
    /// The name of the engine.
    name: String,
    /// The path to the engine binary/script.
    path: String, 
    /// A HashMap of the engine's different commands with command_name:Command pairs.
    commands: Vec<Command>,
    /// A prefix for running the engine (if needed).
    /// **i.e:** `python3`, `bash -e`, `ruby`.
    prefix: Option<String>,
    /// An optional description that describes the engine.
    description: Option<String>,
}

impl Engine {
    /// ## Description
    /// A Constructor for the Engine struct.
    /// Used for creating an engine manually from the UI.
    /// ## Example
    /// **Basic usage:**
    /// ```ignore
    /// let engine = Engine::new("MyEngine","engines_path/my_engine",None,None,Some("this is my powerful engine"));
    /// ```
    pub fn new(
        name: &str,
        path: &str,
        prefix: Option<&str>,
        commands: Option<Vec<Command>>,
        description: Option<&str>,
    ) -> Self {
        Engine {
            name: name.to_owned(),
            path: path.to_owned(),
            prefix: prefix.map(ToOwned::to_owned),
            description: description.map(ToOwned::to_owned),
            commands: match commands { 
                Some(commands) => commands,
                None => Vec::new(),
            },
        }
    }

    /// ## Description
    /// Executes a given command.
    /// ## Example
    /// **Basic usage:**
    /// ```ignore
    ///     let res : Result<String,EngineError> = engine.execute("command","query")
    ///         .expect("unknown command");
    ///         println!("{}",res);  
    /// ```
    pub fn execute(&self, command_name: &str, query: &str) -> Result<String, EngineError> {
        //get the command
        let command = self.commands.iter().find(|c|c.get_name() == command_name);//get the command

        match command {
            Some(command) => {
                //get the args for the command as a vector.
                let args: Vec<String> = command
                    .parse_args(query) //replace the queryholder with the requested query
                    .split(' ') //split args by spaces
                    .map(ToOwned::to_owned)
                    .collect();

                //handle the optional prefix
                let output = if let Some(prefix) = &self.prefix {
                    process::Command::new(prefix)
                        .arg(&self.path)
                        .args(&args)
                        .output().map_err(|_|EngineError::ExecutionFailed)?
                } else {
                    process::Command::new(&self.path).args(&args).output().map_err(|_|EngineError::ExecutionFailed)?
                };

                Ok(std::str::from_utf8(&output.stdout).map_err(|_|EngineError::UnknownError)?.to_owned())
            }
            None => Err(EngineError::UnknownCommand), //the command doesn't exists
        }
    }

    /// ## Description
    /// Gets the name of the engine
    /// ## Example
    /// **Basic usage:**
    /// ```ignore
    ///    let engine_name = engine.get_name();
    ///         println!("{}",engine_name);  
    /// ```
    pub fn get_name(&self) -> &str {
        return &self.name;
    }

    /// ## Description
    /// Returns a HashMap of the engine's commands names and their descriptions as keys and values.
    /// ## Example
    /// **Basic usage:**
    /// ```ignore
    ///     let commands_list: HashMap<String,Option<String>> = engine.list_commands;
    ///     for (command_name, description) in commands_list{
    ///         println!("command name: {}, description: {}",
    ///         command_name,description.unwrap_or("".into());
    ///     }
    /// ```
    pub fn list_commands(&self) -> HashMap<String, Option<String>> {
        self.commands //generate a hashmap of command_name:description pairs.
            .iter()
            .map(|c| (c.get_name().into(), c.get_description().cloned()))
            .collect()
    }

    /// ## Description
    /// Gets a reference to the engine's description.
    /// ## Example
    /// **Basic usage:**
    /// ```ignore
    ///     let description = engine.get_description();
    ///     if let Some(desc) = description{
    ///         println!("engine description: {}",desc);
    ///     }
    /// ```
    pub fn get_description(&self) -> Option<&String> {
        self.description.as_ref()
    }

    /// ## Description
    /// Creates a new engine command.
    /// ## Example
    /// **Basic usage:**
    /// ```ignore
    ///     enginge.new_command("command_name","--search $query",None)
    ///         .expect("command exists already");
    /// ```
    pub fn new_command(
        &mut self,
        name: &str,
        args: &str,
        description: Option<&str>,
    ) -> Result<(), EngineError> {
        //check if the command exists already
        if self.is_command_exists(name) {
            return Err(EngineError::CommandExists);
        }

        match Command::new(name, args, description) {
            //create command instance
            Ok(command) => {
                //insert the command
                self.commands.push(command);
                Ok(())
            }
            Err(err) => Err(err), //args doesn't contain `$query`
        }
    }
    /// ## Description
    /// Adds a given command to the engine.
    /// used for creating an engine manually from the UI.
    /// ## Example
    /// **Basic usage:**
    /// ```ignore
    ///     let command = Command::new("command_name","--search $query",None);
    ///     enginge.add_command(command)
    ///         .expect("command exists already");
    /// ```
    pub fn add_command(&mut self, command: Command) -> Result<(), EngineError> {
        //check if the command exists already
        if self.is_command_exists(command.get_name()) {
            return Err(EngineError::CommandExists);
        }
        self.commands.push(command);
        Ok(())
    }

    // Check if there is a command with a given name
    fn is_command_exists(&self,name:&str)-> bool{
        self.commands.iter().any(|c|c.get_name() == name)
    }
}

// ------------------------------------------ Aux Structs ------------------------------------------

/// A struct that is used by the `Engine` struct to hold commands information.
#[derive(Clone, Deserialize, Validate, Debug)]
pub struct Command {
    /// ## Description
    /// the arguments for running the command.
    ///
    /// **Note:** there should be `$query` placeholder in the place where the query should be.
    ///
    /// **For example:**
    /// Let's assume that our engine has a command for searches for a user that goes like this:
    /// ```bash
    /// ./engine binary -searchuser=user123
    /// ```
    /// In that case the args should be: "-searchuser=$query"
    name: String,
    #[validate(
        pattern = r"^.*\$query.*$",
        message = r"`args` must contains `$query`."
    )] // validation for json conversion
    args: String,
    /// ## Description
    /// An optional description that describes the engine.
    description: Option<String>,
}

impl Command {
    /// ## Description
    /// Constructor for the Command struct.
    ///
    /// Used for creating an engine manually from the UI and supposed to be used only by `Engine::add_command`
    ///
    /// **Note:** The command's args must include the `$query` placeholder, which will be replace with the search query at exection.
    /// ## Example
    /// **Basic usage:**
    /// ```ignore
    /// let command = Command::new("command_name","-u $query",Some("command description"))
    ///     .expect("args missing `$query`");
    /// ```
    pub fn new(name: &str, args: &str, description: Option<&str>) -> Result<Command, EngineError> {
        if !args.contains("$query") {
            // make sure that the args contains the `$query` placeholder
            Err(EngineError::InvalidArgs)
        } else {
            Ok(Command {
                name: name.into(),
                args: args.to_owned(),
                description: description.map(ToOwned::to_owned),
            })
        }
    }

    /// ## Description
    /// Replaces the `$query` placeholder with the given query and returns the engine's args for the execution.
    /// ## Example
    /// **Basic usage:**
    /// ```ignore
    ///
    /// let command = Command::new("-search=$query",None).unwrap();
    /// assert_eq!(command.parse_args("user123"),"-search=user123");
    /// ```
    pub fn parse_args(&self, query: &str) -> String {
        self.args.replace("$query", query)
    }

    /// ## Description
    /// Gets a reference to the command description if there is one.
    /// ## Example
    /// **Basic usage:**
    /// ```ignore
    ///     let description = command.get_description();
    ///     if let Some(desc) = description{
    ///         println!("command description: {}",desc);
    ///     }
    /// ```
    pub fn get_description(&self) -> Option<&String> {
        self.description.as_ref()
    }

    /// ## Description
    /// Gets a reference to the command's name
    /// ## Example
    /// **Basic usage:**
    /// ```ignore
    ///     let command_name = command.get_name();
    ///     println!("command's name: {}",command_name);
    /// ```
    pub fn get_name(&self) -> &str {
        &self.name
    }
}

// ------------------------------------------ Custom Error ------------------------------------------
/// ## Description
/// Custom error struct for the Engines Manager crate.
#[derive(PartialEq, Debug)]
pub enum EngineError {
    /// Occurs when trying to add a command that exists already.
    CommandExists,
    /// Occurs when given args without the $query placeholder.
    InvalidArgs,
    /// Occurs when invalid engine path is given.
    InvalidEnginePath,
    /// Occurs when an execution of a command has failed.      
    ExecutionFailed,
    /// Occurs when an unknown command has given.
    UnknownCommand,
    /// Defualt Error
    UnknownError,
}

impl std::fmt::Display for EngineError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            EngineError::CommandExists => f.write_str("CommandExists"),
            EngineError::InvalidArgs => f.write_str("InvalidArgs"),
            EngineError::ExecutionFailed => f.write_str("ExecutionFailed"),
            EngineError::UnknownCommand => f.write_str("UnknownCommand"),
            EngineError::UnknownError => f.write_str("UnknownError"),
            EngineError::InvalidEnginePath => f.write_str("InvalidEnginePath"),
        }
    }
}

impl std::error::Error for EngineError {
    fn description(&self) -> &str {
        match *self {
            EngineError::CommandExists => "Command exists already",
            EngineError::InvalidArgs => "Invalid arguments has provided",
            EngineError::ExecutionFailed => "Failed to execute command",
            EngineError::UnknownCommand => "Unknown command has given",
            EngineError::UnknownError => "Unknown error",
            EngineError::InvalidEnginePath =>"Invalid engine path has provided",
        }
    }
}

// ------------------------------------------- UnitTests -------------------------------------------
mod tests;
