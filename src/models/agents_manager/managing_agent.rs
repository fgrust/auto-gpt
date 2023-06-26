use crate::models::agent_basic::basic_agent::BasicAgent;
use crate::models::agents::agent_traits::{FactSheet, SpecialFunctions};

#[allow(dead_code)]
#[derive(Debug)]
pub struct ManagingAgent {
    attributes: BasicAgent,
    factsheet: FactSheet,
    agent: Vec<Box<dyn SpecialFunctions>>,
}
