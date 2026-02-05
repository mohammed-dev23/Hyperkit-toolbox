use crate::backend::standard::tell;
use crate::toml::toml;
use colored::*;
use evalexpr::eval;

pub fn calc(math: &String) {
    let path = tell();
    let username = toml().customization.username;

    let e = match eval(&math) {
        Ok(t) => t,

        Err(error) => {
            println!("[{path:?}]>{}: due to {error:?}", "Error".red());
            return;
        }
    };

    println!("[{path:?}]~[{username}]>[ \x1b[34m{e}\x1b[0m ]");
}
