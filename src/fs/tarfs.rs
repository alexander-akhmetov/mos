use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;
use core::fmt;
use fs::{utils, FileDescriptor, FileSystem};
use tar;

pub struct TarFS {
    files: Vec<Box<tar::File>>,
}

impl TarFS {
    /// tar file system
    /// a single tar file (ustar formatted) can be mounted as a ro disk
    /// ! supports only flat archives (all files must have names without "/")
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
    fn read(&mut self) -> Vec<u8> {
        Vec::from(self.content.to_vec())
    }

    fn readc(&mut self) -> Option<u8> {
        panic!("unsupported");
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    fn write(&mut self, buf: Vec<u8>) {}
}

#[cfg(test)]
mod test {
    use super::*;
    use fs::test_data;
    use std::println;

    fn get_fs() -> TarFS {
        let archive = tar::Archive::new(test_data::TEST_TAR_ARCHIVE);
        TarFS::new(archive)
    }

    #[test]
    fn test_read_tar_file() {
        let tarfs = get_fs();

        let exp_files_list: Vec<String> =
            vec![String::from("file1.txt"), String::from("file2.txt")];

        let files_list: Vec<String> = tarfs.list_dir("").iter().map(|x| x.name()).collect();
        assert_eq!(files_list, exp_files_list);

        let files_list: Vec<String> = tarfs.list_dir("/").iter().map(|x| x.name()).collect();
        assert_eq!(files_list, exp_files_list);
    }

    #[test]
    fn test_read_tar_file_and_list_root_should_return_empty_list() {
        let tarfs = get_fs();

        let exp_files_list: Vec<String> = vec![];
        let files_list: Vec<String> = tarfs.list_dir("/empty").iter().map(|x| x.name()).collect();

        assert_eq!(files_list, exp_files_list);
    }

    #[test]
    fn test_get_file() {
        let tarfs = get_fs();
        let file = tarfs.get_file("/file1.txt");
        assert_eq!(file.unwrap().name(), "file1.txt");
    }

    #[test]
    fn test_get_file_unknown() {
        let tarfs = get_fs();
        let file = tarfs.get_file("file_unknown.txt");
        assert_eq!(file.is_none(), true);
    }
}
