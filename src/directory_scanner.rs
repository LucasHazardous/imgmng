use std::fs;

pub fn find_images(path: &str) -> Vec<String> {
    let paths = fs::read_dir(path).unwrap();
    let mut res = Vec::new();

    for path in paths {
        let filename = path.unwrap().file_name();
        let string_filename = filename.to_str().unwrap();
        if str::ends_with(string_filename, ".jpg") {
            res.push(String::from(string_filename));
        }
    }

    res
}
