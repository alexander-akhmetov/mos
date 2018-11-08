use alloc::boxed::Box;
use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;
use core::fmt;
use fs::utils;
use fs::vfs::{FileDescriptor, FileSystem};
use tar;

pub struct TarFS {
    files: Vec<Box<tar::File>>,
}

impl TarFS {
    /// tar file system
    /// a single tar file (ustar formatted) can be mounted as a ro disk
    pub fn new(archive: tar::Archive) -> TarFS {
        let mut files = Vec::new();

        for f in archive.files() {
            files.push(Box::new(f));
        }
        TarFS { files: files }
    }
}

impl FileSystem for TarFS {
    fn fs_name(&self) -> &str {
        return "TarFS";
    }

    fn get_file(&self, path: &str) -> Option<Box<FileDescriptor>> {
        for f in self.files.iter() {
            if utils::normalize(&f.name()) == utils::normalize(path) {
                return Some(Box::new(TarFileDescriptor::new(f)));
            }
        }

        None
    }

    fn list_dir(&self, path: &str) -> Vec<Box<FileDescriptor>> {
        // todo: no such directory
        let mut files: Vec<Box<FileDescriptor>> = Vec::new();
        for f in self.files.iter() {
            if utils::is_file_in_root(&f.name(), path) {
                files.push(Box::new(TarFileDescriptor::new(f)));
            }
        }
        files
    }
}

pub struct TarFSIterator<'a> {
    index: usize,
    fs: &'a TarFS,
}

impl<'a> Iterator for TarFSIterator<'a> {
    type Item = TarFileDescriptor;

    fn next(&mut self) -> Option<TarFileDescriptor> {
        None
    }
}

pub struct TarFileDescriptor {
    name: String,
    content: Vec<u8>,
}

impl TarFileDescriptor {
    fn new(file: &tar::File) -> TarFileDescriptor {
        TarFileDescriptor {
            name: file.name(),
            content: file.content.to_vec(),
        }
    }
}

impl FileDescriptor for TarFileDescriptor {
    fn read(&self) -> Vec<u8> {
        Vec::from(self.content.to_vec())
    }

    fn name(&self) -> String {
        self.name.clone()
    }
}

impl TarFileDescriptor {
    fn is_in_root(&self, path: &str) -> bool {
        true
    }
}
