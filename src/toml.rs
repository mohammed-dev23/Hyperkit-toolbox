use serde::{Deserialize};
use crate::backend::{clean::read_file_cont, safe::{Ughv}};

#[derive(Deserialize)]
pub struct  Config {
    pub dependencies:Dependencies,
    pub customization: Customization,
}
#[derive(Deserialize)]
pub struct Dependencies {
    pub historyfilepath:String
}

#[derive(Deserialize)]
pub struct Customization {
    pub username:String
}

pub fn toml() -> Config {
    let read_config = read_file_cont("/home/mohammed/programming/Rust/practice/Hyperkit/Hyper.toml").ughv();
    let toml_des : Config = toml::from_str(&read_config).unwrap();
    return toml_des
}