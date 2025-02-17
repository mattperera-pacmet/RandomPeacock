use rand::Rng;
use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};

#[derive(Serialize, Deserialize)]
struct ColorData {
    colors: Vec<String>,
}

fn read_colors_from_file(filename: &str) -> ColorData {
    if let Ok(mut file) = File::open(filename) {
        let mut contents = String::new();
        if file.read_to_string(&mut contents).is_ok() {
            if let Ok(data) = serde_json::from_str(&contents) {
                return data;
            }
        }
    }
    ColorData { colors: Vec::new() }
}

fn save_colors_to_file(filename: &str, data: &ColorData) {
    if let Ok(mut file) = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(filename)
    {
        let _ = file.write_all(serde_json::to_string(data).unwrap().as_bytes());
    }
}

fn generate_color(exclude: &Vec<String>) -> String {
    let mut rng = rand::thread_rng();
    loop {
        let color = format!("#{:06X}", rng.gen_range(0..=0xFFFFFF));
        if !exclude.contains(&color) {
            return color;
        }
    }
}

fn main() {
    let filename = "colors.json";
    let mut color_data = read_colors_from_file(filename);
    let new_color = generate_color(&color_data.colors);

    print!("New random color: {} ", new_color);
    color_data.colors.push(new_color.clone());
    save_colors_to_file(filename, &color_data);

    // Display the color in a big box
    let color_code = &new_color[1..]; // Remove the '#' character
    println!(
        "\x1b[48;2;{};{};{}m{:^3}\x1b[0m",
        u8::from_str_radix(&color_code[0..2], 16).unwrap(),
        u8::from_str_radix(&color_code[2..4], 16).unwrap(),
        u8::from_str_radix(&color_code[4..6], 16).unwrap(),
        " "
    );
}
