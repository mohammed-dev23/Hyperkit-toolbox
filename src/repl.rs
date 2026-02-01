use crate::apps::{self, ZipArg, ZipDir, calc, tar, time, transmute, zip};
use crate::backend::commands::hostname;
use crate::backend::safe::{ ErrH, HyperkitError, Ugh, Ughv};
use crate::backend::{commands, standard::tell, parser::* };
use crate::toml::{self, toml};
use std::{env::* , borrow::Cow::{self, Owned}};
use colored::*;
use rustyline::{Completer, Hinter, Validator , error::ReadlineError, completion::FilenameCompleter , highlight::{CmdKind, Highlighter, MatchingBracketHighlighter}, hint::HistoryHinter, 
    validate::MatchingBracketValidator , Cmd , CompletionType , Config , EditMode, Editor , KeyEvent , Helper 
    };

#[derive(Helper , Completer , Hinter , Validator)]
pub struct Enveditor {
        #[rustyline(Completer)]
        comp:FilenameCompleter,
        hig:MatchingBracketHighlighter,
        #[rustyline(Validator)]
        val:MatchingBracketValidator,
        #[rustyline(Hinter)]
        hin:HistoryHinter,
    }

impl Highlighter for Enveditor {
        fn highlight_hint<'h>(&self, hint: &'h str) -> Cow<'h, str> {
            Owned("\x1b[1m".to_owned() + hint + "\x1b[0m")
        }
        fn highlight<'l>(&self, line: &'l str, pos: usize) -> Cow<'l, str> {
            self.hig.highlight(line, pos)
        }
        fn highlight_char(&self, line: &str, pos: usize, kind: CmdKind) -> bool {
            self.hig.highlight_char(line, pos, kind)
        }
    }

pub const GITHUBLINK:&str = "https://github.com/mohamemd-v1/Shell-like-toolbox-.git";

pub fn repl() -> std::result::Result<() , HyperkitError> {
        println!("*{} {} {} to see all the commands , {} to list all the available built in apps , {} for about" , "Enter".green()  , "help".red() ,"--commands".bright_purple() , "--built-in-apps".bright_purple() , "--about".bright_purple() );
        let home = match home_dir() {
            Some(h) => h,
            None => {
                println!(">{}: home directory does not exist?" , "Error".red());
                return Ok(());
            }
        };

        let _ = set_current_dir(home).errh(None);

        let configdef = Config::builder()
        .history_ignore_space(true)
        .completion_type(CompletionType::List)
        .edit_mode(EditMode::Emacs)
        .build();

        let enveditor = Enveditor {
            hig:MatchingBracketHighlighter::new(),
            comp:FilenameCompleter::new(),
            val:MatchingBracketValidator::new(),
            hin:HistoryHinter::new(),
        };

        let filehistory = toml::toml().dependencies.historyfilepath;

        let mut def = Editor::with_config(configdef).unwrap();
        def.set_helper(Some(enveditor));
        def.bind_sequence(KeyEvent::alt('f'), Cmd::ForwardSearchHistory);
        def.bind_sequence(KeyEvent::alt('b'), Cmd::HistorySearchBackward);


        def.load_history(&filehistory).unwrap_or_else(|e| {
            let path = tell();
            match e {
                ReadlineError::Io(e) => {
                    match e.kind() {
                        std::io::ErrorKind::NotFound => eprintln!("[{path:?}]~>{} due to {}" , "Error".bold().bright_red() , "History file doesn't exist".bright_red().bold()),
                        _ => {}
                    }
                }
                _ => {}
            }
        });
    loop {
        let path = tell();
        let username = toml().customization.username;
        let f  = format!("[{path:?}][{username}]~>");
        
        let void = match def.readline(&f) {
            Ok(o) => {
                def.add_history_entry(o.as_str()).unwrap();
                o
            },

            Err(e) => match e {
                ReadlineError::Interrupted => {
                    break;
                }
                _ => {
                    eprintln!("[{path:?}]~>{}: due to {e}" , "Error".red());
                    return Ok(());
                }
            }
        };

        let data = parser(&void);
        let tok1 = match data.get(0) {
            Some(t) => t.to_owned(),
                None => {
                    continue;
            }
        };

        match tok1.trim() {
            "help" => {
                let tok1 = token(&data, 1).checker(Some("topic".to_string())).ughf();

                if let Ok(o) = tok1 {
                    commands::help(o);
                }
                else {
                    continue;
                }
            }
            "clean" => {
                commands::clean().unwrap_or_default();
            }
            "go" => {
                let tok2  = token(&data, 1).checker(Some("path".to_string())).ughf();
                if let Ok(o) =  tok2 {
                    if &o == "back" {
                        commands::go("..").unwrap_or_default();
                    }

                    else {
                        let tok2 = token(&data, 1).checker(Some("path".to_string())).ughf();
                        if let Ok(o) = tok2 {
                            commands::go(&o).unwrap_or_default();
                        }
                        else {
                            continue;
                        }
                    }
                }
                else {
                    continue;
                }
            }
            "wh" => {
                commands::wh().unwrap_or_default();
            }
            "see" => {
                commands::see().unwrap_or_default();
            }
            "peek" => {
                let tok2 = token(&data, 1).checker(Some("path or name".to_string())).ughf();

                if let Ok(o) = tok2{
                    commands::peek(&o).unwrap_or_default();
                }
                else {
                    continue;
                }
            }
            "mk" => {
                let tok2 = token(&data, 1).checker(Some("directory name".to_string())).ughf();

                if let Ok(o) = tok2 {
                    commands::mk(&o).unwrap_or_default();
                }
                else {
                    continue;
                }
            }
            "burn" => {
                let tok2 = token(&data, 1).checker(Some("path or name".to_string())).ughf();

                if let Ok(o) = tok2 {
                    commands::burn(&o).unwrap_or_default();
                }
                else {
                    continue;
                }
            }
            "rn" => {
                let tok1 = token(&data, 1).checker(Some("path or name".to_string())).ughf();
                let tok2 = token(&data, 2).checker(Some("output path or name".to_string())).ughf();

                if let (Ok(tok1) , Ok(tok2)) = (tok1 , tok2) {
                    commands::rn(&tok1 , &tok2).unwrap_or_default();
                }
                else {
                    continue;
                }
            }
            "clone" => {
                let tok1 = token(&data, 1).checker(Some("path or name".to_string())).ughf();
                let tok2 = token(&data, 2).checker(Some("output path or name".to_string())).ughf();

                if let (Ok(tok1) , Ok(tok2)) = (tok1 , tok2) {
                    commands::clone(&tok1 , &tok2).unwrap_or_default();
                }
                else {
                    continue;
                }
            }
            "forge" => {
                let tok1 = token(&data, 1).checker(Some("path or name".to_string())).ughf();

                if let Ok(tok1) = tok1 {
                    commands::forge(&tok1).unwrap_or_default();
                }
                else {
                    continue;
                }
            }
            "run" => {
                let tok1 = token(&data , 1).checker(Some("command name".to_string())).ughf();

                if let Ok(tok1) = tok1 {
                    commands::run(&tok1).unwrap_or_default();
                }
                else {
                    continue;
                }
            }
            "calc" => {
                let tok1 = token(&data , 1).checker(Some("expression".to_string())).ughf();
                
                if let Ok(tok1) = tok1 {
                    calc(&tok1);    
                }
                else {
                    continue;
                }
            }
            "time" => {
                time();
            }
            "mv" => {
                let tok1 = token(&data, 1).checker(Some("path or name".to_string())).ughf();
                let tok2 = token(&data, 2).checker(Some("output path or name".to_string())).ughf();

                if let (Ok(tok1) , Ok(tok2)) = (tok1 , tok2) {
                    commands::mv(&tok1, &tok2).unwrap_or_default();
                }
                else {
                    continue;
                }
            }
            "tar" => {
                let flag  = token(&data, 1).checker(Some("flag".to_string())).ughf();
                let fname = token(&data, 2).checker(Some("path or file".to_string())).ughf();
                let outname = token(&data, 3).checker(Some("path or name".to_string())).ughf();

                if let (Ok(flag) , Ok(fname) , Ok(outname)) = (flag, fname, outname) {
                    tar(&flag , &fname , &outname).unwrap_or_default();
                }
                else {
                    continue;
                }
            }
            "zip" => {
                let flag = token(&data, 1).checker(Some("flag".to_string())).ughf();
                let fname = token(&data, 2).checker(Some("path or file".to_string())).ughf();
                let n1 = token(&data, 3).checker(Some("path or name".to_string())).ughf();
                let f1 = token(&data, 4);
                let n2 = token(&data, 5);
                let f2 = token(&data, 6);
                let n3 = token(&data, 7);
                let f3 = token(&data, 8);

                if let (Ok(flag) , Ok(fname) , Ok(n1)) = (flag , fname , n1) {
                    let ziparg = ZipArg {
                        n1:&n1,
                        n2:&n2,
                        n3:&n3,
                        f1:&f1,
                        f2:&f2,
                        f3:&f3,
                    };
                    let zipdir = ZipDir {
                        src_dir:&fname,
                        res_dir:&n1
                    };
                    zip(&flag, &fname, ziparg , zipdir).unwrap_or_default();
                }
                else {
                    continue;
                }
            }
            "transmute" => {
                let ttype = token(&data, 1).checker(Some("type".to_string())).ughf();
                let flag  = token(&data, 2).checker(Some("flag".to_string())).ughf();
                let fname = token(&data, 3).checker(Some("file name".to_string())).ughf();
                let outname = token(&data, 4).checker(Some("output name".to_string())).ughf();

                if let (Ok(t) , Ok(f) , Ok(fna) , Ok(out)) = (ttype , flag , fname , outname) {
                    transmute(&t, &f, &fna, &out).unwrap_or_default();
                }
                else {
                    continue;
                }
            }
            "find" => {
                let fpath = token(&data, 1).checker(Some("path or name".to_string())).ughf();

                if let Ok(o) = fpath {
                    commands::find(&o).ugh();
                }
                else {
                    continue;
                } 
            }
            "ps" => {
                let tok1 = token(&data, 1).checker(Some("flag".to_string())).ughf();
                let tok2 = token(&data, 2).checker(Some("pid".to_string())).ughf();

                if let (Ok(tok1) , Ok(tok2)) = (tok1  , tok2) {
                    let tok2 = tok2.parse().map(|e: usize| e as usize).unwrap_or_default();
                
                    commands::ps(&tok1 , tok2).ugh();
                }
                else {
                    continue;
                }
            }
            "stop" => {
                let tok1 = token(&data, 1).checker(Some("pid".to_string())).ughf();

                if let Ok(tok1) = tok1 {
                    let tok1 = tok1.parse().map(|e:i32| e as i32).errh(Some(tok1.to_string())).ughv();
                    commands::stop(tok1);
                }
                else {
                    continue;
                }
            }
            "tree" => {
                let path = token(&data, 1).checker(None);
                if let Ok(o) = path {
                    apps::treee(o).ugh();
                } else {
                    let path = String::from(".");
                    apps::treee(path).ugh();
                }
            }
            "hostname" => {
                let flag = token(&data, 1).checker(Some("flag".to_string())).ughf();
                let newhostanme = token(&data, 2);

                if let Ok(o) = flag {
                    hostname(&o, &newhostanme).ugh();
                }
                else {
                    continue;
                }
            }
            "configer" => {
                let flag = token(&data, 1).checker(Some("flag".to_string())).ughf();
                let op = token(&data, 2).checker(Some("--set".to_string())).ughf();
                let username = token(&data, 3);
                let hispath = token(&data, 3);

                if let (Ok(o) , Ok(p)) = (flag , op) {
                    toml::configer(&o, &username, &hispath, &p).ugh();
                }
            }
            "end" => {
                break;
            }
            _ => {
                if !tok1.is_empty() {
                    println!("[{path:?}]~>{}: [\x1b[36m{}\x1b[0m] please try again" , "Unknown command".bright_red() ,&tok1 );
                    continue;
                }
            }
        }
    }
    let path = tell();
        def.save_history(&filehistory).unwrap_or_else(|e| 
            {eprintln!("[{path:?}]~>{}: due to {}" , "Error".bright_red().bold() , e.to_string().bright_red().bold())});
        
        Ok(())
}