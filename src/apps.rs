use core::str;
use std::io::{self, Read, Write};
use std::{ env, fs, path};
use termtree::Tree;
use colored::*;
use evalexpr::*;
use chrono::*;
use tar::{Archive};
use walkdir::WalkDir;
use zip::{ZipArchive, ZipWriter};
use zip::write::SimpleFileOptions;
use crate::backend::clean::{ExtractOptions};
use crate::backend::safe::{Ugh, Ughv};
use crate::backend::standard::tell;

use crate::backend::{clean::read_file_cont,safe::{ErrH, HyperkitError, Success}};
use base64::{prelude::{BASE64_STANDARD, BASE64_STANDARD_NO_PAD, BASE64_URL_SAFE}, *};

pub fn calc (math:&String) {
    let path = tell();

    let e = match eval(&math) {
        Ok(t) => t,
        
        Err(error) => {
            println!("[{path:?}]>{}: due to {error:?}" , "Error".red() );
            return;
        }
    };

    println!("[{path:?}]~>[ \x1b[34m{e}\x1b[0m ]");
}


pub fn time() {
    let path= tell();

    let time = Local::now();
    println!("[{path:?}]~>[{time}]");
}

pub fn tar(flag:&str , the_name_of_the_file:&str , output_name:&str) -> std::result::Result<() , HyperkitError> {
  use tar::{Builder};
  let path = tell();
            match flag.trim() {
                "--load" => {
                    let mut open = fs::File::open(&the_name_of_the_file).errh(Some("The file is not found".to_string())).ughf()?;
                    let make =  fs::File::create(format!("{}.tar", output_name)).errh(Some("Couldn`t make the file".to_string())).ughf()?;
                    let ifdir = open.metadata().errh(Some("The file is not found".to_string())).ughf()?;
                    if ifdir.is_dir() == true {
                        let mut builder1 = Builder::new(make);
                        let _apd = builder1.append_dir_all(&output_name, path::Path::new(&the_name_of_the_file)).errh(Some("Couldn't build the arcive".to_string())).ughf()?;
                        let _finsh = builder1.into_inner().errh(Some("Couldn't build the arcive".to_string()))._success_res("Tar completed", "loaded successfully").ughf()?;
                    }

                    else {
                        let mut builder2 = Builder::new(make);
                        let _ap = builder2.append_file(&output_name, &mut open).errh(Some("Couldn`t build the arcive".to_string())).ughf()?;
                        let _finsh = builder2.into_inner().errh(Some("Couldn't build the arcive".to_string()))._success_res("Tar completed", "loaded successfully").ughf()?;
                    }
                }
                "--Unload" => {
                    let open = fs::File::open(the_name_of_the_file).errh(Some("NotFound".to_string())).ughf()?;

                    let mut arc = Archive::new(open);
                    arc.unpack(output_name).errh(Some("Couldn't build the arcive".to_string()))._success_res("Tar completed", "Unloaded successfully").ughf()?;
                }   
                _ => {
                    println!("[{path:?}]~>{}: due to [{}]" , "Error".red().bold() , "No Flag was supplied".red().bold());
                }
        }
        Ok(())
}

pub struct ZipArg<'z> {
    pub n1:&'z str,
    pub n2:&'z str,
    pub n3:&'z str,
    pub f1:&'z str,
    pub f2:&'z str,
    pub f3:&'z str
}

pub struct ZipDir<'z> {
    pub src_dir:&'z str,
    pub res_dir:&'z str,
}

pub fn zip(flags:&str ,file_name:&str  , ziparg:ZipArg , zipdir:ZipDir) -> std::result::Result<() , HyperkitError> {
    let tell = tell();
    let args = ziparg;

    match flags.trim() {
        "--New-Zip" => {
            let curr = env::current_dir().errh(Some("Couldn't extarct the path".to_string())).ughf()?;
            let creat = std::fs::File::create(format!("{}/{}" , &curr.to_string_lossy().to_string() , &file_name)).errh(Some(curr.to_string_lossy().to_string())).ughf()?;

            let mut zip = zip::ZipWriter::new(creat);
            zip.add_directory("zip/", SimpleFileOptions::default()).errh(None)?;

            let config = SimpleFileOptions::default()
            .compression_method(zip::CompressionMethod::Stored)
            .unix_permissions(0o755);
            
            if args.n1 == "-N1" {
                zip.start_file(args.f1 , config).errh(Some(args.f1.to_string())).ughf()?;
            }
            if args.n2 == "-N2" {
                zip.start_file(args.f2 , config).errh(Some(args.f2.to_string())).ughf()?;
            }
            if args.n3 == "-N3" {
                zip.start_file(args.f3 , config).errh(Some(args.f3.to_string())).ughf()?;
            }

            if args.n1 == "-E1" {   
                let mut open1 = fs::File::open(args.f1).errh(Some(args.f1.to_string())).ughf()?;
                let mut read = Vec::new();
                open1.read_to_end(&mut read).errh(None).ughf()?;

                zip.start_file(args.f1, config).errh(Some(args.f1.to_string())).ughf()?;
                zip.write_all(&mut read).errh(None).ughf()?;
            }
            if args.n2 == "-E2" {
                let mut open2 = fs::File::open(args.f2).errh(Some(args.f2.to_string())).ughf()?;
                let mut read = Vec::new();
                open2.read_to_end(&mut read).errh(None).ughf()?;

                zip.start_file(args.f2, config).errh(Some(args.f2.to_string())).ughf()?;
                zip.write_all(&mut read).errh(None).ughf()?;
            }
            if args.n3 == "-E3" {
                let mut open3 = fs::File::open(args.f3).errh(Some(args.f3.to_string())).ughf()?;
                let mut read = Vec::new();
                open3.read_to_end(&mut read).errh(None).ughf()?;

                zip.start_file(args.f3, config).errh(Some(args.f3.to_string())).ughf()?;
                zip.write_all(&mut read).errh(None).ughf()?;
            }
            zip.finish().errh(None).ughf()._success_res("Zip", "Done")?;
        }
        "--Zip-All" => {
            let creat = fs::File::create(zipdir.res_dir).errh(Some(zipdir.res_dir.to_string())).ughf()?;
            let mut zipdirr = ZipWriter::new(creat);

            let config = SimpleFileOptions::default()
            .compression_method(zip::CompressionMethod::Stored)
            .unix_permissions(0o755);
            
            let src_dir = path::Path::new(zipdir.src_dir);

            for i in WalkDir::new(src_dir).into_iter().filter_map(|e| e.ok()) {
                let path = i.path();
                let name = path.strip_prefix(src_dir).ok().ok_or(HyperkitError::ArchiveErr(crate::backend::safe::ArchiveErr::StripPrefixErr(None)))
                .ughf()?
                .to_str().extract();
                
                if name.is_empty() {
                    continue;
                }

                if path.is_file() {
                    println!("[{tell:?}]~>{}: {}" , "Adding file".bright_yellow().bold() , name.bright_cyan().bold());
                    zipdirr.start_file(name, config).errh(None).ughf()?;
                    let mut open = fs::File::open(path).errh(None).ughf()?;
                    io::copy(&mut open, &mut zipdirr).errh(None).ughf()?;
                }
                else if path.is_dir() {
                    println!("[{tell:?}]~>{}: {}", "Adding directory".bright_yellow().bold() , name.bright_cyan().bold());
                    zipdirr.add_directory(name, config).errh(Some(name.to_string())).ughf()?;
                }
            }
            zipdirr.finish().errh(Some(zipdir.src_dir.to_string())).ughf()._success_res("Zip file created successfully", zipdir.res_dir)?;
        },
        "--extract" => {
            let path = path::Path::new(zipdir.src_dir);
            let outp = path::Path::new(zipdir.res_dir);

            let open = fs::File::open(path).errh(Some(zipdir.src_dir.to_string())).ughf()?;
            let mut arc = ZipArchive::new(open).errh(Some(zipdir.src_dir.to_string())).ughf()?; 

            for i in 0..arc.len() {
                let mut file = arc.by_index(i).errh(None).ughf()?;
                let out = match file.enclosed_name() {
                    Some(o) => outp.join(o),
                    None => {
                        continue;
                    }
                };

                if file.name().ends_with('/') {
                    fs::create_dir_all(&out).errh(Some(out.to_string_lossy().to_string())).ughf()._success_res("Zip", "extracted!")?;
                }
                else {
                    if let Some(o) = out.parent() {
                        if !o.exists() {
                            fs::create_dir_all(o).errh(Some(o.to_string_lossy().to_string())).ughf()._success_res("Zip", "extracted!")?;
                        }
                    }
                    let mut outf = fs::File::create(&out).errh(Some(out.to_string_lossy().to_string())).ughf()?;
                    io::copy(&mut file, &mut outf).errh(Some(format!("{}|{}" , zipdir.src_dir , zipdir.res_dir).to_string())).ughv();
                }
                #[cfg(unix)] 
                {
                    use std::os::unix::fs::PermissionsExt;
                        if let Some(mode) = file.unix_mode() {
                            fs::set_permissions(&out, fs::Permissions::from_mode(mode)).errh(Some(mode.to_string())).ughf()?;
                        }
                    }
                }
            }
        _ => {
            println!("[{tell:?}]~>{}: due to [{}]" , "Error".red().bold() , "No Flag was supplied".red().bold());
        }
    }
    Ok(())
}

pub struct FileZipArg<'s> {
    pub f1:&'s str,
    pub f2:&'s str,
    pub f3:&'s str,
}

pub fn cli_zip(op:&str ,files: FileZipArg , file_name:&str , src_dir:&str , res_dir:&str) -> std::result::Result<() , HyperkitError> {
    let tell = tell();
    match op.trim() {
        "New-Zip" => {
                let curr = env::current_dir().errh(Some("Couldn't extarct the path".to_string())).ughf()?;
                let creat = std::fs::File::create(format!("{}/{}" , &curr.to_string_lossy().to_string() , &file_name)).errh(Some(curr.to_string_lossy().to_string())).ughf()?;

                let mut zip = zip::ZipWriter::new(creat);
                zip.add_directory("zip/", SimpleFileOptions::default()).errh(None)?;

                let config = SimpleFileOptions::default()
                .compression_method(zip::CompressionMethod::Stored)
                .unix_permissions(0o755);

                if files.f1.len() > 0 {
                    let open1 = fs::File::open(files.f1).errh(Some(files.f1.to_string()));
                    if let Ok(mut o) = open1 {
                        let mut read = Vec::new();
                        o.read_to_end(&mut read).errh(None).ughf()?;

                        zip.start_file(files.f1 , config).errh(Some(files.f1.to_string())).ughf()?;
                        zip.write_all(&mut read).errh(None).ughf()?    
                    } else {
                        zip.start_file(files.f1 , config).errh(Some(files.f1.to_string())).ughf()?;
                    }
                }
                if files.f2.len() > 0 {
                    let open2  = fs::File::open(files.f2).errh(Some(files.f2.to_string()));
                    if let Ok(mut o) = open2 {
                        let mut read = Vec::new();
                        o.read_to_end(&mut read).errh(None).ughf()?;

                        zip.start_file(files.f2 , config).errh(Some(files.f1.to_string())).ughf()?;
                        zip.write_all(&mut read).errh(None).ughf()?
                    } else {
                        zip.start_file(files.f2 , config).errh(Some(files.f2.to_string())).ughf()?;
                    }
                }
                if files.f3.len() > 0 {
                    let open3  = fs::File::open(files.f3).errh(Some(files.f3.to_string()));
                    if let Ok(mut o) = open3 {
                        let mut read = Vec::new();
                        o.read_to_end(&mut read).errh(None).ughf()?;

                        zip.start_file(files.f3 , config).errh(Some(files.f1.to_string())).ughf()?;
                        zip.write_all(&mut read).errh(None).ughf()?
                    } else {
                        zip.start_file(files.f3 , config).errh(Some(files.f3.to_string())).ughf()?;
                    }
                }
            }
            "Zip-All" => {
                let creat = fs::File::create(res_dir).errh(Some(res_dir.to_string())).ughf()?;
                let mut zipdirr = ZipWriter::new(creat);

                let config = SimpleFileOptions::default()
                .compression_method(zip::CompressionMethod::Stored)
                .unix_permissions(0o755);
            
                let src_dir = path::Path::new(src_dir);

                for i in WalkDir::new(src_dir).into_iter().filter_map(|e| e.ok()) {
                    let path = i.path();
                    let name = path.strip_prefix(src_dir).ok().ok_or(HyperkitError::ArchiveErr(crate::backend::safe::ArchiveErr::StripPrefixErr(None)))
                    .ughf()?
                    .to_str().extract();
                
                    if name.is_empty() {
                        continue;
                    }
                    if path.is_file() {
                        println!("[{tell:?}]~>{}: {}" , "Adding file".bright_yellow().bold() , name.bright_cyan().bold());
                        zipdirr.start_file(name, config).errh(None).ughf()?;
                        let mut open = fs::File::open(path).errh(None).ughf()?;
                        io::copy(&mut open, &mut zipdirr).errh(None).ughf()?;
                    }
                    else if path.is_dir() {
                        println!("[{tell:?}]~>{}: {}", "Adding directory".bright_yellow().bold() , name.bright_cyan().bold());
                        zipdirr.add_directory(name, config).errh(Some(name.to_string())).ughf()?;
                    }
            }
                zipdirr.finish().errh(Some(src_dir.to_string_lossy().to_string())).ughf()._success_res("Zip file created successfully", res_dir)?;
            }
            "extract" => {
                let path = path::Path::new(src_dir);
                let outp = path::Path::new(res_dir);

                let open = fs::File::open(path).errh(Some(src_dir.to_string())).ughf()?;
                let mut arc = ZipArchive::new(open).errh(Some(src_dir.to_string())).ughf()?; 

                for i in 0..arc.len() {
                    let mut file = arc.by_index(i).errh(None).ughf()?;
                    let out = match file.enclosed_name() {
                        Some(o) => outp.join(o),
                        None => {
                            continue;
                        }
                    };

                    if file.name().ends_with('/') {
                        fs::create_dir_all(&out).errh(Some(out.to_string_lossy().to_string())).ughf()._success_res("Zip", "extracted!")?;
                    }
                    else {
                        if let Some(o) = out.parent() {
                            if !o.exists() {
                                fs::create_dir_all(o).errh(Some(o.to_string_lossy().to_string())).ughf()._success_res("Zip", "extracted!")?;
                            }
                        }
                        let mut outf = fs::File::create(&out).errh(Some(out.to_string_lossy().to_string())).ughf()?;
                        io::copy(&mut file, &mut outf).errh(Some(format!("{}|{}" , src_dir , res_dir).to_string())).ughv();
                    }
                    #[cfg(unix)] 
                    {
                        use std::os::unix::fs::PermissionsExt;
                            if let Some(mode) = file.unix_mode() {
                                fs::set_permissions(&out, fs::Permissions::from_mode(mode)).errh(Some(mode.to_string())).ughf()?;
                            }
                        }
                    }
                }
            _ => todo!()
        }
    Ok(())
}


pub fn transmute (ttype:&str, flag:&str , the_name_of_the_file:&str , output_name:&str) -> std::result::Result<() , HyperkitError> {
    let path = tell();
    let readed = read_file_cont( the_name_of_the_file)?;
    match ttype.trim() {
        "base64-ST" => {
            match flag.trim() {
                "--enc" => {
                    let enc = BASE64_STANDARD.encode(&readed);

                    fs::write(&output_name, enc).errh(Some("couldn`t write the encoded codic to the file Consider trying abother type".to_string()))._success_res("transmute" , "encoded successfully").ughf()?;
                }
                "--dec" => {
                    let dec = BASE64_STANDARD.decode(&readed.trim()).unwrap_or_default();

                    fs::write(&output_name, dec).errh(Some("couldn`t write the decoded codic to the file Consider trying abother type".to_string()))._success_res("transmute" , "decoded successfully").ughf()?;
                }
                _ => {
                    println!("[{path:?}]~>{}: due to [{}]" , "Error".red().bold() , "No Flag was supplied".red().bold());
                }
            }
        }
        "base64-PD" => {
            match flag.trim() {
                "--enc" => {
                    let enc = BASE64_STANDARD_NO_PAD.encode(&readed);

                    fs::write(&output_name, enc).errh(Some("couldn`t write the encoded codic to the file Consider trying abother type".to_string()))._success_res("transmute" , "encoded successfully").ughf()?;
                }
                "--dec" => {
                    let dec = BASE64_STANDARD_NO_PAD.decode(&readed).unwrap_or_default();

                    fs::write(&output_name, dec).errh(Some("couldn`t write the decoded codic to the file Consider trying abother type".to_string()))._success_res("transmute" , "decoded successfully").ughf()?
                }
                _ => {
                    println!("[{path:?}]~>{}: due to [{}]" , "Error".red().bold() , "No Flag was supplied".red().bold());
                }
            }
        }
        "base64-URL" => {
            match flag.trim() {
                "--enc" => {
                    let enc = BASE64_URL_SAFE.encode(&readed);

                    fs::write(&output_name, enc).errh(Some("couldn`t write the encoded codic to the file Consider trying abother type".to_string()))._success_res("transmute" , "encoded successfully").ughf()?;
                }
                "--dec" => {
                    let dec = BASE64_URL_SAFE.decode(&readed).unwrap_or_default();

                    fs::write(&output_name, dec).errh(Some("couldn`t write the decoded codic to the file Consider trying abother type".to_string()))._success_res("transmute" , "decoded successfully").ughf()?;
                }
                _ => {
                    println!("[{path:?}]~>{}: due to [{}]" , "Error".red().bold() , "No Flag was supplied".red().bold());
                }
            }            
        }
        "hex" => {
            match flag.trim() {
                "--enc" => {
                    let enc = hex::encode(&readed);

                    fs::write(&output_name, enc).errh(Some("couldn`t write the encoded codic to the file Consider trying abother type".to_string()))._success_res("transmute" , "encoded successfully").ughf()?;
                }
                "--dec" => {
                    let dec = hex::decode(&readed).unwrap_or_default();

                    fs::write(&output_name, dec).errh(Some("couldn`t write the decoded codic to the file Consider trying abother type".to_string()))._success_res("transmute" , "decoded successfully").ughf()?;
                }
                _ => {
                    println!("[{path:?}]~>{}: due to [{}]" , "Error".red().bold() , "No Flag was supplied".red().bold());
                }
            }
        }
        "HEX" => {
            match flag.trim() {
                "--enc" => {
                    let enc = hex::encode_upper(&readed);

                    fs::write(&output_name, enc).errh(Some("couldn`t write the encoded codic to the file Consider trying abother type".to_string()))._success_res("transmute" , "encoded successfully").ughf()?;
                }
                "--dec" => {
                    let dec = hex::decode(&readed).unwrap_or_default();

                    fs::write(&output_name, dec).errh(Some("couldn`t write the decoded codic to the file Consider trying abother type".to_string()))._success_res("transmute" , "decoded successfully").ughf()?;
                }
                _ => {
                    println!("[{path:?}]~>{}: due to [{}]" , "Error".red().bold() , "No Flag was supplied".red().bold());
                }
            }
        }
        "url" => {
            match flag.trim() {
                "--enc" => {
                    let enc = urlencoding::encode(&readed).into_owned();

                    fs::write(&output_name, enc).errh(Some("couldn`t write the encoded codic to the file Consider trying abother type".to_string()))._success_res("transmute" , "encoded successfully").ughf()?;
                }
                "--dec" => {
                    let dec = urlencoding::decode(&readed).unwrap_or_default().into_owned();
            
                    fs::write(&output_name, dec).errh(Some("couldn`t write the decoded codic to the file Consider trying abother type".to_string()))._success_res("transmute" , "decoded successfully").ughf()?;
                }
                _ => {
                    println!("[{path:?}]~>{}: due to [{}]" , "Error".red().bold() , "No Flag was supplied".red().bold());
                }
            }
        }
        _ => {
            println!("[{path:?}]~>{}: due to [{}]" , "Error".red().bold() , "No type was supplied".red().bold());
        }
    }
    Ok(())
}

fn help_tree<P: AsRef<std::path::Path>>(path: P) -> String {
    let p = path.as_ref().file_name().extract().to_str().extract().bright_green().bold().to_string();
    p
}

pub fn treee<P: AsRef<std::path::Path> >(path: P) -> std::result::Result<Tree<String> , HyperkitError> {
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
