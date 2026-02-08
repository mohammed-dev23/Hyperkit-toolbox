use crate::backend::safe::{Ugh, Ughv};
use crate::backend::standard::tell;
use crate::backend::{
    clean::read_file_cont,
    safe::{ErrH, HyperkitError, Success},
};
use colored::*;
use core::str;
use sha2::digest::generic_array::{ArrayLength, GenericArray};
use std::{fs, io};

use sha2::{Digest, Sha256, Sha512};

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

pub trait OutputFormat {
    fn format(&self, format: &str) -> std::result::Result<String, HyperkitError>;
}

impl<T, U: ArrayLength<T>> OutputFormat for GenericArray<T, U>
where
    GenericArray<T, U>: AsRef<[u8]>,
{
    fn format(&self, format: &str) -> std::result::Result<String, HyperkitError> {
        let formated = match format {
            "base64" => Ok(BASE64_STANDARD.encode(&self)),
            "hex" => Ok(hex::encode(&self)),
            _ => Err(HyperkitError::CryptographyErr(
                crate::backend::safe::CryptographyErr::UnsupportedFormat(None),
            )),
        };
        formated
    }
}

fn hash_sha256_and_write_to_file(
    file: &str,
    output_file_name: &str,
    format_type: &str,
) -> std::result::Result<(), HyperkitError> {
    let mut open_file = fs::File::open(&file).errh(Some(file.to_string())).ughf()?;
    let mut hasher = Sha256::new();
    io::copy(&mut open_file, &mut hasher).errh(None).ughf()?;
    let sha256 = hasher.finalize().format(format_type).ughf()?;
    fs::File::create(output_file_name).errh(None).ughf()?;
    fs::write(output_file_name, sha256).errh(None).ughv();
    Ok(())
}

fn hash_sha512_and_write_to_file(
    file: &str,
    output_file_name: &str,
    format_type: &str,
) -> std::result::Result<(), HyperkitError> {
    let mut open_file = fs::File::open(&file).errh(Some(file.to_string())).ughf()?;
    let mut hasher = Sha512::new();
    io::copy(&mut open_file, &mut hasher).errh(None).ughf()?;
    let sha512 = hasher.finalize().format(format_type).ughf()?;
    fs::File::create(output_file_name).errh(None).ughf()?;
    fs::write(output_file_name, sha512).errh(None).ughv();
    Ok(())
}

pub fn seal(
    type_: &str,
    output_format: &str,
    kind: &str,
    file: &str,
    output_file_name: &str,
) -> std::result::Result<(), HyperkitError> {
    let tell = tell();
    match type_ {
        "sha256" => match kind {
            "--file" => hash_sha256_and_write_to_file(file, output_file_name, output_format)
                ._success_res("Seal", "Sealed Successfully")
                .ughv(),
            _ => println!(
                "[{tell:?}]~>{} due to [{}]",
                "Error".red().bold(),
                "missing file kind".red().bold()
            ),
        },
        "sha512" => match kind {
            "--file" => hash_sha512_and_write_to_file(file, output_file_name, output_format)
                ._success_res("Seal", "Sealed Successfully!")
                .ughv(),
            _ => println!(
                "[{tell:?}]~>{} due to [{}]",
                "Error".red().bold(),
                "missing file kind".red().bold()
            ),
        },
        _ => println!(
            "[{tell:?}]~>{} due to [{}]",
            "Error".red().bold(),
            "missing type".red().bold()
        ),
    }
    Ok(())
}
