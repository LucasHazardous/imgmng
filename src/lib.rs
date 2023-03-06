pub mod directory_scanner;
pub mod image_modifier;

use colored::*;

pub struct Modifications {
    resize: f32,
}

impl Default for Modifications {
    fn default() -> Self {
        Modifications { resize: 1.0 }
    }
}

pub fn run(target_path: &str) {
    let mut imgs = directory_scanner::find_images(target_path);
    banner_and_clear_screen();
    println!("Found {} files.", imgs.len());

    if question("Do you want to work on all images? (Y/n)", "n") {
        if question("Do you want to apply modifications? (y/N)", "y") {
            let modifications = Modifications {
                ..Default::default()
            };
            for filename in imgs {
                println!("Processing {}", filename);
                if let Err(e) = image_modifier::modify_image(&filename, &modifications, target_path)
                {
                    println!("{} | Unable to modify {}", "Fail".red().bold(), filename);
                    println!("{}: {}", "Error".red(), e)
                } else {
                    println!("{} | Modified {}", "Success".green().bold(), filename);
                }
            }
        } else {
            println!("Currently unavailable.");
        }
    } else {
        println!("Currently unavailable.");
    }
}

fn question(content: &str, nondefault: &str) -> bool {
    println!("{}", content);

    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();

    let first_character = line.chars().nth(0).unwrap().to_lowercase().to_string();

    banner_and_clear_screen();

    !(first_character == nondefault)
}

fn banner_and_clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
    println!("{}{}", "img".green().bold(), "mng".cyan().bold());
}
