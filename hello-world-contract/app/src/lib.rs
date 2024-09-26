#![no_std]
use sails_rs::prelude::*;

pub mod services;
use services::my_service::MyService;

#[derive(Default)]
pub struct MyProgram;

#[program]
impl MyProgram {
    pub fn new() -> Self {
        Self
    }

    #[route("MyService")]
    pub fn my_service_svc(&self) -> MyService {
        MyService::new()
    }
}