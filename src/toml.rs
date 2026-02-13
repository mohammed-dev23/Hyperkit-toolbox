use crate::backend::{
    clean::{ExtractOptions, read_file_cont},
    safe::{ErrH, HyperkitError, Success, Ugh, Ughv},
    standard::tell,
};
use colored::*;
use serde::Deserialize;
use std::{
    env::{self},
    fs,
    io::Read,
    path::PathBuf,
};
use walkdir::WalkDir;

#[derive(Deserialize)]
pub struct Config {
    pub dependencies: Dependencies,
    pub customization: Customization,
}
#[derive(Deserialize)]
pub struct Dependencies {
    pub historyfilepath: String,
    pub fangdirpath: String,
}

#[derive(Deserialize)]
pub struct Customization {
    pub username: String,
}

pub fn toml() -> Config {
    let config_file_path = connfig_file_finder().ughv();
    let read_config = read_file_cont(&config_file_path.to_str().extract()).ughv();
    let toml_des: Config = toml::from_str(&read_config).unwrap();
    return toml_des;
}

fn write_to_config(value: String) -> std::result::Result<(), HyperkitError> {
    let config_file_path = connfig_file_finder()?;
    let okay = std::fs::write(config_file_path, value).errh(None).ughf();
    okay
}

fn connfig_file_finder() -> std::result::Result<PathBuf, HyperkitError> {
    let home_dir = env::home_dir().extract();
    for i in WalkDir::new(home_dir.join(".config"))
        .into_iter()
        .map(|s| s.map_err(|s| s.into_io_error()))
    {
        let i = i.errh(Some("toml.rs/38".to_string()))?;

        if i.file_name() == "Hyper.toml" {
            return Ok(i.path().to_path_buf());
        }
    }
    Err(HyperkitError::from(HyperkitError::FileError(
        crate::backend::safe::FileError::FileNotFound(None),
    )))
}

fn help_configer_seter(
    username: &str,
    hispath: &str,
    fangdir: &str,
) -> std::result::Result<(), HyperkitError> {
    let mut done = String::new();

    let find_config = connfig_file_finder()?;

    fs::File::open(find_config)
        .errh(None)
        .ughf()?
        .read_to_string(&mut done)
        .errh(None)
        .ughv();

    let rep = done.replace(&toml().dependencies.fangdirpath, &fangdir);
    write_to_config(rep)?;

    if !username.contains("/") {
        let rep = done.replace(&toml().customization.username, &username);
        let okay = write_to_config(rep);
        return Ok(okay?);
    } else {
        let rep = done.replace(&toml().dependencies.historyfilepath, &hispath);
        let okay = write_to_config(rep);
        return Ok(okay?);
    }
}

pub fn configer(
    flag: &str,
    username: &str,
    hispath: &str,
    fangdir: &str,
    operation: &str,
) -> std::result::Result<(), HyperkitError> {
    let tell = tell();

    match flag {
        "username" => match operation {
            "--set" => {
                help_configer_seter(&username, &hispath, &fangdir)
                    ._success_res("Configer", "succeeded")
                    .ugh();
            }
            _ => {
                println!(
                    "[{tell:?}]~>{}: due to [{}]",
                    "Error".red().bold(),
                    "No operation was supplied".red().bold()
                );
            }
        },
        "hispath" => match operation {
            "--set" => {
                help_configer_seter(&username, &hispath, &fangdir)
                    ._success_res("Configer", "succeeded")
                    .ugh();
            }
            _ => {
                println!(
                    "[{tell:?}]~>{}: due to [{}]",
                    "Error".red().bold(),
                    "No operation was supplied".red().bold()
                );
            }
        },
        "fangdirpath" => match operation {
            "--set" => {
                help_configer_seter(&username, &hispath, &fangdir)
                    ._success_res("Configer", "succeeded")
                    .ugh();
            }
            _ => {
                println!(
                    "[{tell:?}]~>{}: due to [{}]",
                    "Error".red().bold(),
                    "No operation was supplied".red().bold()
                )
            }
        },
        _ => {
            println!(
                "[{tell:?}]~>{}: due to [{}]",
                "Error".red().bold(),
                "No flag was supplied".red().bold()
            );
        }
    }
    Ok(())
}
