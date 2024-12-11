use actix::prelude::*;
use log;

pub struct Orchestrator {
    pub name: String,
}

impl Actor for Orchestrator {
    type Context = Context<Self>;

    fn started(&mut self, _: &mut Self::Context) {
        log::info!("Orchestrator {} started", self.name);
    }
}

impl Orchestrator {
    pub fn new(name: String) -> Orchestrator {
        Orchestrator { name }
    }
}
