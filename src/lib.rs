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
        banner_and_clear_screen();
        choose_files(&mut imgs);
        println!("{:?}", imgs);
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

fn choose_files(imgs: &mut Vec<String>) {
    let mut selected = 0;
    let mut included = vec![false; imgs.len()];

    let mut input = String::new();

    while input.len() == 0 || input.chars().nth(0).unwrap() != 'a' {
        banner_and_clear_screen();
        println!("Select files for modification:");
        input.clear();

        print_available_files_with_selection(imgs, &included, &selected);
        std::io::stdin().read_line(&mut input).unwrap();

        match input.chars().nth(0).unwrap() {
            'e' => {
                included[selected] = !included[selected];
            }
            's' => {
                selected = (selected + 1) % imgs.len();
            }
            'w' => {
                selected = if selected as i32 - 1 < 0 {
                    imgs.len() - 1
                } else {
                    selected - 1
                };
            }
            _ => (),
        }
    }

    let mut true_index = 0;
    for i in 0..imgs.len() {
        if !included[i] {
            imgs.remove(i - true_index);
            true_index += 1;
        }
    }
}

fn print_available_files_with_selection(
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
