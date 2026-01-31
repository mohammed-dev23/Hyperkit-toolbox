use std::{fs, io::Read, mem::replace};

use serde::{Deserialize};
use crate::backend::{clean::read_file_cont, safe::{ErrH, HyperkitError, Ugh, Ughv}, standard::tell};

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

pub fn configer (flag:&str , username:&str , hispath:&str , opration:&str) -> std::result::Result<() , HyperkitError> {
    let tell = tell();
    let mut open = fs::File::open("/home/mohammed/programming/Rust/practice/Hyperkit/Hyper.toml").errh(None).ughf()?;

    match flag {
        "username" => {
            match opration {
                "--set" => {
                    let replacer = todo!();  
                },
                _ => {todo!()}
            }
        },
        "HisPath" => {
            match opration {
                "--set" => {

                }
                _ => {todo!()}
            }
        },
        _ => {todo!()}
    }
    Ok(())
}