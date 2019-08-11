use super::dal_service::{DalService, ServiceStatus};

#[derive(Debug)]
pub struct DbService {
    status: ServiceStatus,
    name:String,
}

impl DalService for DbService {
    fn start(&mut self) -> Result<i32, i32> {
        println!("db service started");
        self.status = ServiceStatus::Start;
        Ok(0)
    }
    fn stop(&mut self) -> Result<i32, i32> {
        println!("db service stopped");
        self.status = ServiceStatus::Stop;
        Ok(0)
    }
}

pub fn new_db_service(name: String) -> DbService {
    DbService {
        status: ServiceStatus::Stop,
        name:name,
    }
}