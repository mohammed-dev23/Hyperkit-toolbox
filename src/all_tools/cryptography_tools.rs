use crate::backend::safe::{Ugh, Ughv};
use crate::backend::standard::tell;
use crate::backend::{
    clean::read_file_cont,
    safe::{ErrH, HyperkitError, Success},
};
use aes_gcm::Aes256Gcm;
use aes_gcm::aead::rand_core::RngCore;
use argon2::Argon2;
use chacha20poly1305::{AeadInPlace, Key};
use colored::*;
use core::str;
use sha2::digest::generic_array::{ArrayLength, GenericArray};
use sha2::{Digest, Sha256, Sha512};
use std::io::Read;
use std::{fs, io};

use chacha20poly1305::{
    ChaCha20Poly1305, Nonce,
    aead::{AeadCore, KeyInit, OsRng},
};

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

impl OutputFormat for md5::Digest {
    fn format(&self, format: &str) -> std::result::Result<String, HyperkitError> {
        let formated = match format {
            "base64" => Ok(BASE64_STANDARD.encode(**self)),
            "hex" => Ok(hex::encode(**self)),
            _ => Err(HyperkitError::CryptographyErr(
                crate::backend::safe::CryptographyErr::UnsupportedFormat(None),
            )),
        };
        formated
    }
}

impl OutputFormat for Vec<u8> {
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

fn hash_md5_and_write_to_file(
    file: &str,
    output_file_name: &str,
    format_type: &str,
) -> std::result::Result<(), HyperkitError> {
    let mut open_file = fs::File::open(&file).errh(Some(file.to_string())).ughf()?;
    let mut md5_hash = md5::Context::new();
    io::copy(&mut open_file, &mut md5_hash).errh(None).ughf()?;
    let md5 = md5_hash.finalize().format(format_type).ughf()?;
    fs::File::create(output_file_name).errh(None).ughf()?;
    fs::write(output_file_name, md5).errh(None).ughv();
    Ok(())
}

fn enc_dec_chacha20poly1305(
    file: &str,
    output_file_name: &str,
    passwored: &str,
    op: &str,
) -> std::result::Result<(), HyperkitError> {
    let tell = tell();

    let mut salt = [0u8; 16];
    OsRng.fill_bytes(&mut salt);
    let mut out_passwored = [0u8; 32];

    Argon2::default()
        .hash_password_into(passwored.as_bytes(), &salt, &mut out_passwored)
        .errh(None)
        .ughf()?;

    let key = Key::from_slice(&out_passwored);
    let cipher = ChaCha20Poly1305::new(&key);
    let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng);

    let mut buffer: Vec<u8> = Vec::new();

    let mut open_file = fs::File::open(&file).errh(None).ughf()?;
    let mut stored = Vec::new();
    open_file.read_to_end(&mut stored).errh(None).ughf()?;

    buffer.extend_from_slice(&stored);

    match op {
        "--seal" => {
            cipher
                .encrypt_in_place(&nonce, b"", &mut buffer)
                .map_err(|_| {
                    HyperkitError::CryptographyErr(
                        crate::backend::safe::CryptographyErr::UnsupportedFormat(None),
                    )
                })
                .ughf()?;

            let mut out = Vec::new();
            out.extend_from_slice(&salt);
            out.extend_from_slice(&nonce);
            out.extend_from_slice(&buffer);

            fs::write(output_file_name, out).errh(None).ughv();
        }
        "--unseal" => {
            let data = fs::read(file).errh(Some(file.to_string())).ughf()?;

            let (salt, rest) = data.split_at(16);
            let mut out_pass_dec = [0u8; 32];

            Argon2::default()
                .hash_password_into(passwored.as_bytes(), salt, &mut out_pass_dec)
                .errh(None)
                .ughf()?;

            let key = Key::from_slice(&mut out_pass_dec);

            let cipher_dec = ChaCha20Poly1305::new(&key);

            let (noncee, ciphertxt) = rest.split_at(12);
            let mut buffer = ciphertxt.to_vec();

            let noncee = Nonce::from_slice(&noncee);

            cipher_dec
                .decrypt_in_place(&noncee, b"", &mut buffer)
                .map_err(|_| {
                    HyperkitError::CryptographyErr(
                        crate::backend::safe::CryptographyErr::DecryptionFailed(None),
                    )
                })
                .ughf()?;

            fs::write(output_file_name, buffer).errh(None).ughv();
        }
        _ => println!(
            "[{tell:?}]~>{} due to [{}]",
            "Error".red().bold(),
            "missing opration".red().bold()
        ),
    }

    Ok(())
}

fn enc_dec_aes(
    file: &str,
    output_file_name: &str,
    passwored: &str,
    op: &str,
) -> std::result::Result<(), HyperkitError> {
    let tell = tell();

    let mut salt = [0u8; 16];
    OsRng.fill_bytes(&mut salt);
    let mut out_passwored = [0u8; 32];

    Argon2::default()
        .hash_password_into(passwored.as_bytes(), &salt, &mut out_passwored)
        .errh(None)
        .ughf()?;

    let key = Key::from_slice(&out_passwored);
    let cipher = Aes256Gcm::new(&key);
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);

    let mut buffer: Vec<u8> = Vec::new();

    let mut open_file = fs::File::open(&file).errh(None).ughf()?;
    let mut stored = Vec::new();
    open_file.read_to_end(&mut stored).errh(None).ughf()?;

    buffer.extend_from_slice(&stored);

    match op {
        "--seal" => {
            cipher
                .encrypt_in_place(&nonce, b"", &mut buffer)
                .map_err(|_| {
                    HyperkitError::CryptographyErr(
                        crate::backend::safe::CryptographyErr::UnsupportedFormat(None),
                    )
                })
                .ughf()?;

            let mut out = Vec::new();
            out.extend_from_slice(&salt);
            out.extend_from_slice(&nonce);
            out.extend_from_slice(&buffer);

            fs::write(output_file_name, out).errh(None).ughv();
        }
        "--unseal" => {
            let data = fs::read(file).errh(Some(file.to_string())).ughf()?;

            let (salt, rest) = data.split_at(16);
            let mut out_pass_dec = [0u8; 32];

            Argon2::default()
                .hash_password_into(passwored.as_bytes(), salt, &mut out_pass_dec)
                .errh(None)
                .ughf()?;

            let key = Key::from_slice(&mut out_pass_dec);

            let cipher_dec = Aes256Gcm::new(&key);

            let (noncee, ciphertxt) = rest.split_at(12);
            let mut buffer = ciphertxt.to_vec();

            let noncee = Nonce::from_slice(&noncee);

            cipher_dec
                .decrypt_in_place(&noncee, b"", &mut buffer)
                .map_err(|_| {
                    HyperkitError::CryptographyErr(
                        crate::backend::safe::CryptographyErr::DecryptionFailed(None),
                    )
                })
                .ughf()?;

            fs::write(output_file_name, buffer).errh(None).ughv();
        }
        _ => println!(
            "[{tell:?}]~>{} due to [{}]",
            "Error".red().bold(),
            "missing opration".red().bold()
        ),
    }

    Ok(())
}

pub fn seal(
    type_: &str,
    output_format: &str,
    op: &str,
    file: &str,
    output_file_name: &str,
    pass: &str,
) -> std::result::Result<(), HyperkitError> {
    let tell = tell();
    match type_ {
        "sha256" => match op {
            "--seal" => hash_sha256_and_write_to_file(file, output_file_name, output_format)
                ._success_res("Seal", "Sealed Successfully")
                .ughv(),
            _ => println!(
                "[{tell:?}]~>{} due to [{}]",
                "Error".red().bold(),
                "This type of hashing is unsealable!".red().bold()
            ),
        },
        "sha512" => match op {
            "--seal" => hash_sha512_and_write_to_file(file, output_file_name, output_format)
                ._success_res("Seal", "Sealed Successfully!")
                .ughv(),
            _ => println!(
                "[{tell:?}]~>{} due to [{}]",
                "Error".red().bold(),
                "This type of hashing is unsealable!".red().bold()
            ),
        },
        "md5" => match op {
            "--seal" => hash_md5_and_write_to_file(file, output_file_name, output_format)
                ._success_res("Seal", "Sealed Successfully!")
                .ughv(),
            _ => println!(
                "[{tell:?}]~>{} due to [{}]",
                "Error".red().bold(),
                "This type of hashing is unsealable!".red().bold()
            ),
        },
        "chacha20poly1305" => {
            if pass.is_empty() {
                return Err(HyperkitError::MissingParameter(Some(
                    "Password".to_string(),
                )));
            }
            if output_format != "None" {
                return Err(HyperkitError::MissingParameter(Some(
                    "format is missing or not <None>".to_string(),
                )));
            }

            enc_dec_chacha20poly1305(file, output_file_name, pass, op)
                ._success_res("Seal", "Seal Successed!")
                .ughf()?;
        }
        "aes256gcm" => {
            if pass.is_empty() {
                return Err(HyperkitError::MissingParameter(Some(
                    "Password".to_string(),
                )));
            }
            if output_format != "None" {
                return Err(HyperkitError::MissingParameter(Some(
                    "format is missing or not <None>".to_string(),
                )));
            }

            enc_dec_aes(file, output_file_name, pass, op)
                ._success_res("Seal", "Seal Successed!")
                .ughf()?;
        }
        _ => {
            if type_.is_empty() {
                println!(
                    "[{tell:?}]~>{} due to [{} <{}>]",
                    "Error".red().bold(),
                    "Unsupported Type!".red().bold(),
                    type_.bright_yellow().bold()
                )
            } else {
                println!(
                    "[{tell:?}]~>{} due to [{}]",
                    "Error".red().bold(),
                    "missing type".red().bold()
                )
            }
        }
    }
    Ok(())
}
