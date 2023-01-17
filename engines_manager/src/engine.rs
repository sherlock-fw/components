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
    path: String, // TODO: consider if validation is needed from here
    /// A HashMap of the engine's different commands with command_name:Command pairs.
    commands: HashMap<String, Command>,
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
    /// let engine = Engine::new("MyEngine","engines_path/my_engine",None,Some("this is my powerful engine"));
    /// ```
    //TODO: add a Vec<Command> arg for the engine's command. (expose of Command struct is required).
    pub fn new(name: &str, path: &str, prefix: Option<&str>, description: Option<&str>) -> Self {
        Engine {
            name: name.to_owned(),
            path: path.to_owned(),
            commands: HashMap::new(),
            prefix: prefix.map(ToOwned::to_owned),
            description: description.map(ToOwned::to_owned),
        }
    }

    /// ## Description
    /// Executes a given command.
    /// ## Example
    /// **Basic usage:**
    /// ```ignore
    ///     let res : Result<String,EngineError> = engine.execute("command","query")
    ///         .expect("unkown command");
    ///         println!("{}",res);  
    /// ```
    pub fn execute(&self, command_name: &str, query: &str) -> Result<String, EngineError> {
        //get the command
        let command = self.commands.get(command_name); //get the command

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
                        .output()?
                } else {
                    process::Command::new(&self.path).args(&args).output()?
                };

                Ok(std::str::from_utf8(&output.stdout).unwrap().to_owned())
                //TODO: make a conversion for the error handling
            }
            None => Err(EngineError::UnkownCommand), //the command doesn't exists
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
            .map(|(k, v)| (k.clone(), v.get_description().cloned()))
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

    // TODO: change to getting a command struct (rewrite)
    /// ## Description
    /// Adds a new command to the engine.
    /// used for creating an engine manually from the UI.
    pub fn add_command(
        &mut self,
        name: &str,
        args: &str,
        description: Option<&str>,
    ) -> Result<(), EngineError> {
        if self.commands.contains_key(name) {
            //check if the command exists already
            Err(EngineError::CommandExists)
        } else {
            //if the command doesn't exist yet.
            let command = Command::new(args, description);

            match command {
                Ok(command) => {
                    self.commands.insert(name.to_owned(), command); //insert new command
                    Ok(())
                }
                Err(err) => Err(err), //args doesn't contain `$query`
            }
        }
    }
}

// ------------------------------------------ Aux Structs ------------------------------------------

/// A struct that is used by the `Engine` struct to hold commands information.
#[derive(Deserialize, Validate, Debug)]
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
    #[validate(
        pattern = r"^.*\$query.*$",
        message = r"`args` must contains `$query`."
    )]// validation for json conversion
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
    /// let command = Command::new("-u $query",Some("command description"))
    ///     .expect("args missing `$query`");
    /// ```
    pub fn new(args: &str, description: Option<&str>) -> Result<Command, EngineError> {
        if !args.contains("$query") {
            // make sure that the args contains the `$query` placeholder
            Err(EngineError::InvalidArgs)
        } else {
            Ok(Command {
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
    /// Occurs when an execution of a command has failed.      
    ExecutionFailed,
    /// Occurs when an unknown command has given.
    UnkownCommand,
}

impl std::fmt::Display for EngineError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            EngineError::CommandExists => f.write_str("CommandExists"),
            EngineError::InvalidArgs => f.write_str("InvalidArgs"),
            EngineError::ExecutionFailed => f.write_str("ExecutionFailed"),
            EngineError::UnkownCommand => f.write_str("UnkownCommand"),
        }
    }
}

impl std::error::Error for EngineError {
    fn description(&self) -> &str {
        match *self {
            EngineError::CommandExists => "Command exists already",
            EngineError::InvalidArgs => "Invalid arguments has provided",
            EngineError::ExecutionFailed => "Failed to execute command",
            EngineError::UnkownCommand => "Unkown command has given",
        }
    }
}

impl From<std::io::Error> for EngineError {
    fn from(_: std::io::Error) -> Self {
        EngineError::ExecutionFailed
    }
}

// ------------------------------------------- UnitTests -------------------------------------------
mod tests;
