pub struct MessagesBox{
    jobs:Vec<Job>,
    responds:Vec<Respond>,
    pending:bool
}

impl MessagesBox{
   
	pub fn init() -> Self{
		MessagesBox { jobs: vec![], responds: vec![], pending: false }
	}

	pub fn send_jobs(&mut self, jobs:Vec<Job>){
		self.jobs.extend(jobs);
		self.pending = true;
    }

     pub fn recieve_jobs(&mut self) -> Vec<Job>{
        let out = self.jobs.clone();
		self.jobs.clear();
		out
     }

	 pub fn send_responds(&mut self,responds:Vec<Respond>){
		self.responds.extend(responds);
	 }

	 pub fn finish(&mut self){
		self.pending = false;
	 }

     pub fn recieve_responds() -> Vec<Respond>{
        todo!()
     }
}


#[derive(Clone)]
pub enum Job{
	ListEngines,
	RunEninges{engines_list:Vec<String>,query:String},
}


pub enum Respond{
	EngineResult{engine:String,output:String},
	Message(String),
	Error(String),
}

#[cfg(test)]
mod messages_box_tests{
    use super::*;

	#[test]
	fn clear_queues_after_read(){
		// init
		let mut ipc = MessagesBox::init();
		// send a job
		ipc.send_jobs(vec![Job::ListEngines]);
		assert_eq!(ipc.jobs.len(),1);
		// recieve a job
		let jobs = ipc.recieve_jobs();
		// make sure that the instance's jobs queue is clear
		// and that the recievied jobs vector is not.
		assert_eq!(ipc.jobs.len(),0);
		assert_ne!(jobs.len(),ipc.jobs.len());
	}

}