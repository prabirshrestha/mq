use std::collections::HashMap;

use crate::JobHandler;

pub struct Consumer {
    handlers: HashMap<String, HashMap<String, Box<dyn JobHandler>>>,
}

impl Consumer {
    pub fn new() -> Self {
        Self {
            handlers: HashMap::new(),
        }
    }

    pub fn register(mut self, handler: impl JobHandler + 'static) -> Self {
        if !self.handlers.contains_key(handler.queue()) {
            self.handlers.insert(handler.queue().into(), HashMap::new());
        }

        self.handlers
            .get_mut(handler.queue())
            .unwrap()
            .insert(handler.kind().into(), Box::new(handler));

        self
    }

    pub fn handlers(&self) -> &HashMap<String, HashMap<String, Box<dyn JobHandler>>> {
        &self.handlers
    }
}
