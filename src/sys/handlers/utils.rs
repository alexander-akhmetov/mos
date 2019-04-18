use core::{slice, str};

pub fn read_str(arg_ptr: u64, length: u64) -> &'static str {
    unsafe {
        let bytes_buf: &[u8] = slice::from_raw_parts(arg_ptr as *const u8, length as usize);

        str::from_utf8_unchecked(bytes_buf)
    }
}
