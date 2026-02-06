use crate::backend::safe::Ugh;
use crate::backend::standard::tell;
use crate::backend::{
    clean::read_file_cont,
    safe::{ErrH, HyperkitError, Success},
};
use colored::*;
use core::str;
use std::fs;

use base64::{
    prelude::{BASE64_STANDARD, BASE64_STANDARD_NO_PAD, BASE64_URL_SAFE},
    *,
};

pub fn transmute(
    ttype: &str,
    flag: &str,
    the_name_of_the_file: &str,
    output_name: &str,
) -> std::result::Result<(), HyperkitError> {
    let path = tell();
    let readed = read_file_cont(the_name_of_the_file)?;
    match ttype.trim() {
        "base64-ST" => {
            match flag.trim() {
                "--enc" => {
                    let enc = BASE64_STANDARD.encode(&readed);

                    fs::write(&output_name, enc).errh(Some("couldn`t write the encoded codic to the file Consider trying another type".to_string()))._success_res("transmute" , "encoded successfully").ughf()?;
                }
                "--dec" => {
                    let dec = BASE64_STANDARD.decode(&readed.trim()).unwrap_or_default();

                    fs::write(&output_name, dec).errh(Some("couldn`t write the decoded codic to the file Consider trying another type".to_string()))._success_res("transmute" , "decoded successfully").ughf()?;
                }
                _ => {
                    println!(
                        "[{path:?}]~>{}: due to [{}]",
                        "Error".red().bold(),
                        "No Flag was supplied".red().bold()
                    );
                }
            }
        }
        "base64-PD" => {
            match flag.trim() {
                "--enc" => {
                    let enc = BASE64_STANDARD_NO_PAD.encode(&readed);

                    fs::write(&output_name, enc).errh(Some("couldn`t write the encoded codic to the file Consider trying another type".to_string()))._success_res("transmute" , "encoded successfully").ughf()?;
                }
                "--dec" => {
                    let dec = BASE64_STANDARD_NO_PAD.decode(&readed).unwrap_or_default();

                    fs::write(&output_name, dec).errh(Some("couldn`t write the decoded codic to the file Consider trying another type".to_string()))._success_res("transmute" , "decoded successfully").ughf()?
                }
                _ => {
                    println!(
                        "[{path:?}]~>{}: due to [{}]",
                        "Error".red().bold(),
                        "No Flag was supplied".red().bold()
                    );
                }
            }
        }
        "base64-URL" => {
            match flag.trim() {
                "--enc" => {
                    let enc = BASE64_URL_SAFE.encode(&readed);

                    fs::write(&output_name, enc).errh(Some("couldn`t write the encoded codic to the file Consider trying another type".to_string()))._success_res("transmute" , "encoded successfully").ughf()?;
                }
                "--dec" => {
                    let dec = BASE64_URL_SAFE.decode(&readed).unwrap_or_default();

                    fs::write(&output_name, dec).errh(Some("couldn`t write the decoded codic to the file Consider trying another type".to_string()))._success_res("transmute" , "decoded successfully").ughf()?;
                }
                _ => {
                    println!(
                        "[{path:?}]~>{}: due to [{}]",
                        "Error".red().bold(),
                        "No Flag was supplied".red().bold()
                    );
                }
            }
        }
        "hex" => {
            match flag.trim() {
                "--enc" => {
                    let enc = hex::encode(&readed);

                    fs::write(&output_name, enc).errh(Some("couldn`t write the encoded codic to the file Consider trying another type".to_string()))._success_res("transmute" , "encoded successfully").ughf()?;
                }
                "--dec" => {
                    let dec = hex::decode(&readed).unwrap_or_default();

                    fs::write(&output_name, dec).errh(Some("couldn`t write the decoded codic to the file Consider trying another type".to_string()))._success_res("transmute" , "decoded successfully").ughf()?;
                }
                _ => {
                    println!(
                        "[{path:?}]~>{}: due to [{}]",
                        "Error".red().bold(),
                        "No Flag was supplied".red().bold()
                    );
                }
            }
        }
        "HEX" => {
            match flag.trim() {
                "--enc" => {
                    let enc = hex::encode_upper(&readed);

                    fs::write(&output_name, enc).errh(Some("couldn`t write the encoded codic to the file Consider trying another type".to_string()))._success_res("transmute" , "encoded successfully").ughf()?;
                }
                "--dec" => {
                    let dec = hex::decode(&readed).unwrap_or_default();

                    fs::write(&output_name, dec).errh(Some("couldn`t write the decoded codic to the file Consider trying another type".to_string()))._success_res("transmute" , "decoded successfully").ughf()?;
                }
                _ => {
                    println!(
                        "[{path:?}]~>{}: due to [{}]",
                        "Error".red().bold(),
                        "No Flag was supplied".red().bold()
                    );
                }
            }
        }
        "url" => {
            match flag.trim() {
                "--enc" => {
                    let enc = urlencoding::encode(&readed).into_owned();

                    fs::write(&output_name, enc).errh(Some("couldn`t write the encoded codic to the file Consider trying another type".to_string()))._success_res("transmute" , "encoded successfully").ughf()?;
                }
                "--dec" => {
                    let dec = urlencoding::decode(&readed)
                        .unwrap_or_default()
                        .into_owned();

                    fs::write(&output_name, dec).errh(Some("couldn`t write the decoded codic to the file Consider trying another type".to_string()))._success_res("transmute" , "decoded successfully").ughf()?;
                }
                _ => {
                    println!(
                        "[{path:?}]~>{}: due to [{}]",
                        "Error".red().bold(),
                        "No Flag was supplied".red().bold()
                    );
                }
            }
        }
        _ => {
            println!(
                "[{path:?}]~>{}: due to [{}]",
                "Error".red().bold(),
                "No type was supplied".red().bold()
            );
        }
    }
    Ok(())
}
