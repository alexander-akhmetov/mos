use alloc::boxed::Box;
use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;
use core::fmt;
use fs::utils;
use spin::Mutex;

pub struct VirtualFS {
    /// VirtualFS is an abstraction for the kernel around other FS'.
    /// Any FS which implements necessary traits can be mounted at any path
    /// and then VirtualFS can use it to list/read files
    /// Must be used via VFS mutex-locked instance
    mountpoints: BTreeMap<String, Box<FileSystem>>,
}

impl VirtualFS {
    pub fn new() -> VirtualFS {
        VirtualFS {
            mountpoints: BTreeMap::new(),
        }
    }

    pub fn mount(&mut self, mountpoint: &str, fs: Box<FileSystem>) {
        self.mountpoints.insert(utils::normalize(mountpoint), fs);
        system_log!("VFS: attached new FS at {}", mountpoint);
    }

    pub fn get_file(&self, path: &str) -> Option<Box<FileDescriptor>> {
        let (fs, mountpoint) = self.get_fs(path);
        if let Some(fs) = fs {
            let filepath = utils::remove_prefix(path, &mountpoint);
            let filepath = utils::remove_prefix(&filepath, "/");
            fs.get_file(&filepath)
        } else {
            None
        }
    }

    fn get_fs(&self, path: &str) -> (Option<&Box<FileSystem>>, String) {
        // keys returns an iterator over the keys of the map, in sorted order
        // so we iterate in reversed order to find the most longest correspoinding
        // path first
        let n_path = &utils::normalize(path);
        for key in self.mountpoints.keys().rev() {
            let n_key = utils::normalize(key);
            if n_path.starts_with(&n_key) {
                let fs = self.mountpoints.get(key);
                return (fs, n_key);
            }
        }

        (None, String::new())
    }

    pub fn list_dir(&self, path: &str) -> Vec<Box<FileDescriptor>> {
        // todo: no such directory
        let (fs, mountpoint) = self.get_fs(path);
        if let Some(fs) = fs {
            let dirpath = utils::remove_prefix(path, &mountpoint);
            let s_dirpath = utils::add_prefix_slash(&dirpath);
            fs.list_dir(&s_dirpath)
        } else {
            Vec::new()
        }
    }
}

pub trait FileSystem: Send + Sync {
    fn get_file(&self, path: &str) -> Option<Box<FileDescriptor>>;
    fn list_dir(&self, path: &str) -> Vec<Box<FileDescriptor>>;
    fn fs_name(&self) -> &str;
}

impl fmt::Debug for FileSystem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<{}>", self.fs_name())
    }
}

pub trait FileDescriptor {
    /// Represents a ready to read file
    fn read(&self) -> Vec<u8>;
    fn name(&self) -> String;
}

impl fmt::Debug for FileDescriptor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FileDescriptor<name: '{}'>", self.name())
    }
}

lazy_static! {
    pub static ref VFS: Mutex<VirtualFS> = Mutex::new(VirtualFS::new());
}
