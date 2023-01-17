#[cfg(test)]
mod command_tests {
    use crate::engine::Command;
    use serde_valid::json::FromJsonStr;

    #[test]
    fn create_with_new() {
        //valid command
        let _command = Command::new("-u $query", None).unwrap();

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
        let _command = Command::from_json_str(valid_json_command).unwrap();

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
    use crate::engine::*;
    use serde_valid::json::FromJsonReader;
    use std::fs;

    #[test]
    fn create_with_new() {
        let mut _engine: Engine = Engine::new(
            "google",
            "./engines/google_engine",
            None,
            Some("google search engine"),
        );
    }
    #[test]
    fn check_add_command() {
        let mut engine = Engine::new(
            "Engine",
            "../config_manager/mock_files/engines/test_engine/engine",
            None,
            None,
        );
        engine
            .add_command("search", "-search=$query", None)
            .unwrap();
        assert!(engine.execute("search", "test123").is_ok());
    }

    #[test]
    fn create_from_json_and_list() {
        //open the config file
        let fd = fs::File::open("../config_manager/mock_files/engines/facebook_engine/config.json")
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
        let cmd_list: Vec<String> = engine
            .list_commands()
            .keys()
            .map(ToOwned::to_owned)
            .collect();
        assert_eq!(cmd_list.len(), 2);
        assert!(cmd_list.contains(&"user".to_owned()));
        assert!(cmd_list.contains(&"group".to_owned()));
    }
    #[test]
    fn execute_command() {
        //open the config file
        let fd = fs::File::open("../config_manager/mock_files/engines/facebook_engine/config.json")
            .expect("couldn't open the file");
        //convert the json to an engine instance
        let engine = Engine::from_json_reader(fd).expect("couldn't parse the json file");
        //check valid command
        assert_eq!(engine.execute("user", "user123").unwrap(), "test output\n");
        //check invalid command
        assert_eq!(
            engine.execute("search", "user123").unwrap_err(),
            EngineError::UnkownCommand
        );
    }
}
