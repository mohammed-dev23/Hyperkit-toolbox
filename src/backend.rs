pub mod commands {
    use colored::Colorize;
    use sysinfo::System;
    pub const _GITHUBLINK: &str = "https://github.com/mohamemd-v1/Shell-like-toolbox-.git";
    use crate::{
        backend::{
            safe::{ErrH, HyperkitError, Success, Ugh, Ughv},
            standard::{input, tell},
        },
        toml::toml,
    };
    use nix::{
        sys::{self, signal::*},
        unistd::Pid,
        unistd::{gethostname, sethostname},
    };
    use std::{
        env::{self},
        fs::{self, File},
        io::*,
        path::PathBuf,
        process,
    };
    pub fn help(helpt: String) {
        match helpt.trim() {
            "--commands" => {
                println!(
                    "   *{} {} to end the session",
                    "enter".green(),
                    "end".bright_blue()
                );
                println!(
                    "   *{} {} to clear the screen",
                    "enter".green(),
                    "clean".bright_blue()
                );
                println!(
                    "   *{} {} {} to change the dir",
                    "enter".green(),
                    "go".bright_blue(),
                    "<Dir>".bright_purple()
                );
                println!(
                    "   *{} {} to see the current dir",
                    "enter".green(),
                    "wh".bright_blue()
                );
                println!(
                    "   *{} {} to show all the files in the current dir",
                    "enter".green(),
                    "see".bright_blue()
                );
                println!(
                    "   *{} {} {} to see what is inside the file",
                    "enter".green(),
                    "peek ".bright_blue(),
                    "<Path>".bright_purple()
                );
                println!(
                    "   *{} {} {} to delete anything",
                    "enter".green(),
                    "burn".bright_blue(),
                    "<Path/File>".bright_purple()
                );
                println!(
                    "   *{} {} {} {} to copy a file",
                    "enter".green(),
                    "clone".bright_blue(),
                    "<Name/File>".bright_purple(),
                    "<Nname/File>".bright_yellow()
                );
                println!(
                    "   *{} {} {} to create a file",
                    "enter".green(),
                    "forge".bright_blue(),
                    "<Name>".bright_purple()
                );
                println!(
                    "   *{} {} {} to make a dir",
                    "enter".green(),
                    "mk".bright_blue(),
                    "<Name>".bright_purple()
                );
                println!(
                    "   *{} {} {} to run a program",
                    "enter".green(),
                    "run".bright_blue(),
                    "<App>".bright_purple()
                );
                println!(
                    "   *{} {} {} to move a file from place to another",
                    "enter".green(),
                    "mv".bright_blue(),
                    "<Name>".bright_purple()
                );
                println!(
                    "   *{} {} {} to find the dir of a file",
                    "enter".green(),
                    "find".bright_blue(),
                    "<FileName>".bright_purple()
                );
                println!(
                    "   *{} {} {} to list and lookup processes",
                    "enter".green(),
                    "ps".bright_blue(),
                    "<Flag[-SF: search for pros , -A: list all the pros]>".bright_purple()
                );
                println!("   *{} {} {} to stop processes |{}|" ,  "enter".green() , "stop".bright_blue() , "<PID>".bright_purple() , "#Warning do not even attempt to enter latters only numbers is allowed otherwise it will stop itself!!".bright_red().bold());
                println!(
                    "   *{} {} {} {} to see and edit the hostname",
                    "enter".green(),
                    "hostname".bright_blue(),
                    "<Flags: --show/--set>".bright_purple(),
                    "<--set: <NewHostName>>".bright_yellow()
                );
                println!(
                    "   *{} {} {} {} {} to config the config file",
                    "enter".green(),
                    "Configer".bright_blue(),
                    "<Flags:username/hispath>".bright_purple(),
                    "<operation:--set>".bright_yellow(),
                    "<Value>".bright_cyan()
                );
                println!("   *{} {} {} {} to manage the history file" , "enter".green() , "history".bright_blue(), "<Flag:--clean(to clean the history)/--search(to find a keyword in the history file)>".bright_purple() , "<--search <Value>>".bright_yellow())
            }
            "--built-in-apps" => {
                println!(
                    "   *{} {} {} to use the built-in calculator",
                    "enter".green(),
                    "calc".bright_blue(),
                    "<Math>".purple()
                );
                println!(
                    "   *{} {} {} {} {} to make/extract tar files",
                    "enter".green(),
                    "tar".bright_blue(),
                    "<Flag>".bright_purple(),
                    "<File-Name>".bright_cyan(),
                    "<File-Outpot-Name>".bright_magenta()
                );
                println!(
                    "   *{} {} {} {} {} {} to encode/decode files",
                    "enter".green(),
                    "transmute".bright_blue(),
                    "<Type>".bright_purple(),
                    "<Flag>".bright_yellow(),
                    "<File-Name>".bright_cyan(),
                    "<File-Outpot-Name>".bright_magenta()
                );
                println!(
                    "   *{} {} {} {} {} {} {} {} {} to make/extract zip files",
                    "enter".green(),
                    "zip".bright_blue(),
                    "<Name>".bright_purple(),
                    "<Flag>".bright_yellow(),
                    "<Name/Path>".bright_cyan(),
                    "<Flag2>".bright_yellow(),
                    "<Name/Path2>".bright_cyan(),
                    "<Flag3>".bright_yellow(),
                    "<Name/Path3>".bright_cyan()
                );
                println!(
                    "   *{} {} {} to list a dir in tree format",
                    "enter".green(),
                    "tree".bright_blue(),
                    "<Path| default value = .>".bright_purple()
                );
                println!(
                    "   *{} {} {} to get informations about your system ",
                    "enter".green(),
                    "yank".bright_blue(),
                    "<info:<mem / temp / cpu / bios / os / battery / disk>>".purple()
                );
                println!(
                    "   *{} {} {} to get informations a file in your system ",
                    "enter".green(),
                    "indicate".bright_blue(),
                    "<File>".purple()
                );
            }
            "--about" => {
                println!(
                    "{}HyperKit is a modern, extensible, and lightweight command-line environment built to unify the tools you need into one powerful workspace.",
                    "@".bright_green()
                )
            }
            "transmute" => {
                println!(
                    "   *[{}: base64-PD<pedding> , base64-ST<standerd> , base64-URL<url> , hex<low-case hex> , Hex<uper-case hex> ][{}: --enc {} --dec {}]",
                    "Types".bright_green().bold(),
                    "flags".bright_blue().bold(),
                    "to encode a file".bright_purple().bold(),
                    "to decode a file".bright_yellow().bold()
                );
            }
            "zip" => {
                println!(
                    "   *[{}: -N:{} , -E:{} , --Zip-All:{} , --extract:{}]",
                    "Flags".bright_blue().bold(),
                    "New File".bright_yellow().bold(),
                    "Exsited File".bright_yellow().bold(),
                    "turn the whole folder ot dir into zip"
                        .bright_yellow()
                        .bold(),
                    "to extract a zip file".bright_yellow().bold()
                );
            }
            _ => {
                println!(
                    "   *{} {} {} to see all the commands , {} to list all the available built in apps , {} for about",
                    "Enter".green(),
                    "help".red(),
                    "--commands".bright_purple(),
                    "--built-in-apps".bright_purple(),
                    "--about".bright_purple()
                );
            }
        }
    }

    pub fn clean() -> std::result::Result<(), HyperkitError> {
        print!("\x1B[2J\x1B[1;1H");
        stdout().flush().errh(None)?;
        Ok(())
    }

    pub fn go(t: &str) -> std::result::Result<(), HyperkitError> {
        let path = PathBuf::from(&t);

        env::set_current_dir(&path)
            .errh(Some(t.to_string()))
            ._success_res("Go", "directory has been changed successfully")
            .ughf()?;
        Ok(())
    }

    pub fn wh() -> std::result::Result<(), HyperkitError> {
        let path = tell();
        let username = toml().customization.username;

        let wh = env::current_dir().errh(None)?;
        println!(
            "[{path:?}][{username}]~>{}\x1b[34m{}\x1b[0m",
            "~".bright_green(),
            wh.display()
        );
        Ok(())
    }

    pub fn see() -> std::result::Result<(), HyperkitError> {
        let path = tell();

        let cur = env::current_dir().errh(None)?;
        let dir = fs::read_dir(cur);

        match dir {
            Ok(w) => {
                for i in w {
                    let dir = match i {
                        Ok(t) => t,
                        Err(e) => {
                            println!("[{path:?}]~>Error: due to {e:?}");
                            return Ok(());
                        }
                    };
                    println!(
                        "   {}\x1B[94m{}\x1b[0m",
                        "~".bright_green(),
                        dir.file_name().to_string_lossy()
                    );
                }
            }
            Err(error) => {
                println!(
                    "[{path:?}]~>{}: due to \x1b[33m{error:?}\x1b[0m",
                    "Error".red()
                );
                return Ok(());
            }
        }
        Ok(())
    }

    pub fn peek(file: &str) -> std::result::Result<(), HyperkitError> {
        let path = tell();
        let fe = File::open(&file);
        let username = toml().customization.username;

        if let Err(e) = &fe {
            if e.kind() == ErrorKind::NotFound {
                println!(
                    "[{path:?}][{username}]~>{}: couldn't open the file due to [{}]",
                    "Error".red().bold(),
                    "NotFound error".red().bold()
                );
                println!("[{path:?}][{username}]~>Do you want to make this file?");
                print!("[{path:?}][{username}]~>({}/{}):", "Y".green(), "N".red());
                stdout().flush().errh(None)?;

                let yesorno = input()?;

                if yesorno == "Y" {
                    fs::File::create(&file).errh(Some(file.to_string()))?;
                }
            }
        };

        let fe = &mut fe.errh(None)?;
        let mut r = String::new();

        let _read = fe.read_to_string(&mut r).errh(Some(file.to_string()));

        println!("\x1b[34m{}\x1b[0m", r);
        Ok(())
    }

    pub fn mk(path: &str) -> std::result::Result<(), HyperkitError> {
        fs::create_dir(&path)
            .errh(Some(path.to_string()))
            ._success_res("Mk", "Directory created successfully")
            .ughv();
        Ok(())
    }

    pub fn burn(path: &str) -> std::result::Result<(), HyperkitError> {
        let tell = tell();
        let username = toml().customization.username;

        let burn = fs::remove_file(&path);

        if burn.is_ok() == true {
            println!(
                "[{tell:?}][{username}]~>{}: [{}]",
                "burn".bright_green().bold(),
                "file has been burned successfully".bright_green().bold()
            );
        }

        if let Err(e) = burn {
            match e.kind() {
                ErrorKind::IsADirectory => {
                    let burn_dir = fs::remove_dir(&path);

                    if let Err(e) = burn_dir {
                        if e.kind() == ErrorKind::DirectoryNotEmpty {
                            print!(
                                "[{tell:?}][{username}]~>[{}/{}]: the Directory is Not Empty do you stil want to delete it? >> ",
                                "Y".bold().green(),
                                "N".bold().red()
                            );
                            stdout().flush().errh(None)?;

                            let yesorno = input()?;
                            if yesorno == "Y" {
                                fs::remove_dir_all(&path)
                                    .errh(Some(path.to_string()))
                                    ._success_res("burn", "Directory has been burned successfully")
                                    .ughf()?;
                            }
                        }
                    }
                }
                _ => {}
            }
        }
        Ok(())
    }

    pub fn rn(f: &str, t: &str) -> std::result::Result<(), HyperkitError> {
        let tt = format!("{} or {}", &f, &t);

        fs::rename(&f, &t)
            .errh(Some(tt.to_string()))
            ._success_res("rn", "Renamed successfully")
            .ughf()?;
        Ok(())
    }

    pub fn clone(f: &String, t: &String) -> std::result::Result<(), HyperkitError> {
        fs::copy(&f, &t)
            .errh(Some(t.to_string()))
            ._success_res("clone", "Copied!")
            .ughf()?;
        Ok(())
    }

    pub fn forge(file: &String) -> std::result::Result<(), HyperkitError> {
        fs::File::create(&file)
            .errh(Some(file.to_string()))
            ._success_res("Forge completed!", "File created")
            .ughf()?;
        Ok(())
    }

    pub fn run(app: &str) -> std::result::Result<(), HyperkitError> {
        let path = tell();
        let username = toml().customization.username;
        let run = process::Command::new(&app)
            .output()
            .errh(Some(app.to_string()))
            .ughf()?;
        println!(
            "[{path:?}][{username}]~>\x1b[34m{}\x1b[0m",
            String::from_utf8_lossy(&run.stdout)
        );
        Ok(())
    }

    pub fn mv(name: &str, path: &str) -> std::result::Result<(), HyperkitError> {
        let format = format!("{}/{}", &name, &path);

        fs::copy(&name, &format).errh(Some(format.to_string()))?;

        let delete_eveadnice = fs::remove_file(&name);

        if let Err(e) = delete_eveadnice {
            match e.kind() {
                ErrorKind::IsADirectory => {
                    fs::remove_dir_all(&name)
                        .errh(Some(format.to_string()))
                        ._success_res("mv", "moving completed")
                        .ughf()?;
                }
                _ => {}
            }
        }

        Ok(())
    }

    pub fn find(file_path: &str) -> std::result::Result<(), HyperkitError> {
        use walkdir::*;
        let username = toml().customization.username;
        let tell = tell();
        let mut err = false;

        let find = WalkDir::new("/").into_iter().filter_map(|e| e.ok());

        for i in find {
            if i.file_name() == file_path {
                println!(
                    "[{tell:?}][{username}]~> [{}] {}: \x1b[33m{}\x1b[0m",
                    "find".bright_green().bold(),
                    "found at".bright_green().bold(),
                    i.path().display()
                );
                err = true;
            }
        }

        if err == false {
            println!(
                "[{tell:?}]~> [{}] {}: \x1b[31m{}\x1b[0m",
                "find".bright_green().bold(),
                "Couldn`t find it anywhere".bright_red().bold(),
                &file_path
            );
        }

        Ok(())
    }

    pub fn ps(_flag: &str, _pid: usize) -> std::result::Result<(), HyperkitError> {
        use sysinfo::Pid;

        let username = toml().customization.username;
        let tell = tell();
        let mut sys = System::new_all();
        sys.refresh_all();
        match _flag {
            "-SF" => {
                if _flag == "-SF" {
                    if let Some(p) = sys.process(Pid::from(_pid)) {
                        println!(
                            "[{tell:?}][{username}]~>[{}] \x1B[1m\x1B[36m{}\x1B[0m\x1B[0m | {}:\x1B[1m\x1B[32m{}\x1B[0m\x1B[0m Gib | \x1B[1m\x1B[36m{}\x1B[0m\x1B[0m:\x1B[1m\x1B[32m{}\x1B[0m\x1B[0m Gib",
                            "ps".bright_green().bold(),
                            p.name().display(),
                            "Disk usage".bright_yellow().bold(),
                            p.disk_usage().total_written_bytes as f64 / f64::from(1024).powi(3),
                            "memory usage".bright_yellow().bold(),
                            p.memory() as f64 / f64::from(1024).powi(3)
                        );
                    }
                    if let None = sys.process(Pid::from(_pid)) {
                        println!(
                            "[{tell:?}]~>[{}] {}: process not found or not running \x1b[1m\x1b[31m<{}>\x1b[0m\x1b[0m",
                            "ps".bright_green().bold(),
                            "Error".bright_red().bold(),
                            _pid
                        )
                    }
                }
            }
            "-A" => {
                for (pid, pros) in sys.processes() {
                    println!(
                        "[\x1B[1m\x1B[32m{pid}\x1B[0m\x1B[0m]~>\x1B[1m\x1B[36m{}\x1B[0m\x1B[0m",
                        pros.name().display()
                    )
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

    pub fn stop(pid: i32) {
        let pid = Pid::from_raw(pid);

        let _kill = sys::signal::kill(pid, SIGKILL)
            .errh(Some(pid.to_string()))
            ._success_res("stop", "the target has been stoped!")
            .ughv();
    }

    pub fn hostname(flag: &str, newhostanme: &str) -> std::result::Result<(), HyperkitError> {
        let tell = tell();
        let username = toml().customization.username;

        if flag == "--show" {
            let hostname = gethostname()
                .errh(None)
                .ughv()
                .to_string_lossy()
                .to_string();
            println!(
                "[{tell:?}][{username}]~>{}: [{}]  ",
                "HostName".bright_green().bold(),
                hostname.bright_green().bold()
            );
        }

        if flag == "--set" {
            sethostname(newhostanme)
                .errh(Some(newhostanme.to_string()))
                ._success_res("HostName succeeded: ", newhostanme)
                .ughv();
        }

        Ok(())
    }

    pub fn history(flag: &str, search: &str) -> std::result::Result<(), HyperkitError> {
        let path = toml().dependencies.historyfilepath;
        let path = path.as_str();
        let tell = tell();

        match flag {
            "--search" => {
                let mut readed = String::new();
                let mut open = fs::File::open(path).errh(Some(path.to_string())).ughf()?;
                open.read_to_string(&mut readed).errh(None).ughv();

                let t: Vec<&str> = readed.lines().collect();

                let mut num = 0;

                for i in t {
                    if i.contains(search) {
                        println!(
                            "   [{}]: {}-{}",
                            "Found at line".bright_purple().bold(),
                            num.to_string().bright_cyan().bold(),
                            i.bright_green().bold()
                        );
                    } else {
                        num += 1
                    }
                }
            }
            _ => println!(
                "[{tell:?}]~>{}: due to [{}]",
                "Error".red().bold(),
                "No flag was supplied".red().bold()
            ),
        }

        Ok(())
    }
}

pub mod standard {
    use colored::Colorize;
    use std::{env::*, io::stdin, path::PathBuf};

    use crate::backend::safe::{ErrH, HyperkitError};

    pub fn input() -> std::result::Result<String, HyperkitError> {
        let mut input = String::new();
        stdin().read_line(&mut input).errh(None)?;
        let input = input.trim().to_string();

        Ok(input)
    }

    pub fn tell() -> PathBuf {
        let path = match current_dir() {
            Ok(o) => o,
            Err(e) => {
                eprintln!("([Error]~>{}: due to {}", "Error".red(), e);
                PathBuf::new()
            }
        };
        path
    }
}

pub mod parser {
    use crate::backend::safe::HyperkitError;

    pub trait Checker {
        type Out;

        fn checker(self, res: Option<String>) -> Self::Out
        where
            Self: Sized;
    }

    impl Checker for String {
        type Out = std::result::Result<Self, HyperkitError>;
        fn checker(self, res: Option<String>) -> Self::Out
        where
            Self: Sized,
        {
            if self == "" {
                return Err(HyperkitError::MissingParameter(res));
            } else {
                return Ok(self);
            }
        }
    }

    pub fn parser(input: &str) -> Vec<&str> {
        let parse: Vec<&str> = input.split_whitespace().collect();
        return parse;
    }
    pub fn token(data: &Vec<&str>, index: usize) -> String {
        let token = data.get(index).map(|e| e.to_string());

        if let Some(t) = token {
            return t;
        } else {
            return token.unwrap_or_default();
        }
    }
}

pub mod safe {
    use crate::{
        backend::{
            clean::{ExtractOptions, ExtractOptionsErr},
            standard::tell,
        },
        repl::GITHUBLINK,
        toml::toml,
    };
    use argon2::Error;
    use colored::Colorize;
    use core::fmt;
    use nix::errno::Errno;
    use rustyline::error::ReadlineError;
    use std::num::{IntErrorKind, ParseIntError};
    use zip::result::ZipError;

    pub type _Result<'h, T> = std::result::Result<T, HyperkitError>;

    #[derive(Debug)]
    pub enum HyperkitError {
        ParsingErr(ParsingErr),
        InputReadingErr(InputReadingErr),
        FileError(FileError),
        ArchiveErr(ArchiveErr),
        MissingParameter(Option<String>),
        SystemErr(SystemErr),
        CryptographyErr(CryptographyErr),
        ShouldNotHappen,
    }

    #[derive(Debug)]
    pub enum FileError {
        FileNotFound(Option<String>),
        FileTooLarge(Option<String>),
        PermissionDenied(Option<String>),
        ReadOnlyFile(Option<String>),
        IsADirectory(Option<String>),
        NotADirectory(Option<String>),
        UnsupportedFileType(Option<String>),
        InvalidFilename(Option<String>),
    }

    #[derive(Debug)]
    pub enum ParsingErr {
        NotNumber(Option<String>),
        ZeroOrEmputy(Option<String>),
        OverFlow(Option<String>),
        InvalidDigit(Option<String>),
        ExtractingErr(Option<String>),
    }

    #[derive(Debug)]
    pub enum InputReadingErr {
        Interrupted(Option<String>),
        BadEncoding(Option<String>),
        StreamClosed(Option<String>),
        PipeBroken(Option<String>),
        Blocked(Option<String>),
        OutOfMemory(Option<String>),
        Unknown(Option<String>),
    }

    #[derive(Debug)]
    pub enum ArchiveErr {
        PassWordNotVaild(Option<String>),
        FileNotFound(Option<String>),
        UnsupportedArc(Option<String>),
        InvalidArchive(Option<String>),
        StripPrefixErr(Option<String>),
    }

    #[derive(Debug)]
    pub enum SystemErr {
        Permissiondenied(Option<String>),
        Invalidargument(Option<String>),
        ResourceTempUnavailable(Option<String>),
        OperationNpermitted(Option<String>),
        OutOfMemory(Option<String>),
        ResourceBusy(Option<String>),
        InterruptedCall(Option<String>),
        DiskFull(Option<String>),
        BadAddress(Option<String>),
        NameTooLong(Option<String>),
        UnknownSysErr(Option<String>),
    }

    #[derive(Debug)]
    pub enum CryptographyErr {
        UnsupportedFormat(Option<String>),
        AssociatedDataTooLong(Option<String>),
        InvalidAlgorithm(Option<String>),
        KeyIdTooLong(Option<String>),
        MemoryCostTooLow(Option<String>),
        MemoryCostTooHigh(Option<String>),
        OutputTooShort(Option<String>),
        OutputTooLong(Option<String>),
        PasswordTooLong(Option<String>),
        SaltTooShort(Option<String>),
        SaltTooLong(Option<String>),
        SecretTooLong(Option<String>),
        ThreadCountTooLow(Option<String>),
        ThreadCountTooHigh(Option<String>),
        TimeCostTooLow(Option<String>),
        InvalidVersion(Option<String>),
        DecryptionFailed(Option<String>),
        UnknownCryptoErr,
    }

    impl fmt::Display for HyperkitError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                HyperkitError::ShouldNotHappen => write!(
                    f,
                    "{}: due to [{}: <{}>]",
                    "Error".bright_red().bold(),
                    "".bright_red(),
                    GITHUBLINK
                ),

                HyperkitError::ParsingErr(e) => match e {
                    ParsingErr::NotNumber(err_res) => write!(
                        f,
                        "{}: due to [{}: <{}>]",
                        "Error".bright_red().bold(),
                        "Expected a number, but no input was provided".bright_red(),
                        err_res.extract().bright_yellow().bold()
                    ),
                    ParsingErr::InvalidDigit(err_res) => write!(
                        f,
                        "{}: due to [{}: <{}>]",
                        "Error".bright_red().bold(),
                        "Invalid digits".bright_red(),
                        err_res.extract().bright_yellow().bold()
                    ),
                    ParsingErr::OverFlow(err_res) => write!(
                        f,
                        "{}: due to [{}: <{}>]",
                        "Error".bright_red().bold(),
                        "Number is out of range".bright_red(),
                        err_res.extract().bright_yellow().bold()
                    ),
                    ParsingErr::ZeroOrEmputy(err_res) => write!(
                        f,
                        "{}: due to [{}: <{}>]",
                        "Error".bright_red().bold(),
                        "Zero is not a valid value".bright_red(),
                        err_res.extract().bright_yellow().bold()
                    ),
                    ParsingErr::ExtractingErr(err_res) => write!(
                        f,
                        "{}: due to [{}: <{}>]",
                        "Error".bright_red().bold(),
                        "extracting faild".bright_red(),
                        err_res.extract().bright_yellow().bold()
                    ),
                },

                HyperkitError::FileError(e) => match e {
                    FileError::FileNotFound(err_res) => write!(
                        f,
                        "{}: due to [{}: <{}>]",
                        "Error".bright_red().bold(),
                        "the requested resource was not found".bright_red(),
                        err_res.extract().bright_yellow().bold()
                    ),
                    FileError::FileTooLarge(err_res) => write!(
                        f,
                        "{}: due to [{}: <{}>]",
                        "Error".bright_red().bold(),
                        "The file is to large".bright_red(),
                        err_res.extract().bright_yellow().bold()
                    ),
                    FileError::InvalidFilename(err_res) => write!(
                        f,
                        "{}: due to [{}: <{}>]",
                        "Error".bright_red().bold(),
                        "Invalid file name".bright_red(),
                        err_res.extract().bright_yellow().bold()
                    ),
                    FileError::IsADirectory(err_res) => write!(
                        f,
                        "{}: due to [{}: <{}>]",
                        "Error".bright_red().bold(),
                        "is a directory".bright_red(),
                        err_res.extract().bright_yellow().bold()
                    ),
                    FileError::NotADirectory(err_res) => write!(
                        f,
                        "{}: due to [{}: <{}>]",
                        "Error".bright_red().bold(),
                        "is not a directory".bright_red(),
                        err_res.extract().bright_yellow().bold()
                    ),
                    FileError::PermissionDenied(err_res) => write!(
                        f,
                        "{}: due to [{}: <{}>]",
                        "Error".bright_red().bold(),
                        "Permission denied".bright_red(),
                        err_res.extract().bright_yellow().bold()
                    ),
                    FileError::ReadOnlyFile(err_res) => write!(
                        f,
                        "{}: due to [{}: <{}>]",
                        "Error".bright_red().bold(),
                        "Read only file".bright_red(),
                        err_res.extract().bright_yellow().bold()
                    ),
                    FileError::UnsupportedFileType(err_res) => write!(
                        f,
                        "{}: due to [{}: <{}>]",
                        "Error".bright_red().bold(),
                        "Unsupported file type".bright_red(),
                        err_res.extract().bright_yellow().bold()
                    ),
                },

                HyperkitError::InputReadingErr(e) => match e {
                    InputReadingErr::BadEncoding(err_res) => write!(
                        f,
                        "{}: due to [{}: <{}>]",
                        "Error".bright_red().bold(),
                        "Invalid input encoding".bright_red(),
                        err_res.extract().bright_yellow().bold()
                    ),
                    InputReadingErr::Blocked(err_res) => write!(
                        f,
                        "{}: due to [{}: <{}>]",
                        "Error".bright_red().bold(),
                        "Input operation would block".bright_red(),
                        err_res.extract().bright_yellow().bold()
                    ),
                    InputReadingErr::Interrupted(err_res) => write!(
                        f,
                        "{}: due to [{}: <{}>]",
                        "Error".bright_red().bold(),
                        "Input operation was interrupted".bright_red(),
                        err_res.extract().bright_yellow().bold()
                    ),
                    InputReadingErr::OutOfMemory(err_res) => write!(
                        f,
                        "{}: due to [{}: <{}>]",
                        "Error".bright_red().bold(),
                        "Out of memory while reading input".bright_red(),
                        err_res.extract().bright_yellow().bold()
                    ),
                    InputReadingErr::PipeBroken(err_res) => write!(
                        f,
                        "{}: due to [{}: <{}>]",
                        "Error".bright_red().bold(),
                        "Broken pipe while reading input".bright_red(),
                        err_res.extract().bright_yellow().bold()
                    ),
                    InputReadingErr::StreamClosed(err_res) => write!(
                        f,
                        "{}: due to [{}: <{}>]",
                        "Error".bright_red().bold(),
                        "Input stream closed unexpectedly".bright_red(),
                        err_res.extract().bright_yellow().bold()
                    ),
                    InputReadingErr::Unknown(err_res) => write!(
                        f,
                        "{}: due to [{}: <{}>]",
                        "Error".bright_red().bold(),
                        "Unknown input error".bright_red(),
                        err_res.extract().bright_yellow().bold()
                    ),
                },

                HyperkitError::ArchiveErr(e) => match e {
                    ArchiveErr::FileNotFound(err_res) => write!(
                        f,
                        "{}: due to [{}: <{}>]",
                        "Error".bright_red().bold(),
                        "File not found".bright_red(),
                        err_res.extract().bright_yellow().bold()
                    ),
                    ArchiveErr::InvalidArchive(err_res) => write!(
                        f,
                        "{}: due to [{}: <{}>]",
                        "Error".bright_red().bold(),
                        "invalid Zip archive".bright_red(),
                        err_res.extract().bright_yellow().bold()
                    ),
                    ArchiveErr::PassWordNotVaild(err_res) => write!(
                        f,
                        "{}: due to [{}: <{}>]",
                        "Error".bright_red().bold(),
                        "provided password is incorrect".bright_red(),
                        err_res.extract().bright_yellow().bold()
                    ),
                    ArchiveErr::UnsupportedArc(err_res) => write!(
                        f,
                        "{}: due to [{}: <{}>]",
                        "Error".bright_red().bold(),
                        "unsupported Zip archive".bright_red(),
                        err_res.extract().bright_yellow().bold()
                    ),
                    ArchiveErr::StripPrefixErr(err_res) => write!(
                        f,
                        "{}: due to [{}: <{}>]",
                        "Error".bright_red().bold(),
                        "prefix was not found".bright_red(),
                        err_res.extract().bright_yellow().bold()
                    ),
                },

                HyperkitError::MissingParameter(err_res) => write!(
                    f,
                    "{}: due to [{}: <{}>]",
                    "Error".bright_red().bold(),
                    "missing required parameter".bright_red(),
                    err_res.extract().bright_yellow().bold()
                ),

                HyperkitError::SystemErr(e) => match e {
                    SystemErr::BadAddress(err_res) => write!(
                        f,
                        "{}: due to [{}: <{}>]",
                        "Error".bright_red().bold(),
                        "Bad memory address".bright_red(),
                        err_res.extract().bright_yellow().bold()
                    ),
                    SystemErr::DiskFull(err_res) => write!(
                        f,
                        "{}: due to [{}: <{}>]",
                        "Error".bright_red().bold(),
                        "No space left on device".bright_red(),
                        err_res.extract().bright_yellow().bold()
                    ),
                    SystemErr::InterruptedCall(err_res) => write!(
                        f,
                        "{}: due to [{}: <{}>]",
                        "Error".bright_red().bold(),
                        "System call was interrupted".bright_red(),
                        err_res.extract().bright_yellow().bold()
                    ),
                    SystemErr::Invalidargument(err_res) => write!(
                        f,
                        "{}: due to [{}: <{}>]",
                        "Error".bright_red().bold(),
                        "Invalid argument provided".bright_red(),
                        err_res.extract().bright_yellow().bold()
                    ),
                    SystemErr::NameTooLong(err_res) => write!(
                        f,
                        "{}: due to [{}: <{}>]",
                        "Error".bright_red().bold(),
                        "Name exceeds maximum length".bright_red(),
                        err_res.extract().bright_yellow().bold()
                    ),
                    SystemErr::OperationNpermitted(err_res) => write!(
                        f,
                        "{}: due to [{}: <{}>]",
                        "Error".bright_red().bold(),
                        "Operation not permitted".bright_red(),
                        err_res.extract().bright_yellow().bold()
                    ),
                    SystemErr::OutOfMemory(err_res) => write!(
                        f,
                        "{}: due to [{}: <{}>]",
                        "Error".bright_red().bold(),
                        "Out of memory".bright_red(),
                        err_res.extract().bright_yellow().bold()
                    ),
                    SystemErr::Permissiondenied(err_res) => write!(
                        f,
                        "{}: due to [{}: <{}>]",
                        "Error".bright_red().bold(),
                        "Permission denied".bright_red(),
                        err_res.extract().bright_yellow().bold()
                    ),
                    SystemErr::ResourceBusy(err_res) => write!(
                        f,
                        "{}: due to [{}: <{}>]",
                        "Error".bright_red().bold(),
                        "Resource is busy".bright_red(),
                        err_res.extract().bright_yellow().bold()
                    ),
                    SystemErr::ResourceTempUnavailable(err_res) => write!(
                        f,
                        "{}: due to [{}: <{}>]",
                        "Error".bright_red().bold(),
                        "Resource temporarily unavailable".bright_red(),
                        err_res.extract().bright_yellow().bold()
                    ),
                    SystemErr::UnknownSysErr(err_res) => write!(
                        f,
                        "{}: due to [{}: <{}>]",
                        "Error".bright_red().bold(),
                        "Unknown system error".bright_red(),
                        err_res.extract().bright_yellow().bold()
                    ),
                },
                HyperkitError::CryptographyErr(e) => match e {
                    CryptographyErr::UnsupportedFormat(err_res) => write!(
                        f,
                        "{}: due to [{}: <{}>]",
                        "Error".bright_red().bold(),
                        "Unsupported Format".bright_red(),
                        err_res.extract().bright_yellow().bold()
                    ),
                    CryptographyErr::AssociatedDataTooLong(err_res) => write!(
                        f,
                        "{}: due to [{}: <{}>]",
                        "Error".bright_red().bold(),
                        "Associated data exceeds the maximum of 2³²−1 bytes.".bright_red(),
                        err_res.extract().bright_yellow().bold()
                    ),
                    CryptographyErr::InvalidAlgorithm(err_res) => write!(
                        f,
                        "{}: due to [{}: <{}>]",
                        "Error".bright_red().bold(),
                        "the hash string doesn't match the expected variant".bright_red(),
                        err_res.extract().bright_yellow().bold()
                    ),
                    CryptographyErr::InvalidVersion(err_res) => write!(
                        f,
                        "{}: due to [{}: <{}>]",
                        "Error".bright_red().bold(),
                        "Version number in the hash string is not a recognised".bright_red(),
                        err_res.extract().bright_yellow().bold()
                    ),
                    CryptographyErr::KeyIdTooLong(err_res) => write!(
                        f,
                        "{}: due to [{}: <{}>]",
                        "Error".bright_red().bold(),
                        "The key id is too long".bright_red(),
                        err_res.extract().bright_yellow().bold()
                    ),
                    CryptographyErr::MemoryCostTooHigh(err_res) => write!(
                        f,
                        "{}: due to [{}: <{}>]",
                        "Error".bright_red().bold(),
                        "Memory cost is too high".bright_red(),
                        err_res.extract().bright_yellow().bold()
                    ),
                    CryptographyErr::MemoryCostTooLow(err_res) => write!(
                        f,
                        "{}: due to [{}: <{}>]",
                        "Error".bright_red().bold(),
                        "Memory cost is too low".bright_red(),
                        err_res.extract().bright_yellow().bold()
                    ),
                    CryptographyErr::OutputTooLong(err_res) => write!(
                        f,
                        "{}: due to [{}: <{}>]",
                        "Error".bright_red().bold(),
                        "Hash output exceeds the maximum".bright_red(),
                        err_res.extract().bright_yellow().bold()
                    ),
                    CryptographyErr::OutputTooShort(err_res) => write!(
                        f,
                        "{}: due to [{}: <{}>]",
                        "Error".bright_red().bold(),
                        "Hash output is below the minimum".bright_red(),
                        err_res.extract().bright_yellow().bold()
                    ),
                    CryptographyErr::PasswordTooLong(err_res) => write!(
                        f,
                        "{}: due to [{}: <{}>]",
                        "Error".bright_red().bold(),
                        "Password is too long".bright_red(),
                        err_res.extract().bright_yellow().bold()
                    ),
                    CryptographyErr::SaltTooLong(err_res) => write!(
                        f,
                        "{}: due to [{}: <{}>]",
                        "Error".bright_red().bold(),
                        "Salt exceeds the maximum".bright_red(),
                        err_res.extract().bright_yellow().bold()
                    ),
                    CryptographyErr::SaltTooShort(err_res) => write!(
                        f,
                        "{}: due to [{}: <{}>]",
                        "Error".bright_red().bold(),
                        "Salt is shorter than the minimum".bright_red(),
                        err_res.extract().bright_yellow().bold()
                    ),
                    CryptographyErr::SecretTooLong(err_res) => write!(
                        f,
                        "{}: due to [{}: <{}>]",
                        "Error".bright_red().bold(),
                        "Secret key exceeds the maximum".bright_red(),
                        err_res.extract().bright_yellow().bold()
                    ),
                    CryptographyErr::ThreadCountTooHigh(err_res) => write!(
                        f,
                        "{}: due to [{}: <{}>]",
                        "Error".bright_red().bold(),
                        "Thread count is too high".bright_red(),
                        err_res.extract().bright_yellow().bold()
                    ),
                    CryptographyErr::ThreadCountTooLow(err_res) => write!(
                        f,
                        "{}: due to [{}: <{}>]",
                        "Error".bright_red().bold(),
                        "Thread count is too low".bright_red(),
                        err_res.extract().bright_yellow().bold()
                    ),
                    CryptographyErr::TimeCostTooLow(err_res) => write!(
                        f,
                        "{}: due to [{}: <{}>]",
                        "Error".bright_red().bold(),
                        "Time cost is too low".bright_red(),
                        err_res.extract().bright_yellow().bold()
                    ),
                    CryptographyErr::DecryptionFailed(err_res) => write!(
                        f,
                        "{}: due to [{}: <{}>]",
                        "Error".bright_red().bold(),
                        "Decryption Failed".bright_red(),
                        err_res.extract().bright_yellow().bold()
                    ),
                    CryptographyErr::UnknownCryptoErr => write!(
                        f,
                        "{}: due to [{}]",
                        "Error".bright_red().bold(),
                        "Unknown crypto err".bright_red()
                    ),
                },
            }
        }
    }

    pub trait Success<T> {
        type Out;

        fn _success(self, mas1: &str, mas2: &str);
        fn _success_res(self, mas1: &str, mas2: &str) -> Self::Out
        where
            Self: Sized;
    }

    pub trait ErrH {
        type Out;

        fn errh(self, res: Option<String>) -> Self::Out
        where
            Self: Sized;
    }

    pub trait Ugh {
        type Out;

        fn ugh(&self);
        fn ughf(self) -> Self
        where
            Self: Sized;
    }

    pub trait Ughv {
        type Out;

        fn ughv(self) -> Self::Out
        where
            Self: Sized;
    }

    impl<T> Success<T> for std::result::Result<T, HyperkitError> {
        type Out = std::result::Result<T, HyperkitError>;

        fn _success(self, mas1: &str, mas2: &str) {
            let path = tell();
            let username = toml().customization.username;
            match self {
                Ok(_) => {
                    println!(
                        "[{path:?}][{username}]~>{}: [{}]",
                        mas1.bright_green().bold(),
                        mas2.bright_green().bold()
                    );
                    return;
                }
                Err(_) => {
                    return;
                }
            }
        }

        fn _success_res(self, mas1: &str, mas2: &str) -> Self::Out
        where
            Self: Sized,
        {
            let path = tell();
            let username = toml().customization.username;

            match self {
                Ok(o) => {
                    println!(
                        "[{path:?}][{username}]~>{}: [{}]",
                        mas1.bright_green().bold(),
                        mas2.bright_green().bold()
                    );
                    return Ok(o);
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }
    }

    impl<T> ErrH for std::io::Result<T> {
        type Out = std::result::Result<T, HyperkitError>;

        fn errh(self, res: Option<String>) -> Self::Out
        where
            Self: Sized,
        {
            match self {
                Ok(o) => return Ok(o),
                Err(e) => {
                    let hypere = match e.kind() {
                        std::io::ErrorKind::NotFound => {
                            HyperkitError::FileError(FileError::FileNotFound(res))
                        }
                        std::io::ErrorKind::FileTooLarge => {
                            HyperkitError::FileError(FileError::FileTooLarge(res))
                        }
                        std::io::ErrorKind::NotADirectory => {
                            HyperkitError::FileError(FileError::NotADirectory(res))
                        }
                        std::io::ErrorKind::IsADirectory => {
                            HyperkitError::FileError(FileError::IsADirectory(res))
                        }
                        std::io::ErrorKind::InvalidFilename => {
                            HyperkitError::FileError(FileError::InvalidFilename(res))
                        }
                        std::io::ErrorKind::PermissionDenied => {
                            HyperkitError::FileError(FileError::PermissionDenied(res))
                        }
                        std::io::ErrorKind::ReadOnlyFilesystem => {
                            HyperkitError::FileError(FileError::ReadOnlyFile(res))
                        }
                        std::io::ErrorKind::Unsupported => {
                            HyperkitError::FileError(FileError::UnsupportedFileType(res))
                        }

                        std::io::ErrorKind::Interrupted => {
                            HyperkitError::InputReadingErr(InputReadingErr::Interrupted(res))
                        }
                        std::io::ErrorKind::InvalidData => {
                            HyperkitError::InputReadingErr(InputReadingErr::BadEncoding(res))
                        }
                        std::io::ErrorKind::BrokenPipe => {
                            HyperkitError::InputReadingErr(InputReadingErr::PipeBroken(res))
                        }
                        std::io::ErrorKind::UnexpectedEof => {
                            HyperkitError::InputReadingErr(InputReadingErr::StreamClosed(res))
                        }
                        std::io::ErrorKind::OutOfMemory => {
                            HyperkitError::InputReadingErr(InputReadingErr::OutOfMemory(res))
                        }
                        std::io::ErrorKind::Other => {
                            HyperkitError::InputReadingErr(InputReadingErr::Unknown(res))
                        }
                        std::io::ErrorKind::WouldBlock => {
                            HyperkitError::InputReadingErr(InputReadingErr::Blocked(res))
                        }

                        _ => HyperkitError::ShouldNotHappen,
                    };
                    return Err(hypere);
                }
            }
        }
    }

    impl<T> ErrH for core::result::Result<T, ParseIntError> {
        type Out = std::result::Result<T, HyperkitError>;

        fn errh(self, res: Option<String>) -> Self::Out
        where
            Self: Sized,
        {
            match self {
                Ok(o) => return Ok(o),
                Err(e) => {
                    let hypere = match e.kind() {
                        IntErrorKind::Empty => {
                            HyperkitError::ParsingErr(ParsingErr::NotNumber(res))
                        }
                        IntErrorKind::InvalidDigit => {
                            HyperkitError::ParsingErr(ParsingErr::InvalidDigit(res))
                        }
                        IntErrorKind::NegOverflow => {
                            HyperkitError::ParsingErr(ParsingErr::OverFlow(res))
                        }
                        IntErrorKind::PosOverflow => {
                            HyperkitError::ParsingErr(ParsingErr::OverFlow(res))
                        }
                        IntErrorKind::Zero => {
                            HyperkitError::ParsingErr(ParsingErr::ZeroOrEmputy(res))
                        }

                        _ => HyperkitError::ShouldNotHappen,
                    };

                    return Err(hypere);
                }
            }
        }
    }

    impl<T> ErrH for zip::result::ZipResult<T> {
        type Out = std::result::Result<T, HyperkitError>;

        fn errh(self, res: Option<String>) -> Self::Out
        where
            Self: Sized,
        {
            match self {
                Ok(o) => Ok(o),
                Err(e) => {
                    let hypere = match e {
                        ZipError::Io(s) => match s.kind() {
                            std::io::ErrorKind::NotFound => {
                                HyperkitError::FileError(FileError::FileNotFound(res))
                            }
                            std::io::ErrorKind::FileTooLarge => {
                                HyperkitError::FileError(FileError::FileTooLarge(res))
                            }
                            std::io::ErrorKind::NotADirectory => {
                                HyperkitError::FileError(FileError::NotADirectory(res))
                            }
                            std::io::ErrorKind::IsADirectory => {
                                HyperkitError::FileError(FileError::IsADirectory(res))
                            }
                            std::io::ErrorKind::InvalidFilename => {
                                HyperkitError::FileError(FileError::InvalidFilename(res))
                            }
                            std::io::ErrorKind::PermissionDenied => {
                                HyperkitError::FileError(FileError::PermissionDenied(res))
                            }
                            std::io::ErrorKind::ReadOnlyFilesystem => {
                                HyperkitError::FileError(FileError::ReadOnlyFile(res))
                            }
                            std::io::ErrorKind::Unsupported => {
                                HyperkitError::FileError(FileError::UnsupportedFileType(res))
                            }

                            _ => HyperkitError::ShouldNotHappen,
                        },
                        ZipError::InvalidPassword => {
                            HyperkitError::ArchiveErr(ArchiveErr::PassWordNotVaild(res))
                        }
                        ZipError::FileNotFound => {
                            HyperkitError::ArchiveErr(ArchiveErr::FileNotFound(res))
                        }
                        ZipError::UnsupportedArchive(_) => {
                            HyperkitError::ArchiveErr(ArchiveErr::UnsupportedArc(res))
                        }
                        ZipError::InvalidArchive(_) => {
                            HyperkitError::ArchiveErr(ArchiveErr::InvalidArchive(res))
                        }

                        _ => HyperkitError::ShouldNotHappen,
                    };
                    return Err(hypere);
                }
            }
        }
    }

    impl<T> ErrH for std::result::Result<T, Errno> {
        type Out = std::result::Result<T, HyperkitError>;

        fn errh(self, res: Option<String>) -> Self::Out
        where
            Self: Sized,
        {
            match self {
                Ok(o) => Ok(o),
                Err(e) => {
                    let hypererr = match e {
                        Errno::EACCES => HyperkitError::SystemErr(SystemErr::Permissiondenied(res)),
                        Errno::EINVAL => HyperkitError::SystemErr(SystemErr::Invalidargument(res)),
                        Errno::EAGAIN => {
                            HyperkitError::SystemErr(SystemErr::ResourceTempUnavailable(res))
                        }
                        Errno::EPERM => {
                            HyperkitError::SystemErr(SystemErr::OperationNpermitted(res))
                        }
                        Errno::ENOMEM => HyperkitError::SystemErr(SystemErr::OutOfMemory(res)),
                        Errno::EBUSY => HyperkitError::SystemErr(SystemErr::ResourceBusy(res)),
                        Errno::EINTR => HyperkitError::SystemErr(SystemErr::InterruptedCall(res)),
                        Errno::ENOSPC => HyperkitError::SystemErr(SystemErr::DiskFull(res)),
                        Errno::EFAULT => HyperkitError::SystemErr(SystemErr::BadAddress(res)),
                        Errno::ENAMETOOLONG => {
                            HyperkitError::SystemErr(SystemErr::NameTooLong(res))
                        }
                        Errno::UnknownErrno => {
                            HyperkitError::SystemErr(SystemErr::UnknownSysErr(res))
                        }

                        _ => HyperkitError::ShouldNotHappen,
                    };
                    return Err(hypererr);
                }
            }
        }
    }

    impl<T> ErrH for std::result::Result<T, Option<std::io::Error>> {
        type Out = std::result::Result<T, HyperkitError>;

        fn errh(self, res: Option<String>) -> Self::Out
        where
            Self: Sized,
        {
            match self {
                Ok(o) => return Ok(o),
                Err(e) => {
                    let hypere = match e.extract_err(Some("backend/675".to_string()))?.kind() {
                        std::io::ErrorKind::NotFound => {
                            HyperkitError::FileError(FileError::FileNotFound(res))
                        }
                        std::io::ErrorKind::FileTooLarge => {
                            HyperkitError::FileError(FileError::FileTooLarge(res))
                        }
                        std::io::ErrorKind::NotADirectory => {
                            HyperkitError::FileError(FileError::NotADirectory(res))
                        }
                        std::io::ErrorKind::IsADirectory => {
                            HyperkitError::FileError(FileError::IsADirectory(res))
                        }
                        std::io::ErrorKind::InvalidFilename => {
                            HyperkitError::FileError(FileError::InvalidFilename(res))
                        }
                        std::io::ErrorKind::PermissionDenied => {
                            HyperkitError::FileError(FileError::PermissionDenied(res))
                        }
                        std::io::ErrorKind::ReadOnlyFilesystem => {
                            HyperkitError::FileError(FileError::ReadOnlyFile(res))
                        }
                        std::io::ErrorKind::Unsupported => {
                            HyperkitError::FileError(FileError::UnsupportedFileType(res))
                        }

                        std::io::ErrorKind::Interrupted => {
                            HyperkitError::InputReadingErr(InputReadingErr::Interrupted(res))
                        }
                        std::io::ErrorKind::InvalidData => {
                            HyperkitError::InputReadingErr(InputReadingErr::BadEncoding(res))
                        }
                        std::io::ErrorKind::BrokenPipe => {
                            HyperkitError::InputReadingErr(InputReadingErr::PipeBroken(res))
                        }
                        std::io::ErrorKind::UnexpectedEof => {
                            HyperkitError::InputReadingErr(InputReadingErr::StreamClosed(res))
                        }
                        std::io::ErrorKind::OutOfMemory => {
                            HyperkitError::InputReadingErr(InputReadingErr::OutOfMemory(res))
                        }
                        std::io::ErrorKind::Other => {
                            HyperkitError::InputReadingErr(InputReadingErr::Unknown(res))
                        }
                        std::io::ErrorKind::WouldBlock => {
                            HyperkitError::InputReadingErr(InputReadingErr::Blocked(res))
                        }

                        _ => HyperkitError::ShouldNotHappen,
                    };
                    return Err(hypere);
                }
            }
        }
    }

    impl<T> ErrH for std::result::Result<T, ReadlineError> {
        type Out = std::result::Result<T, HyperkitError>;

        fn errh(self, res: Option<String>) -> Self::Out
        where
            Self: Sized,
        {
            match self {
                Ok(o) => Ok(o),
                Err(e) => {
                    let hypererr = match e {
                        ReadlineError::Io(e) => match e.kind() {
                            std::io::ErrorKind::NotFound => {
                                HyperkitError::FileError(FileError::FileNotFound(res))
                            }
                            std::io::ErrorKind::FileTooLarge => {
                                HyperkitError::FileError(FileError::FileTooLarge(res))
                            }
                            std::io::ErrorKind::NotADirectory => {
                                HyperkitError::FileError(FileError::NotADirectory(res))
                            }
                            std::io::ErrorKind::IsADirectory => {
                                HyperkitError::FileError(FileError::IsADirectory(res))
                            }
                            std::io::ErrorKind::InvalidFilename => {
                                HyperkitError::FileError(FileError::InvalidFilename(res))
                            }
                            std::io::ErrorKind::PermissionDenied => {
                                HyperkitError::FileError(FileError::PermissionDenied(res))
                            }
                            std::io::ErrorKind::ReadOnlyFilesystem => {
                                HyperkitError::FileError(FileError::ReadOnlyFile(res))
                            }
                            std::io::ErrorKind::Unsupported => {
                                HyperkitError::FileError(FileError::UnsupportedFileType(res))
                            }

                            std::io::ErrorKind::Interrupted => {
                                HyperkitError::InputReadingErr(InputReadingErr::Interrupted(res))
                            }
                            std::io::ErrorKind::InvalidData => {
                                HyperkitError::InputReadingErr(InputReadingErr::BadEncoding(res))
                            }
                            std::io::ErrorKind::BrokenPipe => {
                                HyperkitError::InputReadingErr(InputReadingErr::PipeBroken(res))
                            }
                            std::io::ErrorKind::UnexpectedEof => {
                                HyperkitError::InputReadingErr(InputReadingErr::StreamClosed(res))
                            }
                            std::io::ErrorKind::OutOfMemory => {
                                HyperkitError::InputReadingErr(InputReadingErr::OutOfMemory(res))
                            }
                            std::io::ErrorKind::Other => {
                                HyperkitError::InputReadingErr(InputReadingErr::Unknown(res))
                            }
                            std::io::ErrorKind::WouldBlock => {
                                HyperkitError::InputReadingErr(InputReadingErr::Blocked(res))
                            }

                            _ => HyperkitError::ShouldNotHappen,
                        },
                        ReadlineError::Errno(e) => match e {
                            Errno::EACCES => {
                                HyperkitError::SystemErr(SystemErr::Permissiondenied(res))
                            }
                            Errno::EINVAL => {
                                HyperkitError::SystemErr(SystemErr::Invalidargument(res))
                            }
                            Errno::EAGAIN => {
                                HyperkitError::SystemErr(SystemErr::ResourceTempUnavailable(res))
                            }
                            Errno::EPERM => {
                                HyperkitError::SystemErr(SystemErr::OperationNpermitted(res))
                            }
                            Errno::ENOMEM => HyperkitError::SystemErr(SystemErr::OutOfMemory(res)),
                            Errno::EBUSY => HyperkitError::SystemErr(SystemErr::ResourceBusy(res)),
                            Errno::EINTR => {
                                HyperkitError::SystemErr(SystemErr::InterruptedCall(res))
                            }
                            Errno::ENOSPC => HyperkitError::SystemErr(SystemErr::DiskFull(res)),
                            Errno::EFAULT => HyperkitError::SystemErr(SystemErr::BadAddress(res)),
                            Errno::ENAMETOOLONG => {
                                HyperkitError::SystemErr(SystemErr::NameTooLong(res))
                            }
                            Errno::UnknownErrno => {
                                HyperkitError::SystemErr(SystemErr::UnknownSysErr(res))
                            }

                            _ => HyperkitError::ShouldNotHappen,
                        },
                        _ => HyperkitError::ShouldNotHappen,
                    };
                    return Err(hypererr);
                }
            }
        }
    }

    impl<T> ErrH for core::result::Result<T, Error> {
        type Out = std::result::Result<T, HyperkitError>;

        fn errh(self, res: Option<String>) -> Self::Out
        where
            Self: Sized,
        {
            match self {
                Ok(o) => Ok(o),
                Err(e) => {
                    let hypererr = match e {
                        Error::AdTooLong => HyperkitError::CryptographyErr(
                            CryptographyErr::AssociatedDataTooLong(res),
                        ),
                        Error::AlgorithmInvalid => {
                            HyperkitError::CryptographyErr(CryptographyErr::InvalidAlgorithm(res))
                        }
                        Error::MemoryTooLittle => {
                            HyperkitError::CryptographyErr(CryptographyErr::MemoryCostTooLow(res))
                        }
                        Error::MemoryTooMuch => {
                            HyperkitError::CryptographyErr(CryptographyErr::MemoryCostTooHigh(res))
                        }
                        Error::PwdTooLong => {
                            HyperkitError::CryptographyErr(CryptographyErr::PasswordTooLong(res))
                        }
                        Error::SaltTooLong => {
                            HyperkitError::CryptographyErr(CryptographyErr::SaltTooLong(res))
                        }
                        Error::SaltTooShort => {
                            HyperkitError::CryptographyErr(CryptographyErr::SaltTooShort(res))
                        }
                        Error::OutputTooLong => {
                            HyperkitError::CryptographyErr(CryptographyErr::OutputTooLong(res))
                        }
                        Error::OutputTooShort => {
                            HyperkitError::CryptographyErr(CryptographyErr::OutputTooShort(res))
                        }
                        Error::ThreadsTooFew => {
                            HyperkitError::CryptographyErr(CryptographyErr::ThreadCountTooLow(res))
                        }
                        Error::ThreadsTooMany => {
                            HyperkitError::CryptographyErr(CryptographyErr::ThreadCountTooHigh(res))
                        }
                        Error::SecretTooLong => {
                            HyperkitError::CryptographyErr(CryptographyErr::SecretTooLong(res))
                        }
                        Error::KeyIdTooLong => {
                            HyperkitError::CryptographyErr(CryptographyErr::KeyIdTooLong(res))
                        }
                        Error::TimeTooSmall => {
                            HyperkitError::CryptographyErr(CryptographyErr::TimeCostTooLow(res))
                        }
                        Error::VersionInvalid => {
                            HyperkitError::CryptographyErr(CryptographyErr::InvalidVersion(res))
                        }

                        _ => HyperkitError::CryptographyErr(CryptographyErr::UnknownCryptoErr),
                    };
                    return Err(hypererr);
                }
            }
        }
    }

    impl<T> Ugh for std::result::Result<T, HyperkitError> {
        type Out = std::result::Result<T, HyperkitError>;

        fn ugh(&self) {
            let path = tell();

            if let Err(e) = self {
                eprintln!("[{path:?}]~>{e}");
            }
        }
        fn ughf(self) -> Self
        where
            Self::Out: Sized,
        {
            let path = tell();

            match self {
                Ok(o) => return Ok(o),
                Err(e) => {
                    eprintln!("[{path:?}]~>{e}");
                    return Err(e);
                }
            }
        }
    }

    impl<T: Default> Ughv for std::result::Result<T, HyperkitError> {
        type Out = T;

        fn ughv(self) -> Self::Out
        where
            Self: Sized,
        {
            let path = tell();

            match self {
                Ok(o) => o,
                Err(e) => {
                    eprintln!("[{path:?}]~>{e}");
                    T::default()
                }
            }
        }
    }
}

pub mod clean {
    use colored::Colorize;
    use std::{fs::File, io::Read};

    use crate::backend::safe::HyperkitError;

    pub trait ExtractOptions {
        type Out;

        fn extract(&self) -> Self::Out
        where
            Self: Sized;
    }

    pub trait ExtractOptionsErr {
        type Out;

        fn extract_err(self, res: Option<String>) -> Self::Out
        where
            Self: Sized;
    }

    impl<T: Default + Clone> ExtractOptions for Option<T> {
        type Out = T;

        fn extract(&self) -> Self::Out
        where
            Self: Sized,
        {
            if let Some(o) = self {
                return o.clone();
            } else {
                eprintln!(
                    "[{}]~> due to {}",
                    "Error".bright_red().bold(),
                    "extracting faild".bright_red().bold()
                );
                return T::default();
            }
        }
    }

    impl<E> ExtractOptionsErr for Option<E> {
        type Out = std::result::Result<E, HyperkitError>;

        fn extract_err(self, res: Option<String>) -> Self::Out
        where
            Self: Sized,
        {
            if let Some(o) = self {
                Ok(o)
            } else {
                return Err(HyperkitError::ParsingErr(
                    super::safe::ParsingErr::ExtractingErr(res),
                ));
            }
        }
    }

    pub fn read_file_cont(path: &str) -> std::result::Result<String, HyperkitError> {
        use super::safe::*;

        let mut txtf = File::open(&path).errh(Some(path.to_string()))?;

        let mut readed = String::new();
        txtf.read_to_string(&mut readed)
            .errh(Some(path.to_string()))?;
        return Ok(readed);
    }
}
