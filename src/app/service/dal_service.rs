use std::fmt::Debug;

#[derive(Debug)]
pub enum ServiceStatus {
    Stop,
    Start,
}

pub trait DalService:Debug  {
    fn start(&mut self) -> Result<i32, i32>;
    fn stop(&mut self) -> Result<i32, i32>;
}
