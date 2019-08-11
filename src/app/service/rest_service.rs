use super::dal_service::{DalService, ServiceStatus};

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[derive(Debug)]
pub struct RestService {
    status: ServiceStatus,
    name: String,
}

impl DalService for RestService {
    fn start(&mut self) -> Result<i32, i32> {
        println!("rest service started");
        self.status = ServiceStatus::Start;

        rocket::ignite().mount("/", routes![index]).launch();

        Ok(0)
    }
    fn stop(&mut self) -> Result<i32, i32> {
        println!("rest service stopped");
        self.status = ServiceStatus::Stop;
        Ok(0)
    }
}

pub fn new_rest_service(name: String) -> RestService {
    RestService {
        status: ServiceStatus::Stop,
        name:name,
    }
}