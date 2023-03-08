use std::error::Error;

use colored::*;

pub fn print_available_files_with_selection(
    imgs: &Vec<String>,
    included: &Vec<bool>,
    selected: &usize,
) {
    for i in 0..imgs.len() {
        if selected == &i {
            println!("{}", imgs[i].cyan().bold());
        } else if included[i] {
            println!("{}", imgs[i].green());
        } else {
            println!("{}", imgs[i]);
        }
    }
}

pub fn banner_and_clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
    println!("{}{}", "img".green().bold(), "mng".cyan().bold());
}

pub fn print_processed_filename(res: Result<(), Box<dyn Error>>, filename: &str) {
    match res {
        Err(e) => {
            println!("{} | Unable to modify {}", "Fail".red().bold(), filename);
            println!("{}: {}", "Error".red(), e);
        }
        _ => {
            println!("{} | Modified {}", "Success".green().bold(), filename);
        }
    }
}
