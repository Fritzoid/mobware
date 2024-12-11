use actix::prelude::*;
use log;

pub struct Developer {
    pub name: String,
}

impl Actor for Developer {
    type Context = Context<Self>;

    fn started(&mut self, _: &mut Self::Context) {
        log::info!("Developer {} started", self.name);
    }
}

impl Developer {
    pub fn new(name: String) -> Developer {
        Developer { name }
    }
}
