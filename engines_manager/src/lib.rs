//! Engines Manager:
//! This componenet is incharge of the communication with
//! the different engines.
//! For more information check the wiki page:

const ENGINES_FOLDER: &str = "../mock_files/engines";



struct Command{
    name:String, 
    cmd: String,
    prefix:Option<String>, 
}


impl Command{
    pub fn new(name:&str,command:&str,prefix:Option<String>) -> Command{
        Command{
            name:name.to_owned(),
            cmd:command.to_owned(),
            prefix:prefix,
        }
    }

    pub fn execute(){
        todo!();
    }

    pub fn get_name(&self) -> String{
        return self.name.clone()
    }
}

struct Engine{
    name: String,
    path: String,
    commands: Vec<Command>,
}

impl Engine{
    pub fn new(name:&str,path:&str,commands:Vec<Command>) -> Engine{
        todo!();
    }

    pub fn get_name(&self) -> String {
        return self.name.clone();
    }

    pub fn list_commands(&self) -> Vec<String>{
        self.commands.iter().map(|cmd| cmd.get_name()).collect()
    }
}


 pub struct EnginesManager{
    engines: Vec<Engine>,
 }
 
 
 impl EnginesManager{
    pub fn new() -> EnginesManager{
        todo!();
    }

    pub fn list_engines() ->Vec<String>{
        todo!();
    }
 }