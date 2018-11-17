use alloc::string::String;
use alloc::vec::Vec;

use librust;

use super::utils;

pub fn help_cmd() {
    println!("msh: 0.0.1\nAvailable commands: help, uname, date, pwd, cd");
}

pub fn uname_cmd() {
    let uname = librust::syscall::UtsName::new();
    unsafe { librust::syscall::uname(&uname) };
    println!("{} - {}", uname.sysname, uname.version);
}

pub fn date_cmd() {
    let time = unsafe { librust::syscall::time() };
    println!("{}", time);
}

pub fn unknown_cmd(cmd: &str) {
    println!("Unknown command: '{}'", cmd)
}

pub fn pwd_cmd() {
    println!("{}", utils::get_pwd());
}

pub fn cd_cmd(args: Vec<&str>) {
    if args.len() > 2 {
        println!("cd: too many arguments")
    }

    let mut new_dir = "/";
    if args.len() == 2 {
        new_dir = args[1];
    }
    unsafe { librust::syscall::chdir(new_dir.as_bytes()) };
}

pub fn ls_cmd() {
    unsafe {
        let mut offset = 0;
        loop {
            let mut buf: [u8; 255] = ['\0' as u8; 255];
            let result = librust::syscall::readdir(&mut buf, offset);
            if result != 0 {
                match result {
                    2 => println!("ls: No such file or directory"),
                    _ => {}
                }
                break;
            }
            println!("{}", utils::read_str(&buf));
            offset += 1;
        }
    }
}
