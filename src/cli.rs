pub mod cli {
    use clap::{ColorChoice, Parser, Subcommand};
    use crate::{apps::{self, FileZipArg}, backend::{clean::ExtractOptions, commands::{self, rn}, safe::{HyperkitError, Ugh}}};

    #[derive(Parser)]
    #[command(version , color = ColorChoice::Always)]
    pub struct Cli {
        #[command(subcommand)]
        commands:Option<Commandd>
    }

    #[derive(Subcommand , Clone)]
    pub enum Commandd {
        #[command(about = "\x1b[33m\x1b[1m -use it to look up for files in your system by there name- \x1b[0m\x1b[0m" , color = ColorChoice::Always)]
        Find {
            #[arg(short = 't' , help = "the name of the file" , required = true , value_name = "File Name")]
            file:String
        },
        #[command(about = "\x1b[33m\x1b[1m -use it to peek into files- \x1b[0m\x1b[0m" , color = ColorChoice::Always)]
        Peek {
            #[arg(short = 'o' , help = "the path or name of the file" , required = true , value_name = "File/Path")]
            file:String
        },
        #[command(about = "\x1b[33m\x1b[1m -use it to delete files or directorys from your system- \x1b[0m\x1b[0m" , color = ColorChoice::Always)]
        Burn {
            #[arg(short = 't' , help = "the name or the path of the file you want to delete" , value_name = "File/Path" , required = true)]
            file:String
        },
        #[command(about = "\x1b[33m\x1b[1m -use to rename files on your system- \x1b[0m\x1b[0m" , color = ColorChoice::Always)]
        Rn {
            #[arg(short = 'f' , help = "add the file you want to rename" , required = true , value_name = "File/Path")]
            from:String,
            #[arg(short = 't' , help = "the new name you want to give to the file" , default_value = "output" , value_name = "New Name")]
            to:String,
        },
        #[command(about = "\x1b[33m\x1b[1m -use it to clone files on your system- \x1b[0m\x1b[0m" , color = ColorChoice::Always)]
        Clone {
            #[arg(long = "target" , help = "the targted file you want to clone" , required = true , value_name = "File/Path")]
            target:String,
            #[arg(long = "output" , help = "the output name of the cloned file or the name of exsisted one" , default_value = "output"  , value_name = "New Name/Exsisted file")]
            outputname:String
        },
        #[command(about = "\x1b[33m\x1b[1m -use it if you want to make a new file but don't forget to add the format with it- \x1b[0m\x1b[0m" , color = ColorChoice::Always)]
        Forge {
            #[arg(long = "make" , short = 'm' , help = "the name of the file you want to make" , required = true , value_name = "Name+format")]
            name:String
        },
        #[command(about = "\x1b[33m\x1b[1m -a calculator- \x1b[0m\x1b[0m")]
        Calc {
            #[arg(long = "slove" , help = "put the math problome you want to slove" , required = true , value_name = "Math")]
            math:String
        },
        #[command(about = "\x1b[33m\x1b[1m -use it to move files from place to another- \x1b[0m\x1b[0m" , color = ColorChoice::Always)]
        Mv {
            #[arg(short = 'f' , help = "the name or the path you want to move" , required = true , value_name = "File/Path")]
            from:String,
            #[arg(short = 't' , help = "the path of the new place you want to put it in" , required = true , value_name = "Path")]
            to:String
        },
        #[command(about = "\x1b[33m\x1b[1m -used to make/extract tar archives- \x1b[33m\x1b[0m" , color = ColorChoice::Always)]
        Tar {
            #[arg(short = 'f' , help = "the opration you want to run" , value_name = "load/Unload" , required = true)]
            flag:String,
            #[arg(short = 's', help = "the name or the path of the file that you want to run the opration on" , required = true)]
            the_name:String,
            #[arg(short = 'o' , help = "the output name" , value_name = "Name" , default_value = "output")]
            output:String
        },
        #[command(about = "\x1b[33m\x1b[1m -use it to encode and decode files- \x1b[0m\x1b[0m", color = ColorChoice::Always)]
        Transmute {
            #[arg(short = 't' , help = "the type of encoding/decodeing you want to use" , required = true , value_name = "Type")]
            ttype:String,
            #[arg(short = 'f' , help = "the opration you want to run" , value_name = "enc/dec" , required = true)]
            flag:String,
            #[arg(short = 's' , help = "the file name or path that you want to encode or decode" , value_name = "File" , required = true)]
            fname:String,
            #[arg(short = 'o' , help = "the output name" , value_name = "Name" , required = true)]
            outname:String,
        },
        #[command(about = "\x1b[33m\x1b[1m -use it to make directorys- \x1b[0m\x1b[0m" , color = ColorChoice::Always)]
        Mk {
            #[arg(required = true , help = "put the name of the directory you want to make" , value_name = "Name")]
            name:String
        },
        #[command(about = "\x1b[33m\x1b[1m -use it to see the time- \x1b[0m\x1b[0m" , color = ColorChoice::Always)]
        Time,
        #[command(about = "\x1b[33m\x1b[1m -use it to see the running process in your system and search for them- \x1b[0m\x1b[0m" , color = ColorChoice::Always)]
        Ps {
            #[arg(short = 'f', help = "choose one of the two opration you can do <SF:to search for prosses by there pid/A:to list all the running prosses>" , value_name = "SF/A" , default_value = None)]
            flag:Option<String>,
            #[arg(long = "pid" ,help = "put the process pid you want to search for" , value_name = "pid" , default_value = None)]
            pid:Option<usize>
        },
        #[command(about  = "\x1b[33m\x1b[1m -use it to stop running prosses in your system by there pid- \x1b[0m\x1b[0m" , color = ColorChoice::Always)]
        Stop {
            #[arg(long = "pid" , required = true , help = "add the pid number of the process you want to shutdown" , value_name = "pid")]
            pid:i32
        },
        #[command(about  = "\x1b[33m\x1b[1m -use it to run cli or desktop apps- \x1b[0m\x1b[0m" , color = ColorChoice::Always)]
        Run {
            #[arg(required = true , help = "the name of the app you want to run" , value_name = "Name")]
            name:String
        },
        #[command(about = "\x1b[33m\x1b[1m -used to make/extarct zip archives- \x1b[0m\x1b[0m" , color = ColorChoice::Always)]
        Zip {
            #[arg(short = 'o' , required = true , help = "the opration you want to make <Zip-New: to make a new zip archive/Zip-All: to comparse any directory or files into archive/extract: to extract zip archives>" , value_name = "Zip-New/Zip-All/extract")]
            op:String,

            #[arg(long = "name" , value_name = "File/Name" , help = "the name of the new file you are making" , default_value = "" )]
            file_name:String,
            #[arg(long = "f1" , value_name = "File/Name" , help = "to add or make a new file inside the new archive you are making" , default_value = "" )]
            f1:String,
            #[arg(long = "f2" , value_name = "File/Name" , help = "to add or make a new file inside the new archive you are making" , default_value = "")]
            f2:String,
            #[arg(long = "f3" , value_name = "File/Name" , help = "to add or make a new file inside the new archive you are making" , default_value = "")]
            f3:String,
            
            #[arg(short = 's' , value_name = "File/Path/Dir" , help = "the dir you want to run opraion on" , default_value = "") ]
            src_dir:String,
            #[arg(short = 'r' , value_name = "Name" , help = "the name of the new file you after extracion or composriong" , default_value = "")]
            res_dir:String
        },
        #[command(about = "\x1b[33m\x1b[1m -use it to list a directory in tree format- \x1b[0m\x1b[0m", color = ColorChoice::Always)]
        Tree {
            #[arg(short = 's' , value_name = "Path" , default_value = "." , help = "the path of the directory you want to list")]
            path:String
        }
    }

    pub fn cli() -> std::result::Result<() , HyperkitError> {
        let cli = Cli::parse();

        match &cli.commands {
            Some(Commandd::Find { file} ) => commands::find(file)?,
            Some(Commandd::Peek { file }) => commands::peek(file)?,
            Some(Commandd::Burn { file }) => commands::burn(file)?,
            Some(Commandd::Rn { from, to }) => rn(from, to)?,
            Some(Commandd::Clone { target, outputname }) => commands::clone(target, outputname)?,
            Some(Commandd::Forge { name }) => commands::forge(name)?,
            Some(Commandd::Calc { math }) => apps::calc(math),
            Some(Commandd::Mv { from, to }) => commands::mv(from, to)?,
            Some(Commandd::Tar { flag, the_name, output }) => {
                let t = format!("--{}" , flag);
                apps::tar(t.as_str(), the_name, output)?;
            },
            Some(Commandd::Transmute { ttype, flag, fname, outname }) => {
                let t = format!("--{}" , flag);
                apps::transmute(ttype, &t.as_str(), fname, outname)?;
            },
            Some(Commandd::Mk { name }) => commands::mk(name)?,
            Some(Commandd::Time) => apps::time(),
            Some(Commandd::Ps { flag, pid }) => {
                let flag = format!("-{}" , flag.extract());
                commands::ps(flag.as_str(), pid.extract())?;
            },
            Some(Commandd::Stop { pid }) => commands::stop(*pid),
            Some(Commandd::Run { name }) => commands::run(name)?,
            Some(Commandd::Zip { op, file_name, f1, f2, f3, src_dir, res_dir }) => {
                let file_name = file_name;
                let files = FileZipArg {
                    f1:&f1,
                    f2:&f2,
                    f3:&f3,
                };
                let src_dir = src_dir;
                let res_dir = res_dir;
                apps::cli_zip(op.as_str(), files, file_name.as_str(), src_dir.as_str(), res_dir.as_str())?;
            }
            Some(Commandd::Tree { path }) => apps::treee(path).ugh(),
            None => {}
        }
        Ok(())
    }
}