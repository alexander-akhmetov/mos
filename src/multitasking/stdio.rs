use alloc::collections::vec_deque::VecDeque;
use alloc::string::String;
use alloc::vec::Vec;
use fs::FileDescriptor;
use multitasking::process::Process;

pub struct StdIn {
    buffer: VecDeque<char>,
    pid: u32,
}

impl StdIn {
    pub fn new(pid: u32) -> StdIn {
        StdIn {
            buffer: VecDeque::new(),
            pid: pid,
        }
    }

    pub fn push(&mut self, c: char) {
        self.buffer.push_front(c);
    }

    pub fn pop(&mut self, c: char) -> Option<char> {
        return self.buffer.pop_back();
    }
}

pub struct StdOut {
    buffer: VecDeque<char>,
    pid: u32,
}

impl StdOut {
    pub fn new(pid: u32) -> StdOut {
        StdOut {
            buffer: VecDeque::new(),
            pid: pid,
        }
    }

    pub fn push(&mut self, c: char) {
        self.buffer.push_front(c);
    }

    pub fn pop(&mut self, c: char) -> Option<char> {
        return self.buffer.pop_back();
    }
}

impl FileDescriptor for StdOut {
    fn read(&self) -> Vec<u8> {
        return Vec::new();
    }

    fn name(&self) -> String {
        return String::from("stdout");
    }

    fn write(&self, buf: Vec<u8>) {
        let s = String::from_utf8(buf.clone()).unwrap();
        system_log!("STDOUT: pid={} msg='{}'", self.pid, s);
        kprint!("{}", &s);
    }
}
