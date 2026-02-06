use crate::backend::clean::{ExtractOptions, ExtractOptionsErr, read_file_cont};
use crate::backend::safe::{ErrH, HyperkitError, Ugh, Ughv};
use crate::backend::standard::tell;
use colored::*;
use core::str;
use std::fs;
use std::io::Read;
use termtree::Tree;

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

fn get_(o: &str) -> std::result::Result<&str, HyperkitError> {
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
                let os_name = get_(*o)?;
                os_name
            } else {
                "NONE"
            };

            let os_version = if let Some(o) = os.iter().find(|s| s.starts_with("VERSION")) {
                let os_version = get_(*o)?;
                os_version
            } else {
                "NONE"
            };

            let os_release_type = if let Some(o) = os.iter().find(|s| s.starts_with("RELEASE_TYPE"))
            {
                let os_release_type = get_(*o)?;
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

            println!("        {}", "---CPU---".bright_cyan().bold());
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

            let mut bat = String::new();
            let mut open = open_file_bat("/sys/class/hwmon/hwmon1/temp1_input")?;

            open.read_to_string(&mut bat).errh(None).ughv();

            let bat_temp: Vec<&str> = bat.lines().collect();

            let bat_temp1 = if let Some(o) = bat_temp.get(0) {
                o.parse::<i64>().errh(None).ughv() / 1000
            } else {
                0
            };

            println!("        {}", "---Battery---".bright_cyan().bold());

            println!(
                "   -[{}] = ~{}{}~ ",
                "Overall Battery Temp".bright_cyan().bold(),
                bat_temp1.to_string().bright_yellow().bold(),
                "C".bright_yellow().bold()
            );

            let mut composite = String::new();
            let mut open = open_file_bat("/sys/class/hwmon/hwmon2/temp1_input")?;
            open.read_to_string(&mut composite).errh(None).ughv();

            let composite_temp: Vec<&str> = composite.lines().collect();

            let composite_temp = if let Some(o) = composite_temp.get(0) {
                o.parse::<i64>().errh(None).ughv() / 1000
            } else {
                0
            };

            println!("        {}", "---Composite---".bright_cyan().bold());

            println!(
                "   -[{}] = ~{}{}~ ",
                "Overall Composite Temp".bright_cyan().bold(),
                composite_temp.to_string().bright_yellow().bold(),
                "C".bright_yellow().bold()
            );

            let mut wifi_temp = String::new();
            let mut open = open_file_bat("/sys/class/hwmon/hwmon6/temp1_input")?;
            open.read_to_string(&mut wifi_temp).errh(None).ughv();

            let wifi_temp: Vec<&str> = wifi_temp.lines().collect();

            let wifi_temp = if let Some(o) = wifi_temp.get(1) {
                o.parse::<i64>().errh(None).ughv() / 1000
            } else {
                0
            };

            println!("        {}", "---Wifi---".bright_cyan().bold());

            println!(
                "   -[{}] = ~{}{}~ ",
                "Overall Wifi Temp".bright_cyan().bold(),
                wifi_temp.to_string().bright_yellow().bold(),
                "C".bright_yellow().bold()
            );

            let mut all = String::new();
            let mut cpu = open_file_bat("/sys/class/hwmon/hwmon3/temp1_input")?;
            let mut ambient = open_file_bat("/sys/class/hwmon/hwmon3/temp2_input")?;
            let mut sodimm = open_file_bat("/sys/class/hwmon/hwmon3/temp3_input")?;

            cpu.read_to_string(&mut all).errh(None).ughv();
            ambient.read_to_string(&mut all).errh(None).ughv();
            sodimm.read_to_string(&mut all).errh(None).ughv();

            let all: Vec<&str> = all.lines().collect();

            let cpu = if let Some(o) = all.get(0) {
                o.parse::<i64>().errh(None).ughv() / 1000
            } else {
                0
            };

            let ambient = if let Some(o) = all.get(1) {
                o.parse::<i64>().errh(None).ughv() / 1000
            } else {
                0
            };

            let sodimm = if let Some(o) = all.get(2) {
                o.parse::<i64>().errh(None).ughv() / 1000
            } else {
                0
            };

            println!("        {}", "---Others---".bright_cyan().bold());

            println!(
                "   -[{}] = ~{}{}~ ",
                "CPU".bright_cyan().bold(),
                cpu.to_string().bright_yellow().bold(),
                "C".bright_yellow().bold()
            );

            println!(
                "   -[{}] = ~{}{}~ ",
                "Ambient".bright_cyan().bold(),
                ambient.to_string().bright_yellow().bold(),
                "C".bright_yellow().bold()
            );

            println!(
                "   -[{}] = ~{}{}~ ",
                "Sodimm".bright_cyan().bold(),
                sodimm.to_string().bright_yellow().bold(),
                "C".bright_yellow().bold()
            );
        }
        "bios" => {
            let mut bios = String::new();

            let mut bios_date = open_file_bat("/sys/devices/virtual/dmi/id/bios_date")?;
            let mut bios_release = open_file_bat("/sys/devices/virtual/dmi/id/bios_release")?;
            let mut bios_vendor = open_file_bat("/sys/devices/virtual/dmi/id/bios_vendor")?;
            let mut bios_version = open_file_bat("/sys/devices/virtual/dmi/id/bios_version")?;
            let mut board_name = open_file_bat("/sys/devices/virtual/dmi/id/board_name")?;
            let mut product_family = open_file_bat("/sys/devices/virtual/dmi/id/product_family")?;
            let mut product_name = open_file_bat("/sys/devices/virtual/dmi/id/product_name")?;

            bios_date.read_to_string(&mut bios).errh(None).ughv();
            bios_release.read_to_string(&mut bios).errh(None).ughv();
            bios_vendor.read_to_string(&mut bios).errh(None).ughv();
            bios_version.read_to_string(&mut bios).errh(None).ughv();
            board_name.read_to_string(&mut bios).errh(None).ughv();
            product_family.read_to_string(&mut bios).errh(None).ughv();
            product_name.read_to_string(&mut bios).errh(None).ughv();

            let bios: Vec<&str> = bios.lines().collect();

            let bios_date = if let Some(o) = bios.get(0) { o } else { "NONE" };

            let bios_release = if let Some(o) = bios.get(1) { o } else { "NONE" };

            let bios_vendor = if let Some(o) = bios.get(2) { o } else { "NONE" };

            let bios_version = if let Some(o) = bios.get(3) { o } else { "NONE" };

            let board_name = if let Some(o) = bios.get(4) { o } else { "NONE" };

            let product_family = if let Some(o) = bios.get(5) { o } else { "NONE" };

            let product_name = if let Some(o) = bios.get(6) { o } else { "NONE" };

            println!(
                "   -[{}] = ~{}~ ",
                "Bios Date".bright_cyan().bold(),
                bios_date.to_string().bright_yellow().bold()
            );

            println!(
                "   -[{}] = ~{}~ ",
                "Bios Release".bright_cyan().bold(),
                bios_release.to_string().bright_yellow().bold()
            );

            println!(
                "   -[{}] = ~{}~ ",
                "Bios Vendor".bright_cyan().bold(),
                bios_vendor.to_string().bright_yellow().bold()
            );

            println!(
                "   -[{}] = ~{}~ ",
                "Bios Version".bright_cyan().bold(),
                bios_version.to_string().bright_yellow().bold()
            );

            println!(
                "   -[{}] = ~{}~ ",
                "Board Name".bright_cyan().bold(),
                board_name.to_string().bright_yellow().bold()
            );

            println!(
                "   -[{}] = ~{}~ ",
                "Product Name".bright_cyan().bold(),
                product_name.to_string().bright_yellow().bold()
            );

            println!(
                "   -[{}] = ~{}~ ",
                "Product Family".bright_cyan().bold(),
                product_family.to_string().bright_yellow().bold()
            );
        }
        "disk" => {
            let mut part1 = String::new();
            let mut uevent = open_file_bat("/sys/class/block/nvme0n1/nvme0n1p1/uevent")?;
            uevent.read_to_string(&mut part1).errh(None).ughv();
            let size = read_file_cont("/sys/class/block/nvme0n1/nvme0n1p1/size")
                .ughv()
                .trim()
                .parse::<i64>()
                .errh(None)
                .ughv()
                * 512
                / 1024
                / 1024;
            let part1: Vec<&str> = part1.lines().collect();

            let partname = if let Some(o) = part1.iter().find(|s| s.starts_with("PARTNAME=")) {
                get_(o).ughv()
            } else {
                "NONE"
            };

            let devname = if let Some(o) = part1.iter().find(|s| s.starts_with("DEVNAME=")) {
                get_(o).ughv()
            } else {
                "NONE"
            };

            let partnum = if let Some(o) = part1.iter().find(|s| s.starts_with("PARTN=")) {
                get_(o).ughv()
            } else {
                "NONE"
            };

            let mut part2 = String::new();
            let mut uevent = open_file_bat("/sys/class/block/nvme0n1/nvme0n1p2/uevent")?;
            uevent.read_to_string(&mut part2).errh(None).ughv();
            let size2 = read_file_cont("/sys/class/block/nvme0n1/nvme0n1p2/size")
                .ughv()
                .trim()
                .parse::<i64>()
                .errh(None)
                .ughv()
                * 512
                / 1024
                / 1024
                / 1024;
            let part2: Vec<&str> = part2.lines().collect();

            let partname2 = if let Some(o) = part2.iter().find(|s| s.starts_with("PARTNAME=")) {
                get_(o).ughv()
            } else {
                "NONE"
            };

            let devname2 = if let Some(o) = part2.iter().find(|s| s.starts_with("DEVNAME=")) {
                get_(o).ughv()
            } else {
                "NONE"
            };

            let partnum2 = if let Some(o) = part2.iter().find(|s| s.starts_with("PARTN=")) {
                get_(o).ughv()
            } else {
                "NONE"
            };

            let mut part3 = String::new();
            let mut uevent = open_file_bat("/sys/class/block/nvme0n1/nvme0n1p3/uevent")?;
            uevent.read_to_string(&mut part3).errh(None).ughv();
            let size3 = read_file_cont("/sys/class/block/nvme0n1/nvme0n1p3/size")
                .ughv()
                .trim()
                .parse::<i64>()
                .errh(None)
                .ughv()
                * 512
                / 1024
                / 1024
                / 1024;
            let part3: Vec<&str> = part3.lines().collect();

            let partname3 = if let Some(o) = part3.iter().find(|s| s.starts_with("PARTNAME=")) {
                get_(o).ughv()
            } else {
                "NONE"
            };

            let devname3 = if let Some(o) = part3.iter().find(|s| s.starts_with("DEVNAME=")) {
                get_(o).ughv()
            } else {
                "NONE"
            };

            let partnum3 = if let Some(o) = part3.iter().find(|s| s.starts_with("PARTN=")) {
                get_(o).ughv()
            } else {
                "NONE"
            };

            println!(
                "        {}~[{}]",
                "Partition".bright_cyan().bold(),
                partnum.bright_yellow().bold()
            );

            println!(
                "   -[{}] = ~{}~ ",
                "Partition Name".bright_cyan().bold(),
                partname.to_string().bright_yellow().bold()
            );

            println!(
                "   -[{}] = ~{}~ ",
                "Partition At".bright_cyan().bold(),
                devname.to_string().bright_yellow().bold()
            );

            println!(
                "   -[{}] = ~{}{}~ ",
                "Partition Size".bright_cyan().bold(),
                size.to_string().bright_yellow().bold(),
                "Mib".bright_yellow().bold()
            );

            println!(
                "        {}~[{}]",
                "Partition".bright_cyan().bold(),
                partnum2.bright_yellow().bold()
            );

            println!(
                "   -[{}] = ~{}~ ",
                "Partition Name".bright_cyan().bold(),
                partname2.to_string().bright_yellow().bold()
            );

            println!(
                "   -[{}] = ~{}~ ",
                "Partition At".bright_cyan().bold(),
                devname2.to_string().bright_yellow().bold()
            );

            println!(
                "   -[{}] = ~{}{}~ ",
                "Partition Size".bright_cyan().bold(),
                size2.to_string().bright_yellow().bold(),
                "GiB".bright_yellow().bold()
            );

            println!(
                "        {}~[{}]",
                "Partition".bright_cyan().bold(),
                partnum3.bright_yellow().bold()
            );

            println!(
                "   -[{}] = ~{}~ ",
                "Partition Name".bright_cyan().bold(),
                partname3.to_string().bright_yellow().bold()
            );

            println!(
                "   -[{}] = ~{}~ ",
                "Partition At".bright_cyan().bold(),
                devname3.to_string().bright_yellow().bold()
            );

            println!(
                "   -[{}] = ~{}{}~ ",
                "Partition Size".bright_cyan().bold(),
                size3.to_string().bright_yellow().bold(),
                "GiB".bright_yellow().bold()
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
