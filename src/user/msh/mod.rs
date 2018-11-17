use alloc::string::String;
use alloc::vec::Vec;
use librust;
use librust::std::screen::{clear, printb};
use multitasking::focus::focus;
use multitasking::scheduler;

pub fn start() {
    scheduler::spawn(_start);
}

const PROMPT: &str = "[alex] / > ";

fn _start() {
    _prepare_start();
    _print_intro();
    cmd_loop();
}

fn _prepare_start() {
    let pid = unsafe { librust::syscall::getpid() };
    system_log!("[msh] started with pid {}", pid);
    focus(pid as u32);
}

fn _print_intro() {
    clear();
    println!("Welcome to mOS!\n");
}

fn cmd_loop() {
    printf!(PROMPT);
    let mut buf = Vec::new();

    loop {
        let c = librust::std::getchar();
        if c == '\n' {
            buf = process_cmd(&mut buf).to_vec();
            printf!(PROMPT);
        } else {
            buf.push(c as u8);
            printf!("{}", c);
        }
        librust::std::time::sleep();
    }
}

fn process_cmd(buf: &mut Vec<u8>) -> &mut Vec<u8> {
    println!("");

    let cmd = unsafe { &String::from_utf8_unchecked(buf.to_vec()) };

    if cmd == "help" {
        println!("msh: 0.0.1\nAvailable commands: help, uname, date");
    }
    if cmd == "uname" {
        let uname = librust::syscall::UtsName::new();
        unsafe { librust::syscall::uname(&uname) };
        println!("{} - {}", uname.sysname, uname.version);
    }
    if cmd == "date" {
        let time = unsafe { librust::syscall::time() };
        println!("{}", time);
    }

    buf.clear();
    return buf;
}
