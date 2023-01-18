//TODO: use one lock call in a method

#[macro_use]
extern crate lazy_static;
use std::sync::Mutex;

lazy_static! {
    static ref GLOBAL_INSTANCE: Mutex<MessagesBox> = Mutex::new(MessagesBox::init());
}

/// A singleton that is used for IPC communication between the UI thread and the SherlockManager's thread.
pub struct MessagesBox {
    jobs: Vec<Job>,
    responds: Vec<Respond>,
    logs: Vec<Log>,
    pending: bool,
}

impl MessagesBox {
    fn init() -> Self {
        MessagesBox {
            jobs: vec![],
            responds: vec![],
            logs: vec![],
            pending: false,
        }
    }

    fn get_instance() -> &'static GLOBAL_INSTANCE {
        &GLOBAL_INSTANCE
    }

    pub fn send_jobs(jobs: Vec<Job>) {
        let m = MessagesBox::get_instance();
        m.lock().unwrap().jobs.extend(jobs);
        m.lock().unwrap().pending = true;
    }

    pub fn recieve_jobs() -> Vec<Job> {
        let m = MessagesBox::get_instance();
        let out = m.lock().unwrap().jobs.clone();
        m.lock().unwrap().jobs.clear();
        out
    }

    pub fn send_responds(responds: Vec<Respond>) {
        let m = MessagesBox::get_instance();
        m.lock().unwrap().responds.extend(responds);
        m.lock().unwrap().pending = true;
    }

    pub fn finish() {
        let m = MessagesBox::get_instance();
        m.lock().unwrap().pending = false;
    }

    pub fn is_pending() -> bool {
        let m = MessagesBox::get_instance();
        m.lock().unwrap().pending
    }

    pub fn recieve_responds() -> Vec<Respond> {
        let m = MessagesBox::get_instance();
        let out = m.lock().unwrap().responds.clone();
        m.lock().unwrap().responds.clear();
        out
    }

    pub fn send_log(log: Log) {
        let m = MessagesBox::get_instance();
        m.lock().unwrap().logs.push(log);
    }
}

#[derive(Clone, Debug)]
pub enum Job {
    ListEngines,
    RunEninges {
        engines_list: Vec<String>,
        query: String,
    },
}

#[derive(Clone, Debug)]
pub enum Log {
    Error(String),
    Warning(String),
    Info(String),
}

#[derive(Clone, Debug)]
pub enum Respond {
    EngineResult { engine: String, output: String },
    Message(String),
    Error(String),
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn clear_queues_after_read() {
        // send a job
        MessagesBox::send_jobs(vec![Job::ListEngines]);
        assert_eq!(GLOBAL_INSTANCE.lock().unwrap().jobs.len(), 1);
        // recieve a job
        let jobs = MessagesBox::recieve_jobs();
        // make sure that the instance's jobs queue is clear
        // and that the recievied jobs vector is not.
        assert_eq!(GLOBAL_INSTANCE.lock().unwrap().jobs.len(), 0);
        assert_ne!(jobs.len(), GLOBAL_INSTANCE.lock().unwrap().jobs.len());
    }

    #[test]
    fn multithread_test() {
        thread::spawn(move || {
            for i in 1..10 {
                let msg = Respond::Message(i.to_string());
                let responds = vec![msg];
                println!("sent{:?}", responds);
                MessagesBox::send_responds(responds);
            }
            MessagesBox::finish();
        });

        let handle = thread::spawn(move || {
            while MessagesBox::is_pending() {
                let res = MessagesBox::recieve_responds();
                println!("recieved {:?}", res);
            }
        });

        handle.join().unwrap();
    }
}
