use alloc::string::String;

pub fn normalize(path: &str) -> String {
    let mut normalized_path = String::from(path);
    if !path.ends_with("/") {
        normalized_path += "/";
    }
    if !path.starts_with("/") {
        normalized_path = String::from("/") + &normalized_path;
    }
    normalized_path
}
