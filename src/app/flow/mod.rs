//pub mod query_flow;
//pub mod create_flow;
//pub mod delete_flow;
//pub mod modify_flow;
pub mod flow_err;

use std::fmt::Debug;
use crate::tlv::record;

trait flow<T:Debug>:Debug {
    fn parse(&mut self) -> Result<record::record<T>, flow_err::FlowErr>;
}
