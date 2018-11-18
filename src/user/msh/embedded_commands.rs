use alloc::vec::Vec;

use librust;

use super::utils;

pub fn help_cmd() {
    /// prints embedded help
    println!("msh: 0.0.1\nAvailable commands: help, uname, date, pwd, cd, ls");
}

pub fn uname_cmd() {
    /// returns info about the system
    let uname = librust::syscall::UtsName::new();
    unsafe { librust::syscall::uname(&uname) };
    println!("{} - {}", uname.sysname, uname.version);
}

pub fn date_cmd() {
    /// returns current timestamp
    let time = unsafe { librust::syscall::time() };
    println!("{}", time);
}

pub fn unknown_cmd(cmd: &str) {
    /// msh executes this function when the command
    /// entered by user is unknown
    println!("Unknown command: '{}'", cmd)
}

pub fn pwd_cmd() {
    /// prints current working directory
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
    let result = unsafe { librust::syscall::chdir(new_dir.as_bytes()) };
    if result != 0 {
        match result {
            2 => println!("cd: No such file or directory"),
            _ => println!("cd: unknown error"),
        }
    }
}

pub fn ls_cmd() {
    /// returns list of files and directories in the current working directory
    unsafe {
        let mut offset = 0;
        loop {
            // readdir returns files/dirs one by one,
            // so we keep offset counter and ask the mos while
            // it responds with 0 result code
            let mut buf: [u8; 255] = ['\0' as u8; 255];
            let result = librust::syscall::readdir(&mut buf, offset);
            if result != 0 {
                // if the result code is not 0, we got all content of the directory
                // or something bad happened :-(
                match result {
                    2 => println!("ls: No such file or directory"),
                    _ => {}
                }
                break;
            }
            println!("  {}", utils::read_str(&buf));
            offset += 1;
        }
    }
}
