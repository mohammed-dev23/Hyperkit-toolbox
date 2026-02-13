use colored::Colorize;

use crate::{
    backend::{
        clean::read_file_cont,
        safe::{ErrH, HyperkitError, Ughv},
        standard::tell,
    },
    toml::toml,
};
use std::{
    net::{SocketAddr, TcpStream},
    path::PathBuf,
    time::Duration,
};

fn scaner(
    file: String,
    part1: u8,
    part2: u8,
    part3: u8,
    part4: u8,
    open_only: &str,
) -> std::result::Result<(), HyperkitError> {
    let ports: Vec<u16> = file
        .split(',')
        .map(|s| s.trim().parse::<u16>().errh(None).ughv())
        .collect();

    for i in ports {
        let ports = [SocketAddr::from(([part1, part2, part3, part4], i as u16))];

        match TcpStream::connect_timeout(&ports[0], Duration::from_millis(500)) {
            Ok(_) => {
                if open_only == "--open-only" {
                    println!(
                        "   {}{}: port {i}",
                        "~".bright_cyan().bold(),
                        "open".bright_green().bold()
                    );
                } else {
                    println!(
                        "   {}{}: port {i}",
                        "~".bright_cyan().bold(),
                        "open".bright_green().bold()
                    );
                }
            }
            Err(e) => {
                if open_only != "--open-only" {
                    match e.kind() {
                        std::io::ErrorKind::ConnectionRefused => println!(
                            "   {}{}: port {i}",
                            "~".bright_cyan().bold(),
                            "closed".bright_red().bold()
                        ),
                        _ => println!(
                            "   {}{}: port {i}",
                            "~".bright_cyan().bold(),
                            "unkowun state".bright_red().bold()
                        ),
                    }
                }
            }
        }
    }
    Ok(())
}

pub fn fang(
    ipaddr: String,
    scaning_type: &str,
    type_: &str,
    open_only: &str,
) -> std::result::Result<(), HyperkitError> {
    let ipaddr: Vec<&str> = ipaddr.split('.').collect();

    let (part1, part2, part3, part4) = if let (Some(p1), Some(p2), Some(p3), Some(p4)) =
        (ipaddr.get(0), ipaddr.get(1), ipaddr.get(2), ipaddr.get(3))
    {
        let p1 = p1.parse::<u8>().errh(None)?;
        let p2 = p2.parse::<u8>().errh(None)?;
        let p3 = p3.parse::<u8>().errh(None)?;
        let p4 = p4.parse::<u8>().errh(None)?;

        (p1, p2, p3, p4)
    } else {
        return Err(HyperkitError::MissingParameter(Some(
            "The ipaddr is missing and it must be from 0.0.0.0 digits".to_string(),
        )));
    };

    let path = PathBuf::from(toml().dependencies.fangdirpath);
    let tell = tell();

    match type_ {
        "-tcp" => match scaning_type {
            "--most-popular-ports" => {
                let most_popular_ports = read_file_cont(
                    path.join("most_popular_ports_tcp_upd.txt")
                        .to_string_lossy()
                        .to_string()
                        .trim(),
                )
                .ughv();
                scaner(most_popular_ports, part1, part2, part3, part4, open_only)?;
            }
            "--top100" => {
                let top100 = read_file_cont(
                    path.join("top100_tcp.txt")
                        .to_string_lossy()
                        .to_string()
                        .trim(),
                )
                .ughv();
                scaner(top100, part1, part2, part3, part4, open_only)?;
            }
            "--top1000" => {
                let top1000 = read_file_cont(
                    path.join("top1000_tcp.txt")
                        .to_string_lossy()
                        .to_string()
                        .trim(),
                )
                .ughv();
                scaner(top1000, part1, part2, part3, part4, open_only)?;
            }
            "--full-tcp" => {
                let top1000 = read_file_cont(
                    path.join("full_tcp.txt")
                        .to_string_lossy()
                        .to_string()
                        .trim(),
                )
                .ughv();
                scaner(top1000, part1, part2, part3, part4, open_only)?;
            }
            _ => {
                if scaning_type.is_empty() {
                    println!(
                        "[{tell:?}]~>{}: due to [{}]",
                        "Error".red().bold(),
                        "missing scaning type".bright_yellow().bold()
                    )
                } else {
                    println!(
                        "[{tell:?}]~>{}: due to [{}: <{}>]",
                        "Error".red().bold(),
                        "unsupported scaning type".red().bold(),
                        scaning_type.bright_yellow().bold()
                    )
                }
            }
        },
        _ => {
            if type_.is_empty() {
                println!(
                    "[{tell:?}]~>{}: due to [{}]",
                    "Error".red().bold(),
                    "missing ports type TCP/UPD".bright_yellow().bold()
                )
            } else {
                println!(
                    "[{tell:?}]~>{}: due to [{}: <{}>]",
                    "Error".red().bold(),
                    "unsupported ports type".red().bold(),
                    type_.bright_yellow().bold()
                )
            }
        }
    }
    Ok(())
}
