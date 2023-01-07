use serde::Deserialize;
use serde_valid::{json::FromJsonStr, Validate, ValidatePattern};
use std::collections::HashMap;
use std::fs;

// ----------------------------------------- Engine Struct ----------------------------------------

/// ## Description
/// A struct that incharge of holding the engines information
/// and operating them.
///
/// The struct instance is created from the engine's `config.json` file,
/// or manually by the user through the UI.
#[derive(Deserialize, Validate, Debug)]
pub struct Engine {
    /// The name of the engine.
    name: String,
    /// The path to the engine binary/script.
    path: String, // TODO: validate that the file exists.
    /// A HashMap of the engine's different commands with the command name as the key
    /// and a Command struct as the value.
    commands: HashMap<String, Command>,
    /// A prefix for running the engine (if needed).
    ///
    /// **for example:** `python3`, `bash -e`, `ruby`.
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
    ///
    /// let engine = Engine::new("PowerfulEngine","engines_path/powerful_engine",None,Some("this is a powerful engine"));
    /// ```
    pub fn new(name: &str, path: &str, prefix: Option<&str>, description: Option<&str>) -> Engine {
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
    ///     // lets assume that our engine has a search_users command
    ///     // and we want to search the user: `user123`
    ///     let res:Result<String,EngineError> = engine.execute("search_users","user123")
    ///         .expect("unkown command");
    ///         println!("{}",res);  
    /// ```
    pub fn execute(&self, command: &str, query: &str) -> Result<String, EngineError> {
        let command = self.commands.get(command); //get the command
        let prefix = self.prefix.clone().unwrap_or("".to_owned()); //get the prefix

        match command {
            Some(command) => {
                //get the args for the command
                let args = command.parse_args(query);
                //execute
                //TODO: for now instead of execution the function will return the String for
                //the execution.
                //Note: this function should execute the command with a subprocess and return its
                //results.
                Ok(format!("{} {} {}", prefix, self.path, args))
            }
            None => Err(EngineError::UnkownCommand),
        }
    }

    /// ## Description
    /// gets the name of the engine
    pub fn get_name(&self) -> String {
        return self.name.clone();
    }

    /// ## Description
    /// returns a vector of the engine's commands names
    pub fn list_commands(&self) -> Vec<String> {
        self.commands.keys().cloned().collect()
    }

    /// ## Description
    /// gets a reference to the description string if there is one.
    pub fn get_description(&self) -> Option<&String> {
        self.description.as_ref()
    }

    /// ## Description
    /// adds a new command to the engine.
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

/// ## Description
/// Contains error codes for the `Engine` struct.
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

/// Private struct that is used by the `Engine` struct to hold commands information.
#[derive(Deserialize, Validate, Debug)]
struct Command {
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
    )]
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
    pub fn get_description(&self) -> Option<&String> {
        self.description.as_ref()
    }
}

// ------------------------------------------- UnitTests -------------------------------------------

#[cfg(test)]
mod command_tests {
    use super::*;
    #[test]
    fn create_with_new() {
        //valid command
        let command = Command::new("-u $query", None).unwrap();

        //invalid command
        let command = Command::new("-u query", None);
        assert!(command.is_err());
    }

    #[test]
    fn create_from_json() {
        //valid json deserialization
        let valid_json_command = r#"
            {
                "args":"-search_user=$query",
                "description":"search a user"
            }"#;
        let command = Command::from_json_str(valid_json_command).unwrap();

        //invalid json deserialization
        let invalid_json_command = r#"
        {
            "args":"-search_user=$q",
            "description":"search a user"
        }"#;
        let command = Command::from_json_str(invalid_json_command);
        assert!(command.is_err());
    }
    #[test]
    fn check_description() {
        let command = Command::new("$query", Some("test description")).unwrap();
        assert_eq!(command.get_description().unwrap(), "test description");
    }
    #[test]
    fn parse_args() {
        let command = Command::new("-searchuser=$query", None).unwrap();
        assert_eq!(command.parse_args("user123"), "-searchuser=user123");
    }
}

#[cfg(test)]
mod engine_tests {
    use serde_valid::json::FromJsonReader;

    use super::*;

    #[test]
    fn create_with_new() {
        let mut engine: Engine = Engine::new(
            "google",
            "./engines/google_engine",
            None,
            Some("google search engine"),
        );
    }
    #[test]
    fn check_add_command() {
        let mut engine = Engine::new("Engine", "./engine", None, None);
        engine.add_command("search", "-s $query", None);
        assert!(engine.execute("search", "test123").is_ok());
    }

    #[test]
    fn create_from_json_and_list() {
        //open the config file
        let fd = fs::File::open("./mock_files/facebook_engine/config.json")
            .expect("couldn't open the file");

        //convert the json to an engine instance
        let engine = Engine::from_json_reader(fd).expect("couldn't parse the json file");

        //check that the instance is valid
        //getters
        assert_eq!(engine.get_name(), "facebook");
        assert_eq!(
            engine.get_description().unwrap(),
            "Search stuff on Facebook."
        );

        //commands
        let cmd_list = engine.list_commands();
        assert_eq!(cmd_list.len(), 2);
        assert!(cmd_list.contains(&"user".to_owned()));
        assert!(cmd_list.contains(&"group".to_owned()));
    }
    #[test]
    fn execute_command() {
        //open the config file
        let fd = fs::File::open("./mock_files/facebook_engine/config.json")
            .expect("couldn't open the file");
        //convert the json to an engine instance
        let engine = Engine::from_json_reader(fd).expect("couldn't parse the json file");
        //check valid command
        assert_eq!(
            engine.execute("user", "user123").unwrap(),
            "python3 ./facebook_engine.py -search_user=user123"
        );
        //check invalid command
        assert_eq!(
            engine.execute("search", "user123").unwrap_err(),
            EngineError::UnkownCommand
        );
    }
}
