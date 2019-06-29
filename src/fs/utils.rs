use alloc::string::String;
use alloc::vec::Vec;

pub fn normalize(path: &str) -> String {
    add_prefix_slash(path)
}

pub fn add_prefix_slash(path: &str) -> String {
    if !path.starts_with("/") {
        String::from("/") + path
    } else {
        String::from(path)
    }
}

pub fn add_trailing_slash(path: &str) -> String {
    if !path.ends_with("/") {
        String::from(path) + "/"
    } else {
        String::from(path)
    }
}

pub fn remove_trailing_slash(path: &str) -> String {
    if path.ends_with("/") {
        String::from(path.trim_end_matches("/"))
    } else {
        String::from(path)
    }
}

pub fn remove_prefix_slash(path: &str) -> String {
    if path.starts_with("/") {
        String::from(path.trim_start_matches("/"))
    } else {
        String::from(path)
    }
}

pub fn remove_prefix(filepath: &str, path: &str) -> String {
    let n_filepath = remove_trailing_slash(&normalize(filepath));
    let n_path = remove_trailing_slash(&normalize(path));
    let result = n_filepath.trim_start_matches(&n_path);
    String::from(add_prefix_slash(result))
}

pub fn is_file_in_root(filename: &str, path: &str) -> bool {
    let (f, _) = get_filename_and_path(&normalize(filename));
    f == add_trailing_slash(&add_prefix_slash(path))
}

pub fn get_filename_and_path(filename: &str) -> (String, String) {
    let n_filename = normalize(filename);
    if n_filename.ends_with("/") {
        return (String::from(filename), String::new());
    }

    let mut splitted_filename: Vec<&str> = (&n_filename).rsplitn(2, "/").collect();
    splitted_filename.reverse();

    (
        add_trailing_slash(&normalize(&splitted_filename[0])),
        String::from(splitted_filename[1]),
    )
}

pub fn get_root_dir(path: &str) -> String {
    let n_path = remove_prefix_slash(&normalize(path));

    let splitted_path: Vec<&str> = (&n_path).splitn(2, "/").collect();

    normalize(&splitted_path[0])
}

#[cfg(test)]
mod test {
    use super::*;
    use std::println;

    #[test]
    fn test_get_filename_and_path() {
        assert_eq!(
            get_filename_and_path("/files/file.txt"),
            (String::from("/files/"), String::from("file.txt")),
        );

        assert_eq!(
            get_filename_and_path("/file"),
            (String::from("/"), String::from("file")),
        );

        assert_eq!(
            get_filename_and_path("file"),
            (String::from("/"), String::from("file")),
        );

        assert_eq!(
            get_filename_and_path("/file/"),
            (String::from("/file/"), String::from("")),
        );
    }

    #[test]
    fn test_normalize() {
        assert_eq!(normalize("files/file.txt"), "/files/file.txt");
        assert_eq!(normalize("files"), "/files");
        assert_eq!(normalize(""), "/");
        assert_eq!(normalize("/"), "/");
    }

    #[test]
    fn test_is_file_in_root() {
        assert_eq!(is_file_in_root("/", "/"), true);
        assert_eq!(is_file_in_root("/file.txt", "/"), true);

        assert_eq!(is_file_in_root("/files/file1.txt", "files"), true);
        assert_eq!(is_file_in_root("files/file1.txt", "files"), true);
    }

    #[test]
    fn test_is_file_not_in_root() {
        assert_eq!(is_file_in_root("/", "/dir/"), false);
        assert_eq!(is_file_in_root("/file/anotherfile.txt", "/"), false);
        assert_eq!(is_file_in_root("/file/anotherfile.txt/", "/"), false);
    }

    #[test]
    fn test_remove_prefix() {
        assert_eq!(remove_prefix("/etc/nginx", "/etc/nginx/"), "/");
        assert_eq!(
            remove_prefix("/etc/nginx/nginx.conf", "/etc"),
            "/nginx/nginx.conf"
        );

        assert_eq!(remove_prefix("/nginx.conf", "/"), "/nginx.conf");
        assert_eq!(remove_prefix("/initrd/hello.bin", "/initrd"), "/hello.bin");
    }

    #[test]
    fn test_get_root_dir() {
        assert_eq!(get_root_dir("/"), "/");
        assert_eq!(get_root_dir("/initrd/file1.txt"), "/initrd");
        assert_eq!(get_root_dir("/initrd/"), "/initrd");
        assert_eq!(get_root_dir(""), "/");
        assert_eq!(get_root_dir("/etc/nginx/conf.d/default.conf"), "/etc");
    }
}
