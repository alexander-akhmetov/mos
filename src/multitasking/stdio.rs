use alloc::string::String;
use alloc::vec::Vec;
use fs::FileDescriptor;
use multitasking::{focus, scheduler};

pub struct StdIn {
    buffer: Vec<u8>,
    pid: u32,
}

impl StdIn {
    pub fn new(pid: u32) -> StdIn {
        StdIn {
            buffer: Vec::new(),
            pid: pid,
        }
    }
}

impl FileDescriptor for StdIn {
    fn read(&mut self) -> Vec<u8> {
        let mut vec = self.buffer.clone();
        self.buffer.clear();
        vec.reverse();
        return vec;
    }

    fn readc(&mut self) -> Option<u8> {
        return self.buffer.pop();
    }

    fn name(&self) -> String {
        return String::from("<stdin>");
    }

    fn write(&mut self, buf: Vec<u8>) {
        for c in buf.iter() {
            self.buffer.insert(0, *c);
        }
    }
}

pub struct StdOut {
    buffer: Vec<u8>,
    pid: u32,
}

impl StdOut {
    pub fn new(pid: u32) -> StdOut {
        StdOut {
            buffer: Vec::new(),
            pid: pid,
        }
    }
}

impl FileDescriptor for StdOut {
    fn read(&mut self) -> Vec<u8> {
        panic!("unsupported");
    }

    fn readc(&mut self) -> Option<u8> {
        panic!("unsupported");
    }

    fn name(&self) -> String {
        return String::from("<stdout>");
    }

    fn write(&mut self, buf: Vec<u8>) {
        let s = String::from_utf8(buf.clone()).unwrap();
        kprint!("{}", &s);
    }
}

pub fn write_to_focused_process_stdin(c: char) {
    // if there is no focused process - write to screen,
    // probably scheduler is not started yet
    let fpid = focus::get_focused_pid();
    if fpid == 0 {
        kprint!("{}", c);
        return;
    }
    unsafe {
        let mut process = scheduler::SCHEDULER.as_mut().unwrap().get_task_mut(fpid);
        if process.is_some() {
            let mut fd = process.as_mut().unwrap().file_descriptors.get_mut(&0);
            fd.as_mut().unwrap().write(vec![c as u8]); // TODO change 0
        }
    }
}
