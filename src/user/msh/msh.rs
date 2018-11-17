use alloc::string::String;
use alloc::vec::Vec;

use librust;
use librust::std::screen::{clear, printb};
use multitasking::focus::focus;

use super::constants;
use super::embedded_commands;

/*
    msh is a main shell for the mOS.
*/

pub fn start() {
    // gets the id of the msh and prints it with debug system call
    let pid = unsafe { librust::syscall::getpid() };
    system_log!("[msh] started with pid {}", pid);

    // it need to put itself to the focus to start getting keyboard input
    // todo: change this
    focus(pid as u32);

    // clear screen and print welcome message
    clear();
    println!("Welcome to mOS!\n");

    // start main command loop of the shell
    cmd_loop();
}

fn cmd_loop() {
    /// main command processing loop,
    /// reads char from stdin, appends it to the buf
    /// and if char is a new line, starts executing program function
    let mut buf = Vec::new();

    printf!(constants::PROMPT);

    loop {
        match librust::std::getchar() {
            '\n' => {
                // new line, user pressed enter, try to process input command
                buf = process_command(&mut buf).to_vec();
                // todo: fork + execve + wait for child process to complete
                //
                // after command executing print prompt again
                printf!(constants::PROMPT)
            }
            c => {
                // this char is not a new line, just append
                // it to the buf and print to the screen
                buf.push(c as u8);
                printf!("{}", c);
            }
        }

        // sleep a little to prevent CPU burning
        librust::std::time::sleep();
    }
}

fn process_command(buf: &mut Vec<u8>) -> &mut Vec<u8> {
    /// checks if there is a known command in the buf
    /// and if it is, starts execution
    println!(""); // print empty string with new line to move cursor down

    unsafe {
        match String::from_utf8_unchecked(buf.to_vec()).as_str() {
            "help" => embedded_commands::help_cmd(),
            "uname" => embedded_commands::uname_cmd(),
            "date" => embedded_commands::date_cmd(),
            cmd => embedded_commands::unknown_cmd(cmd),
        }
    }

    // clear buffer to remove just executed command from it
    buf.clear();
    return buf;
}
