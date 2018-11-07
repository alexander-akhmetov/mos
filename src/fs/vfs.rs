use alloc::boxed::Box;
use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;
use fs::utils;

impl VirtualFS {
    fn new() -> VirtualFS {
        VirtualFS {
            mountpoints: BTreeMap::new(),
        }
    }

    pub fn mount(&mut self, mountpoint: &str, fs: Box<Filesystem>) {
        self.mountpoints.insert(utils::normalize(mountpoint), fs);
    }

    pub fn get_file(&self, path: &str) -> Option<Box<FileDescriptor>> {
        let fs = self.get_fs(path);
        if let Some(fs) = fs {
            fs.get_file(path)
        } else {
            None
        }
    }

    fn get_fs(&self, path: &str) -> Option<&Box<Filesystem>> {
        // keys returns an iterator over the keys of the map, in sorted order
        // so we iterate in reversed order to find the most longest correspoinding
        // path first
        for key in self.mountpoints.keys().rev() {
            if path.starts_with(key) {
                return self.mountpoints.get(key);
            }
        }

        None
    }
}

pub trait Filesystem {
    fn get_file(&self, path: &str) -> Option<Box<FileDescriptor>>;
}

pub trait FileDescriptor {
    fn read(&mut self) -> Vec<u8>;
}

pub struct VirtualFS {
    mountpoints: BTreeMap<String, Box<Filesystem>>,
}
