use alloc::string::String;

pub fn get_pwd() -> String {
    /// returns current working directory
    // todo: error check
    unsafe {
        let mut buf: [u8; 255] = ['\0' as u8; 255];
        librust::syscall::getcwd(&mut buf);

        read_str(&buf)
    }
}

pub fn read_str(buf: &[u8]) -> String {
    /// reads string from byte array (null-terminated)
    let mut len = 0;
    for e in buf {
        if *e != '\0' as u8 {
            len += 1;
        } else {
            break;
        }
    }

    unsafe {
        String::from_utf8_unchecked(buf[0..len].to_vec())
    }
}
