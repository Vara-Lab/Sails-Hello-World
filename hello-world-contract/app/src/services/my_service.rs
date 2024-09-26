use sails_rs::prelude::*;

#[derive(Default)]
pub struct MyService;

#[service]
impl MyService {
    pub fn new() -> Self {
        Self
    }

    pub fn hello(&mut self) -> String {
        "Hello world!".to_string()
    }
}