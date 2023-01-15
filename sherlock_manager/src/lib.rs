#![allow(unused)] //TODO: remove later
use config_manager::ConfigManager;
use engines_manager::EnginesManager;
use storage_manager::StorageManager;

use std::{cell::RefCell, fs, io, path, sync::mpsc, thread, time};
use serde::{Deserialize, Serialize};
use serde_json;
use tauri::Window;

#[derive(Clone, Serialize, Deserialize, Debug)]
enum Task {
    ListEngines,
    RunEngine { engine_name: String, query: String },
}

#[derive(Clone, serde::Serialize)]
enum Log {
    Info(String),
    Error(String),
}

#[derive(Clone, Debug, serde::Serialize)]
enum Result {
    List(Vec<String>),
}

enum Message {
    Log(Log),
    Task(Task),
    Result(Result),
}

pub struct SherlockManager {
    engines_manager: EnginesManager,
    configs: Option<ConfigManager>,
    tauri_window: RefCell<Option<tauri::Window>>,
    //storage_manager: StorageManager,
}

impl SherlockManager {
    //initiate TODO:documentation
    pub fn init() -> SherlockManager {
        match ConfigManager::init() {
            Ok(config_manager) => SherlockManager {
                engines_manager: EnginesManager::init(),
                configs: Some(config_manager),
                tauri_window: RefCell::new(None),
            },
            Err(error) => {
                println!("{}",error);
                SherlockManager {
                    engines_manager: EnginesManager::init(),
                    configs: None,
                    tauri_window: RefCell::new(None),
                }
            }
        }
    }

    //attach to a tauri window TODO: documentation
    pub fn attach(&self, window: tauri::Window) -> &Self {
        *self.tauri_window.borrow_mut() = Some(window);
        self
    }

    //load the engines TODO: documentation
    pub fn build(&self) -> &Self {
        // TODO: improve implementation
        println!("building");
        //if no ConfigManager do nothing
        if self.configs.is_none() {
            println!("no config");
            return self;
        }

        let engines_dir = self.configs.as_ref().unwrap().get_engines_location();
        let mut engines = vec![];
        // get the directories in the engines directory
        match fs::read_dir(engines_dir) {
            Ok(entries) => {
                engines = entries
                    .filter_map(|e| e.ok()) // unwrap entries
                    .filter(|e| e.file_type().is_ok()) // filter out failed reads of the filetype
                    .filter(|e| e.file_type().unwrap().is_dir()) // filter out entries that aren't directories
                    .map(|e| e.path())
                    .collect(); // convert to vector of paths of engines directories
            }
            Err(err) => match self.tauri_window.borrow().as_ref() {
                Some(window) => {
                    window.emit("log-event", Log::Error(err.to_string()));
                }
                None => {
                    //TODO: replace with better result handling
                    println!("{} {}",err.to_string(),engines_dir);
                    return self;
                }
            },
        }

        //import new engine from each config.json file in the engines directories
        for engine in engines {
            let engine_config = engine.join("config.json");
            println!("{}",engine_config.to_str().unwrap());
            self.engines_manager.add_engine_from_config(engine_config.to_str().unwrap());
        }

        if self.tauri_window.borrow().is_some() { //send a success log to the frontend
            self.tauri_window
                .borrow()
                .as_ref()
                .unwrap()
                .emit("log-event", Log::Info("loaded engines".into()));
        }
        self
    }

    fn do_task(&self, task: Task, tx: mpsc::Sender<Message>) {
        thread::spawn(move || {
            //emulate slow responds
            thread::sleep(time::Duration::from_secs(5));
            tx.send(Message::Result(Result::List(vec![
                "google".into(),
                "instagram".into(),
                "mysql".into(),
            ])));
        });
    }

    pub fn list_engines(&self)-> Vec<String>{
        self.engines_manager.list_engines()
    }

    pub fn listen(&self) {
        let win_ref = self.tauri_window.borrow();
        let window = win_ref.as_ref().unwrap(); //TODO:handle calling listen before attaching a window
                                                //create mpsc channel for task and results
        let (tx, rx) = mpsc::channel();

        let tx_tasks = tx.clone(); //clone for the listener handler

        //listen for task events
        //and provide a handler that uses the channel to send back Messages
        window.listen("task-event", move |event| {
            match serde_json::from_str::<Task>(event.payload().unwrap()) {
                Ok(task) => {
                    //recieved task from the frontend
                    //send back a message with the task to `listen`
                    tx_tasks.send(Message::Task(task)).unwrap(); //TODO: remove unwraping later
                }
                Err(_) => {
                    //incase of recieving bad task from the frontend
                    //send back an error log
                    tx_tasks
                        .send(Message::Log(Log::Error("invalid task".into())))
                        .unwrap();
                }
            }
        });

        loop {
            //listen for Messages and emits back to the frontent
            let recieved = rx.recv().unwrap();
            match recieved {
                Message::Task(task) => {
                    //recieved a task
                    window.emit("log-event", Log::Info(format!("{:?}", task)));
                    self.do_task(task, tx.clone())
                }
                Message::Log(log) => {
                    //recieved a log
                    window.emit("log-event", log).unwrap();
                }

                Message::Result(result) => {
                    //recieved a result
                    window.emit(
                        "log-event",
                        Log::Info(format!("{:?}", result)),
                    );
                }
            }
        }
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn build_from_config(){
        SherlockManager::init()
            .build();
    }

    #[test]
    fn list_engines(){
        let engines = SherlockManager::init()
            .build()
            .list_engines();
        println!("{:?}",engines);
    }
}