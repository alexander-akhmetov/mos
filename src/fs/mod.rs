use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;
use core::fmt;

pub mod tarfs;
#[cfg(test)]
mod test_data;
pub mod utils;
pub mod vfs;

pub trait FileSystem: Send + Sync {
    fn get_file(&self, path: &str) -> Option<Box<dyn FileDescriptor>>;
    fn list_dir(&self, path: &str) -> Vec<Box<dyn FileDescriptor>>;
    fn fs_name(&self) -> &str;
}

impl fmt::Debug for dyn FileSystem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<{}>", self.fs_name())
    }
}

pub trait FileDescriptor {
    /// Represents a ready to read file
    fn read(&mut self) -> Vec<u8>;
    fn readc(&mut self) -> Option<u8>;
    fn name(&self) -> String;
    fn write(&mut self, buf: Vec<u8>);
}

impl fmt::Debug for dyn FileDescriptor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FileDescriptor<name: '{}'>", self.name())
    }
}
