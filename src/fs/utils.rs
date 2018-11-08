use alloc::string::String;

pub fn normalize(path: &str) -> String {
    let normalized_path = remove_trailing_slash(path);
    add_prefix_slash(&normalized_path)
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

pub fn remove_prefix(filepath: &str, path: &str) -> String {
    let n_filepath = normalize(filepath);
    let n_path = normalize(path);
    let result = n_filepath.trim_left_matches(&n_path);
    String::from(add_prefix_slash(result))
}

pub fn is_file_in_root(filename: &str, path: &str) -> bool {
    let n_filename = &normalize(filename);
    let n_path = &normalize(path);

    if n_filename.starts_with(n_path) {
        let t_filename = n_filename.trim_start_matches(n_path);
        let t_filename = &t_filename.trim_end_matches("/");
        if !t_filename.contains("/") {
            return true;
        }
    }
    return false;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_is_file_in_root() {
        assert_eq!(is_file_in_root("/", "/"), true);
        assert_eq!(is_file_in_root("/file.txt", "/"), true);
        assert_eq!(is_file_in_root("/file.txt/", "/"), true);
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
}
