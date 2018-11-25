pub struct String {
    buf: [u8; 1024],
}

impl String {
    pub fn new() -> String {
        return String {
            buf: ['\0' as u8; 1024],
        };
    }
}
