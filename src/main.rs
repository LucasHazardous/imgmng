use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut target_path = String::from("./");
    if args.len() > 1 {
        target_path = args[1].clone();
    }

    if let Err(_) = fs::create_dir_all(target_path.clone() + "modified") {
        println!("Unable to create modified directory.");
    } else {
        imgmng::run(&target_path);
    }
}
