#![allow(dead_code)]

pub mod cli;
pub mod run;

#[derive(Debug, Clone)]
pub struct ClientDefinition {
    name: String,
    version: String,
    image: String,
}
