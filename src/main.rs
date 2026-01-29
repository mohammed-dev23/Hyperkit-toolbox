mod backend;
mod apps;
mod cli;
mod repl;
mod toml;
use std::env::*;
use crate::cli::cli::cli;
use crate::repl::repl;
use crate::backend::safe::HyperkitError;

fn main() -> std::result::Result<() , HyperkitError> {
    if args().len() > 1 {
        cli()
    }
    else {
        repl()
    }
}