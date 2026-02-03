mod apps;
mod backend;
mod cli;
mod repl;
mod toml;
use crate::backend::safe::HyperkitError;
use crate::cli::cli::cli;
use crate::repl::repl;
use std::env::*;

fn main() -> std::result::Result<(), HyperkitError> {
    if args().len() > 1 { cli() } else { repl() }
}
