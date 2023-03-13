mod directory_scanner;
mod image_modifier;
mod printer;
use printer::*;

use std::error::Error;
use std::fmt;

pub struct Modifications {
    resize: f32,
    invert: bool,
    special: bool,
}

impl Default for Modifications {
    fn default() -> Self {
        Modifications {
            resize: 1.0,
            invert: false,
            special: false,
        }
    }
}

impl fmt::Display for Modifications {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Resize: {} | Invert: {} | Special: {}",
            self.resize,
            if self.invert { "Yes" } else { "No" },
            if self.special { "Yes" } else { "No" }
        )
    }
}

pub fn run(target_path: &str, imgs: Vec<String>) {
    let custom_img_list = imgs.len() > 0;
    let mut imgs = imgs;

    if !custom_img_list {
        imgs = directory_scanner::find_images(target_path);
    }

    banner_and_clear_screen();
    println!("Found {} files.", imgs.len());

    if !custom_img_list && !question("Do you want to work on all images? (Y/n)", "n") {
        banner_and_clear_screen();

        choose_files(&mut imgs);
        banner_and_clear_screen();
    }

    let modifications;
    if question("Do you want to apply modifications? (y/N)", "y") {
        modifications = Modifications {
            ..Default::default()
        };
    } else {
        let resize = match ask_for_modification_value("Enter resize (from 1 to 100):") {
            Ok(x) => {
                if x >= 1.0 && x <= 100.0 {
                    x / 100.0
                } else {
                    0.01
                }
            }
            Err(_e) => 1.0,
        };

        let invert = !question("Invert colors? (y/N)", "y");

        let special = !question("Special effect? (y/N)", "y");

        modifications = Modifications {
            resize,
            invert,
            special,
        };
    }

    println!("{}", modifications);
    modify_files(&imgs, target_path, &modifications);
}

fn modify_files(imgs: &Vec<String>, target_path: &str, modifications: &Modifications) {
    for filename in imgs {
        println!("Processing {}", filename);
        let res = image_modifier::modify_image(&filename, &modifications, target_path);
        print_processed_filename(res, filename);
    }
}

fn ask_for_modification_value(content: &str) -> Result<f32, Box<dyn Error>> {
    println!("{}", content);

    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;

    banner_and_clear_screen();

    let res = input.trim().parse::<f32>()?;

    Ok(res)
}

fn question(content: &str, nondefault: &str) -> bool {
    println!("{}", content);

    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();

    let first_character = line.chars().nth(0).unwrap().to_lowercase().to_string();

    banner_and_clear_screen();

    !(first_character == nondefault)
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
