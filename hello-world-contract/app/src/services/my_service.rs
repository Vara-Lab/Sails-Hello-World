use sails_rs::prelude::*;

#[derive(Default)]
pub struct MyService;

impl MyService {
    pub fn new() -> Self {
        Self
    }
}

#[service]
impl MyService {
    #[export]
    pub fn hello(&mut self) -> String {
        "Hello world!".to_string()
    }
}