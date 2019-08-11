pub mod service;
pub mod flow;

use service::rest_service::new_rest_service;
use service::db_service::new_db_service;
use service::poll_service::new_poll_service;
use service::dal_service::{DalService};

#[derive(Debug)]
pub struct App<'a> {
    services: Vec<Box<dyn DalService>>,
    name: &'a str,
}

impl<'a> App<'a> {
    pub fn start(&mut self) {
        //let mut rest_service = Box::new(new_rest_service("rest_service".to_string()));
        //let mut db_service = Box::new(new_db_service("db_service".to_string()));
        //let mut poll_service = Box::new(new_poll_service("poll_service".to_string()));
        self.services.push(Box::new(new_rest_service("rest_service".to_string())));
        self.services.push(Box::new(new_db_service("db_service".to_string())));
        self.services.push( Box::new(new_poll_service("poll_service".to_string())));

        for s in self.services.iter_mut() {
            if let Ok(0) = s.start() {
                ;
            }
        }
    }
}

pub fn new_app(name: &str) -> App {
    App {
        services: Vec::new(),
        name:name,
    }
}