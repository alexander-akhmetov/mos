use alloc::string::String;

use librust;

use super::utils;

pub fn help_cmd() {
    println!("msh: 0.0.1\nAvailable commands: help, uname, date, pwd");
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

pub fn cd(new_dir: &str) {}
