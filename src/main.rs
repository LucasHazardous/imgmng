mod directory_scanner;
use std::fs;

fn main() {
    if let Err(_) = fs::create_dir_all("modified") {
        println!("Unable to create modified directory.");
    } else {
        imgmng::run(&directory_scanner::find_images("./"));
    }
}
