use super::dal_service::{DalService, ServiceStatus};

#[derive(Debug)]
pub struct PollService {
    status: ServiceStatus,
    name:String,
}

impl DalService for PollService {
    fn start(&mut self) -> Result<i32, i32> {
        println!("poll service started");
        self.status = ServiceStatus::Start;
        Ok(0)
    }
    fn stop(&mut self) -> Result<i32, i32> {
        println!("poll service stopped");
        self.status = ServiceStatus::Stop;
        Ok(0)
    }
}

pub fn new_poll_service(name: String) -> PollService {
    PollService {
        status: ServiceStatus::Stop,
        name:name,
    }
}