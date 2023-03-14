use std::fs;

const ACCEPTED_FILE_FORMATS: [&str; 3] = ["jpeg", "jpg", "png"];

pub fn find_images(path: &str) -> Vec<String> {
    let paths = fs::read_dir(path).unwrap();
    let mut res = Vec::new();

    for path in paths {
        let filename = path.unwrap().file_name();
        let string_filename = filename.to_str().unwrap();
        if is_acceptable_format(string_filename) {
            res.push(String::from(string_filename));
        }
    }

    res
}

fn is_acceptable_format(filename: &str) -> bool {
    for format in ACCEPTED_FILE_FORMATS {
        if str::ends_with(filename, format) {
            return true;
        }
    }

    false
}
