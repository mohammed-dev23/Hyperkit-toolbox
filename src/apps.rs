use crate::backend::clean::{ExtractOptions, ExtractOptionsErr};
use crate::backend::safe::{Ugh, Ughv};
use crate::backend::standard::tell;
use chrono::*;
use colored::*;
use core::str;
use evalexpr::*;
use std::io::{self, Read, Write};
use std::{env, fs, path};
use tar::Archive;
use termtree::Tree;
use walkdir::WalkDir;
use zip::write::SimpleFileOptions;
use zip::{ZipArchive, ZipWriter};

use crate::backend::{
    clean::read_file_cont,
    safe::{ErrH, HyperkitError, Success},
};
use crate::toml::toml;
use base64::{
    prelude::{BASE64_STANDARD, BASE64_STANDARD_NO_PAD, BASE64_URL_SAFE},
    *,
};

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

pub fn time() {
    let path = tell();
    let username = toml().customization.username;
    let time = Local::now();
    println!("[{path:?}][{username}]~>[{time}]");
}

pub fn tar(
    flag: &str,
    the_name_of_the_file: &str,
    output_name: &str,
) -> std::result::Result<(), HyperkitError> {
    use tar::Builder;
    let path = tell();
    match flag.trim() {
        "--load" => {
            let mut open = fs::File::open(&the_name_of_the_file)
                .errh(Some("The file is not found".to_string()))
                .ughf()?;
            let make = fs::File::create(format!("{}.tar", output_name))
                .errh(Some("Couldn`t make the file".to_string()))
                .ughf()?;
            let ifdir = open
                .metadata()
                .errh(Some("The file is not found".to_string()))
                .ughf()?;
            if ifdir.is_dir() == true {
                let mut builder1 = Builder::new(make);
                let _apd = builder1
                    .append_dir_all(&output_name, path::Path::new(&the_name_of_the_file))
                    .errh(Some("Couldn't build the arcive".to_string()))
                    .ughf()?;
                let _finsh = builder1
                    .into_inner()
                    .errh(Some("Couldn't build the arcive".to_string()))
                    ._success_res("Tar completed", "loaded successfully")
                    .ughf()?;
            } else {
                let mut builder2 = Builder::new(make);
                let _ap = builder2
                    .append_file(&output_name, &mut open)
                    .errh(Some("Couldn`t build the arcive".to_string()))
                    .ughf()?;
                let _finsh = builder2
                    .into_inner()
                    .errh(Some("Couldn't build the arcive".to_string()))
                    ._success_res("Tar completed", "loaded successfully")
                    .ughf()?;
            }
        }
        "--Unload" => {
            let open = fs::File::open(the_name_of_the_file)
                .errh(Some("NotFound".to_string()))
                .ughf()?;

            let mut arc = Archive::new(open);
            arc.unpack(output_name)
                .errh(Some("Couldn't build the arcive".to_string()))
                ._success_res("Tar completed", "Unloaded successfully")
                .ughf()?;
        }
        _ => {
            println!(
                "[{path:?}]~>{}: due to [{}]",
                "Error".red().bold(),
                "No Flag was supplied".red().bold()
            );
        }
    }
    Ok(())
}

pub struct ZipArg<'z> {
    pub n1: &'z str,
    pub n2: &'z str,
    pub n3: &'z str,
    pub f1: &'z str,
    pub f2: &'z str,
    pub f3: &'z str,
}

pub struct ZipDir<'z> {
    pub src_dir: &'z str,
    pub res_dir: &'z str,
}

pub fn zip(
    flags: &str,
    file_name: &str,
    ziparg: ZipArg,
    zipdir: ZipDir,
) -> std::result::Result<(), HyperkitError> {
    let tell = tell();
    let args = ziparg;

    match flags.trim() {
        "--New-Zip" => {
            let curr = env::current_dir()
                .errh(Some("Couldn't extract the path".to_string()))
                .ughf()?;
            let creat = std::fs::File::create(format!(
                "{}/{}",
                &curr.to_string_lossy().to_string(),
                &file_name
            ))
            .errh(Some(curr.to_string_lossy().to_string()))
            .ughf()?;

            let mut zip = zip::ZipWriter::new(creat);
            zip.add_directory("zip/", SimpleFileOptions::default())
                .errh(None)?;

            let config = SimpleFileOptions::default()
                .compression_method(zip::CompressionMethod::Stored)
                .unix_permissions(0o755);

            if args.n1 == "-N1" {
                zip.start_file(args.f1, config)
                    .errh(Some(args.f1.to_string()))
                    .ughf()?;
            }
            if args.n2 == "-N2" {
                zip.start_file(args.f2, config)
                    .errh(Some(args.f2.to_string()))
                    .ughf()?;
            }
            if args.n3 == "-N3" {
                zip.start_file(args.f3, config)
                    .errh(Some(args.f3.to_string()))
                    .ughf()?;
            }

            if args.n1 == "-E1" {
                let mut open1 = fs::File::open(args.f1)
                    .errh(Some(args.f1.to_string()))
                    .ughf()?;
                let mut read = Vec::new();
                open1.read_to_end(&mut read).errh(None).ughf()?;

                zip.start_file(args.f1, config)
                    .errh(Some(args.f1.to_string()))
                    .ughf()?;
                zip.write_all(&mut read).errh(None).ughf()?;
            }
            if args.n2 == "-E2" {
                let mut open2 = fs::File::open(args.f2)
                    .errh(Some(args.f2.to_string()))
                    .ughf()?;
                let mut read = Vec::new();
                open2.read_to_end(&mut read).errh(None).ughf()?;

                zip.start_file(args.f2, config)
                    .errh(Some(args.f2.to_string()))
                    .ughf()?;
                zip.write_all(&mut read).errh(None).ughf()?;
            }
            if args.n3 == "-E3" {
                let mut open3 = fs::File::open(args.f3)
                    .errh(Some(args.f3.to_string()))
                    .ughf()?;
                let mut read = Vec::new();
                open3.read_to_end(&mut read).errh(None).ughf()?;

                zip.start_file(args.f3, config)
                    .errh(Some(args.f3.to_string()))
                    .ughf()?;
                zip.write_all(&mut read).errh(None).ughf()?;
            }
            zip.finish().errh(None).ughf()._success_res("Zip", "Done")?;
        }
        "--Zip-All" => {
            let creat = fs::File::create(zipdir.res_dir)
                .errh(Some(zipdir.res_dir.to_string()))
                .ughf()?;
            let mut zipdirr = ZipWriter::new(creat);

            let config = SimpleFileOptions::default()
                .compression_method(zip::CompressionMethod::Stored)
                .unix_permissions(0o755);

            let src_dir = path::Path::new(zipdir.src_dir);

            for i in WalkDir::new(src_dir).into_iter().filter_map(|e| e.ok()) {
                let path = i.path();
                let name = path
                    .strip_prefix(src_dir)
                    .ok()
                    .ok_or(HyperkitError::ArchiveErr(
                        crate::backend::safe::ArchiveErr::StripPrefixErr(None),
                    ))
                    .ughf()?
                    .to_str()
                    .extract();

                if name.is_empty() {
                    continue;
                }

                if path.is_file() {
                    println!(
                        "[{tell:?}]~>{}: {}",
                        "Adding file".bright_yellow().bold(),
                        name.bright_cyan().bold()
                    );
                    zipdirr.start_file(name, config).errh(None).ughf()?;
                    let mut open = fs::File::open(path).errh(None).ughf()?;
                    io::copy(&mut open, &mut zipdirr).errh(None).ughf()?;
                } else if path.is_dir() {
                    println!(
                        "[{tell:?}]~>{}: {}",
                        "Adding directory".bright_yellow().bold(),
                        name.bright_cyan().bold()
                    );
                    zipdirr
                        .add_directory(name, config)
                        .errh(Some(name.to_string()))
                        .ughf()?;
                }
            }
            zipdirr
                .finish()
                .errh(Some(zipdir.src_dir.to_string()))
                .ughf()
                ._success_res("Zip file created successfully", zipdir.res_dir)?;
        }
        "--extract" => {
            let path = path::Path::new(zipdir.src_dir);
            let outp = path::Path::new(zipdir.res_dir);

            let open = fs::File::open(path)
                .errh(Some(zipdir.src_dir.to_string()))
                .ughf()?;
            let mut arc = ZipArchive::new(open)
                .errh(Some(zipdir.src_dir.to_string()))
                .ughf()?;

            for i in 0..arc.len() {
                let mut file = arc.by_index(i).errh(None).ughf()?;
                let out = match file.enclosed_name() {
                    Some(o) => outp.join(o),
                    None => {
                        continue;
                    }
                };

                if file.name().ends_with('/') {
                    fs::create_dir_all(&out)
                        .errh(Some(out.to_string_lossy().to_string()))
                        .ughf()
                        ._success_res("Zip", "extracted!")?;
                } else {
                    if let Some(o) = out.parent() {
                        if !o.exists() {
                            fs::create_dir_all(o)
                                .errh(Some(o.to_string_lossy().to_string()))
                                .ughf()
                                ._success_res("Zip", "extracted!")?;
                        }
                    }
                    let mut outf = fs::File::create(&out)
                        .errh(Some(out.to_string_lossy().to_string()))
                        .ughf()?;
                    io::copy(&mut file, &mut outf)
                        .errh(Some(
                            format!("{}|{}", zipdir.src_dir, zipdir.res_dir).to_string(),
                        ))
                        .ughv();
                }
                #[cfg(unix)]
                {
                    use std::os::unix::fs::PermissionsExt;
                    if let Some(mode) = file.unix_mode() {
                        fs::set_permissions(&out, fs::Permissions::from_mode(mode))
                            .errh(Some(mode.to_string()))
                            .ughf()?;
                    }
                }
            }
        }
        _ => {
            println!(
                "[{tell:?}]~>{}: due to [{}]",
                "Error".red().bold(),
                "No Flag was supplied".red().bold()
            );
        }
    }
    Ok(())
}

pub struct FileZipArg<'s> {
    pub f1: &'s str,
    pub f2: &'s str,
    pub f3: &'s str,
}

pub fn cli_zip(
    op: &str,
    files: FileZipArg,
    file_name: &str,
    src_dir: &str,
    res_dir: &str,
) -> std::result::Result<(), HyperkitError> {
    let tell = tell();
    match op.trim() {
        "New-Zip" => {
            let curr = env::current_dir()
                .errh(Some("Couldn't extract the path".to_string()))
                .ughf()?;
            let creat = std::fs::File::create(format!(
                "{}/{}",
                &curr.to_string_lossy().to_string(),
                &file_name
            ))
            .errh(Some(curr.to_string_lossy().to_string()))
            .ughf()?;

            let mut zip = zip::ZipWriter::new(creat);
            zip.add_directory("zip/", SimpleFileOptions::default())
                .errh(None)?;

            let config = SimpleFileOptions::default()
                .compression_method(zip::CompressionMethod::Stored)
                .unix_permissions(0o755);

            if files.f1.len() > 0 {
                let open1 = fs::File::open(files.f1).errh(Some(files.f1.to_string()));
                if let Ok(mut o) = open1 {
                    let mut read = Vec::new();
                    o.read_to_end(&mut read).errh(None).ughf()?;

                    zip.start_file(files.f1, config)
                        .errh(Some(files.f1.to_string()))
                        .ughf()?;
                    zip.write_all(&mut read).errh(None).ughf()?
                } else {
                    zip.start_file(files.f1, config)
                        .errh(Some(files.f1.to_string()))
                        .ughf()?;
                }
            }
            if files.f2.len() > 0 {
                let open2 = fs::File::open(files.f2).errh(Some(files.f2.to_string()));
                if let Ok(mut o) = open2 {
                    let mut read = Vec::new();
                    o.read_to_end(&mut read).errh(None).ughf()?;

                    zip.start_file(files.f2, config)
                        .errh(Some(files.f1.to_string()))
                        .ughf()?;
                    zip.write_all(&mut read).errh(None).ughf()?
                } else {
                    zip.start_file(files.f2, config)
                        .errh(Some(files.f2.to_string()))
                        .ughf()?;
                }
            }
            if files.f3.len() > 0 {
                let open3 = fs::File::open(files.f3).errh(Some(files.f3.to_string()));
                if let Ok(mut o) = open3 {
                    let mut read = Vec::new();
                    o.read_to_end(&mut read).errh(None).ughf()?;

                    zip.start_file(files.f3, config)
                        .errh(Some(files.f1.to_string()))
                        .ughf()?;
                    zip.write_all(&mut read).errh(None).ughf()?
                } else {
                    zip.start_file(files.f3, config)
                        .errh(Some(files.f3.to_string()))
                        .ughf()?;
                }
            }
        }
        "Zip-All" => {
            let creat = fs::File::create(res_dir)
                .errh(Some(res_dir.to_string()))
                .ughf()?;
            let mut zipdirr = ZipWriter::new(creat);

            let config = SimpleFileOptions::default()
                .compression_method(zip::CompressionMethod::Stored)
                .unix_permissions(0o755);

            let src_dir = path::Path::new(src_dir);

            for i in WalkDir::new(src_dir).into_iter().filter_map(|e| e.ok()) {
                let path = i.path();
                let name = path
                    .strip_prefix(src_dir)
                    .ok()
                    .ok_or(HyperkitError::ArchiveErr(
                        crate::backend::safe::ArchiveErr::StripPrefixErr(None),
                    ))
                    .ughf()?
                    .to_str()
                    .extract();

                if name.is_empty() {
                    continue;
                }
                if path.is_file() {
                    println!(
                        "[{tell:?}]~>{}: {}",
                        "Adding file".bright_yellow().bold(),
                        name.bright_cyan().bold()
                    );
                    zipdirr.start_file(name, config).errh(None).ughf()?;
                    let mut open = fs::File::open(path).errh(None).ughf()?;
                    io::copy(&mut open, &mut zipdirr).errh(None).ughf()?;
                } else if path.is_dir() {
                    println!(
                        "[{tell:?}]~>{}: {}",
                        "Adding directory".bright_yellow().bold(),
                        name.bright_cyan().bold()
                    );
                    zipdirr
                        .add_directory(name, config)
                        .errh(Some(name.to_string()))
                        .ughf()?;
                }
            }
            zipdirr
                .finish()
                .errh(Some(src_dir.to_string_lossy().to_string()))
                .ughf()
                ._success_res("Zip file created successfully", res_dir)?;
        }
        "extract" => {
            let path = path::Path::new(src_dir);
            let outp = path::Path::new(res_dir);

            let open = fs::File::open(path)
                .errh(Some(src_dir.to_string()))
                .ughf()?;
            let mut arc = ZipArchive::new(open)
                .errh(Some(src_dir.to_string()))
                .ughf()?;

            for i in 0..arc.len() {
                let mut file = arc.by_index(i).errh(None).ughf()?;
                let out = match file.enclosed_name() {
                    Some(o) => outp.join(o),
                    None => {
                        continue;
                    }
                };

                if file.name().ends_with('/') {
                    fs::create_dir_all(&out)
                        .errh(Some(out.to_string_lossy().to_string()))
                        .ughf()
                        ._success_res("Zip", "extracted!")?;
                } else {
                    if let Some(o) = out.parent() {
                        if !o.exists() {
                            fs::create_dir_all(o)
                                .errh(Some(o.to_string_lossy().to_string()))
                                .ughf()
                                ._success_res("Zip", "extracted!")?;
                        }
                    }
                    let mut outf = fs::File::create(&out)
                        .errh(Some(out.to_string_lossy().to_string()))
                        .ughf()?;
                    io::copy(&mut file, &mut outf)
                        .errh(Some(format!("{}|{}", src_dir, res_dir).to_string()))
                        .ughv();
                }
                #[cfg(unix)]
                {
                    use std::os::unix::fs::PermissionsExt;
                    if let Some(mode) = file.unix_mode() {
                        fs::set_permissions(&out, fs::Permissions::from_mode(mode))
                            .errh(Some(mode.to_string()))
                            .ughf()?;
                    }
                }
            }
        }
        _ => todo!(),
    }
    Ok(())
}

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

fn help_tree<P: AsRef<std::path::Path>>(path: P) -> String {
    let p = path
        .as_ref()
        .file_name()
        .extract()
        .to_str()
        .extract()
        .bright_green()
        .bold()
        .to_string();
    p
}

pub fn treee<P: AsRef<std::path::Path>>(
    path: P,
) -> std::result::Result<Tree<String>, HyperkitError> {
    let read_dir = fs::read_dir(&path).errh(Some(path.as_ref().to_string_lossy().to_string()))?;
    let mut tree = Tree::new(help_tree(path.as_ref().canonicalize().errh(None)?));

    for i in read_dir {
        let i = i.errh(None)?;
        let dir = i.metadata().errh(None)?;

        if dir.is_dir() {
            tree.push(treee(i.path())?);
        } else {
            tree.push(Tree::new(help_tree(i.path())));
        }
    }
    println!("{tree}");
    Ok(tree)
}

fn get_mem(i: &str) -> std::result::Result<i64, HyperkitError> {
    let mem: Vec<&str> = i.split(':').collect();
    let get_mem = mem
        .get(1)
        .extract_err(None)
        .ughf()?
        .replace("kB", "")
        .trim()
        .parse::<i64>()
        .errh(None)
        .ughv();

    Ok(get_mem)
}

fn get_os(o: &str) -> std::result::Result<&str, HyperkitError> {
    let os: Vec<&str> = o.split('=').collect();
    let get_os = os
        .get(1)
        .extract_err(Some("get_os".to_string()))
        .ughf()?
        .trim_matches('"');

    Ok(get_os)
}

fn open_file_bat(path: &str) -> std::result::Result<std::fs::File, HyperkitError> {
    let open = fs::File::open(path).errh(Some(path.to_string())).ughf()?;
    Ok(open)
}

pub fn yank(flag: &str) -> std::result::Result<(), HyperkitError> {
    pub const KB_GB: i64 = 1_048_576;
    let tell = tell();

    match flag {
        "mem" => {
            let mut open = fs::File::open("/proc/meminfo").errh(None).ughf()?;
            let mut read = String::new();
            open.read_to_string(&mut read).errh(None).ughv();

            let mem: Vec<&str> = read.lines().collect();

            let memtotal = if let Some(o) = mem.iter().find(|s| s.starts_with("MemTotal")) {
                let get_mem_total = get_mem(*o)?;
                get_mem_total / KB_GB
            } else {
                0
            };

            let memfree = if let Some(o) = mem.iter().find(|s| s.starts_with("MemFree")) {
                let get_mem_free = get_mem(*o)?;
                get_mem_free / KB_GB
            } else {
                0
            };

            let memavailable = if let Some(o) = mem.iter().find(|s| s.starts_with("MemAvailable")) {
                let get_mem_available = get_mem(*o)?;
                get_mem_available / KB_GB
            } else {
                0
            };

            let cached_mem = if let Some(o) = mem.iter().find(|s| s.starts_with("Cached")) {
                let get_mem_chched = get_mem(*o)?;
                get_mem_chched / KB_GB
            } else {
                0
            };

            let active_mem = if let Some(o) = mem.iter().find(|s| s.starts_with("Active")) {
                let get_active_mem = get_mem(*o)?;
                get_active_mem / KB_GB
            } else {
                0
            };

            let inactive_mem = if let Some(o) = mem.iter().find(|s| s.starts_with("Inactive")) {
                let get_inactive_mem = get_mem(*o)?;
                get_inactive_mem / KB_GB
            } else {
                0
            };

            let swaptotal_mem = if let Some(o) = mem.iter().find(|s| s.starts_with("SwapTotal")) {
                let get_swaptotal_mem = get_mem(*o)?;
                get_swaptotal_mem / KB_GB
            } else {
                0
            };

            let swapfree_mem = if let Some(o) = mem.iter().find(|s| s.starts_with("SwapFree")) {
                let get_swapfree_mem = get_mem(*o)?;
                get_swapfree_mem / KB_GB
            } else {
                0
            };

            println!(
                "   -[{}] = ~{}{}~ ",
                "Total Memory".bright_cyan().bold(),
                memtotal.to_string().bright_yellow().bold(),
                "GB".bright_yellow().bold()
            );
            println!(
                "   -[{}] = ~{}{}~ ",
                "Free Memory".bright_cyan().bold(),
                memfree.to_string().bright_yellow().bold(),
                "GB".bright_yellow().bold()
            );
            println!(
                "   -[{}] = ~{}{}~ ",
                "Available Memory".bright_cyan().bold(),
                memavailable.to_string().bright_yellow().bold(),
                "GB".bright_yellow().bold()
            );
            println!(
                "   -[{}] = ~{}{}~ ",
                "Cash Memory".bright_cyan().bold(),
                cached_mem.to_string().bright_yellow().bold(),
                "GB".bright_yellow().bold()
            );
            println!(
                "   -[{}] = ~{}{}~ ",
                "Used Memory".bright_cyan().bold(),
                active_mem.to_string().bright_yellow().bold(),
                "GB".bright_yellow().bold()
            );
            println!(
                "   -[{}] = ~{}{}~ ",
                "Unused Memory".bright_cyan().bold(),
                inactive_mem.to_string().bright_yellow().bold(),
                "GB".bright_yellow().bold()
            );
            println!(
                "   -[{}] = ~{}{}~ ",
                "Swap Total Memory".bright_cyan().bold(),
                swaptotal_mem.to_string().bright_yellow().bold(),
                "GB".bright_yellow().bold()
            );
            println!(
                "   -[{}] = ~{}{}~ ",
                "Swap Free Memory".bright_cyan().bold(),
                swapfree_mem.to_string().bright_yellow().bold(),
                "GB".bright_yellow().bold()
            );
        }
        "os" => {
            let mut os = String::new();
            let mut open = fs::File::open("/etc/os-release")
                .errh(Some("os-release".to_string()))
                .ughf()?;
            open.read_to_string(&mut os).errh(None).ughv();

            let os: Vec<&str> = os.lines().collect();

            let os_name = if let Some(o) = os.iter().find(|s| s.starts_with("NAME")) {
                let os_name = get_os(*o)?;
                os_name
            } else {
                "NONE"
            };

            let os_version = if let Some(o) = os.iter().find(|s| s.starts_with("VERSION")) {
                let os_version = get_os(*o)?;
                os_version
            } else {
                "NONE"
            };

            let os_release_type = if let Some(o) = os.iter().find(|s| s.starts_with("RELEASE_TYPE"))
            {
                let os_release_type = get_os(*o)?;
                os_release_type
            } else {
                "NONE"
            };

            println!(
                "   -[{}] = ~{}~ ",
                "Software Name".bright_cyan().bold(),
                os_name.to_string().bright_yellow().bold()
            );

            println!(
                "   -[{}] = ~{}~ ",
                "Software Version".bright_cyan().bold(),
                os_version.to_string().bright_yellow().bold()
            );

            println!(
                "   -[{}] = ~{}~ ",
                "Software Release Type".bright_cyan().bold(),
                os_release_type.to_string().bright_yellow().bold()
            );
        }
        "battery" => {
            let mut bat = String::new();

            let mut f1 = open_file_bat("/sys/class/power_supply/BAT0/health").ughf()?;
            let mut f2 = open_file_bat("/sys/class/power_supply/BAT0/capacity").ughf()?;
            let mut f3 = open_file_bat("/sys/class/power_supply/BAT0/model_name").ughf()?;
            let mut f4 = open_file_bat("/sys/class/power_supply/BAT0/status").ughf()?;
            let mut f5 = open_file_bat("/sys/class/power_supply/BAT0/hwmon1/name").ughf()?;

            f1.read_to_string(&mut bat).errh(None).ughv();
            f2.read_to_string(&mut bat).errh(None).ughv();
            f3.read_to_string(&mut bat).errh(None).ughv();
            f4.read_to_string(&mut bat).errh(None).ughv();
            f5.read_to_string(&mut bat).errh(None).ughv();

            let bat: Vec<&str> = bat.lines().collect();

            let bat_health = if let Some(o) = bat.get(0) { o } else { "NONE" };

            let bat_capacity = if let Some(o) = bat.get(1) { o } else { "NONE" };

            let bat_model_name = if let Some(o) = bat.get(2) { o } else { "NONE" };

            let bat_status = if let Some(o) = bat.get(3) { o } else { "NONE" };

            let bat_name = if let Some(o) = bat.get(4) { o } else { "NONE" };

            println!(
                "   -[{}] = ~{}~ ",
                "Battery Health".bright_cyan().bold(),
                bat_health.to_string().bright_yellow().bold()
            );

            println!(
                "   -[{}] = ~{}{}~ ",
                "Battery capacity".bright_cyan().bold(),
                bat_capacity.to_string().bright_yellow().bold(),
                "%".bright_green().bold()
            );

            println!(
                "   -[{}] = ~{}~ ",
                "Battery Model Name".bright_cyan().bold(),
                bat_model_name.to_string().bright_yellow().bold()
            );

            println!(
                "   -[{}] = ~{}~ ",
                "Battery State".bright_cyan().bold(),
                bat_status.to_string().bright_yellow().bold()
            );

            println!(
                "   -[{}] = ~{}~ ",
                "Battery Name".bright_cyan().bold(),
                bat_name.to_string().bright_yellow().bold()
            );
        }
        "temp" => {
            let mut cpu_temp = String::new();

            let mut core_package_id_0_temp =
                open_file_bat("/sys/class/hwmon/hwmon5/temp1_input").ughf()?;
            let mut core1_temp = open_file_bat("/sys/class/hwmon/hwmon5/temp2_input").ughf()?;
            let mut core2_temp = open_file_bat("/sys/class/hwmon/hwmon5/temp6_input").ughf()?;
            let mut core3_temp = open_file_bat("/sys/class/hwmon/hwmon5/temp10_input").ughf()?;
            let mut core4_temp = open_file_bat("/sys/class/hwmon/hwmon5/temp11_input").ughf()?;
            let mut core5_temp = open_file_bat("/sys/class/hwmon/hwmon5/temp12_input").ughf()?;
            let mut core6_temp = open_file_bat("/sys/class/hwmon/hwmon5/temp13_input").ughf()?;
            let mut core7_temp = open_file_bat("/sys/class/hwmon/hwmon5/temp14_input").ughf()?;
            let mut core8_temp = open_file_bat("/sys/class/hwmon/hwmon5/temp15_input").ughf()?;
            let mut core9_temp = open_file_bat("/sys/class/hwmon/hwmon5/temp16_input").ughf()?;
            let mut core10_temp = open_file_bat("/sys/class/hwmon/hwmon5/temp17_input").ughf()?;

            core_package_id_0_temp
                .read_to_string(&mut cpu_temp)
                .errh(None)
                .ughv();
            core1_temp.read_to_string(&mut cpu_temp).errh(None).ughv();
            core2_temp.read_to_string(&mut cpu_temp).errh(None).ughv();
            core3_temp.read_to_string(&mut cpu_temp).errh(None).ughv();
            core4_temp.read_to_string(&mut cpu_temp).errh(None).ughv();
            core5_temp.read_to_string(&mut cpu_temp).errh(None).ughv();
            core6_temp.read_to_string(&mut cpu_temp).errh(None).ughv();
            core7_temp.read_to_string(&mut cpu_temp).errh(None).ughv();
            core8_temp.read_to_string(&mut cpu_temp).errh(None).ughv();
            core9_temp.read_to_string(&mut cpu_temp).errh(None).ughv();
            core10_temp.read_to_string(&mut cpu_temp).errh(None).ughv();

            let cpu_temp: Vec<&str> = cpu_temp.lines().collect();

            let all = if let Some(o) = cpu_temp.get(0) {
                o.parse::<i64>().errh(None).ughv() / 1000
            } else {
                0
            };

            let core1 = if let Some(o) = cpu_temp.get(1) {
                o.parse::<i64>().errh(None).ughv() / 1000
            } else {
                0
            };

            let core2 = if let Some(o) = cpu_temp.get(2) {
                o.parse::<i64>().errh(None).ughv() / 1000
            } else {
                0
            };

            let core3 = if let Some(o) = cpu_temp.get(3) {
                o.parse::<i64>().errh(None).ughv() / 1000
            } else {
                0
            };

            let core4 = if let Some(o) = cpu_temp.get(4) {
                o.parse::<i64>().errh(None).ughv() / 1000
            } else {
                0
            };

            let core5 = if let Some(o) = cpu_temp.get(5) {
                o.parse::<i64>().errh(None).ughv() / 1000
            } else {
                0
            };

            let core6 = if let Some(o) = cpu_temp.get(6) {
                o.parse::<i64>().errh(None).ughv() / 1000
            } else {
                0
            };

            let core7 = if let Some(o) = cpu_temp.get(7) {
                o.parse::<i64>().errh(None).ughv() / 1000
            } else {
                0
            };

            let core8 = if let Some(o) = cpu_temp.get(8) {
                o.parse::<i64>().errh(None).ughv() / 1000
            } else {
                0
            };

            let core9 = if let Some(o) = cpu_temp.get(9) {
                o.parse::<i64>().errh(None).ughv() / 1000
            } else {
                0
            };

            let core10 = if let Some(o) = cpu_temp.get(10) {
                o.parse::<i64>().errh(None).ughv() / 1000
            } else {
                0
            };

            println!("          {}", "---CPU---".bright_cyan().bold());
            println!(
                "   -[{}] = ~{}{}~ ",
                "Overall Cpu Temp".bright_cyan().bold(),
                all.to_string().bright_yellow().bold(),
                "C".bright_yellow().bold()
            );
            println!(
                "   -[{}] = ~{}{}~ ",
                "Core1 Temp".bright_cyan().bold(),
                core1.to_string().bright_yellow().bold(),
                "C".bright_yellow().bold()
            );
            println!(
                "   -[{}] = ~{}{}~ ",
                "Core2 Temp".bright_cyan().bold(),
                core2.to_string().bright_yellow().bold(),
                "C".bright_yellow().bold()
            );
            println!(
                "   -[{}] = ~{}{}~ ",
                "Core3 Temp".bright_cyan().bold(),
                core3.to_string().bright_yellow().bold(),
                "C".bright_yellow().bold()
            );
            println!(
                "   -[{}] = ~{}{}~ ",
                "Core4 Temp".bright_cyan().bold(),
                core4.to_string().bright_yellow().bold(),
                "C".bright_yellow().bold()
            );
            println!(
                "   -[{}] = ~{}{}~ ",
                "Core5 Temp".bright_cyan().bold(),
                core5.to_string().bright_yellow().bold(),
                "C".bright_yellow().bold()
            );
            println!(
                "   -[{}] = ~{}{}~ ",
                "Core6 Temp".bright_cyan().bold(),
                core6.to_string().bright_yellow().bold(),
                "C".bright_yellow().bold()
            );
            println!(
                "   -[{}] = ~{}{}~ ",
                "Core7 Temp".bright_cyan().bold(),
                core7.to_string().bright_yellow().bold(),
                "C".bright_yellow().bold()
            );
            println!(
                "   -[{}] = ~{}{}~ ",
                "Core8 Temp".bright_cyan().bold(),
                core8.to_string().bright_yellow().bold(),
                "C".bright_yellow().bold()
            );
            println!(
                "   -[{}] = ~{}{}~ ",
                "Core9 Temp".bright_cyan().bold(),
                core9.to_string().bright_yellow().bold(),
                "C".bright_yellow().bold()
            );
            println!(
                "   -[{}] = ~{}{}~ ",
                "Core10 Temp".bright_cyan().bold(),
                core10.to_string().bright_yellow().bold(),
                "C".bright_yellow().bold()
            );
        }
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
