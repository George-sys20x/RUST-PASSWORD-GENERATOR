use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use rand::{thread_rng, Rng};
use std::iter;

fn main() {
    let mut password_map = HashMap::new();

    let file = File::open("passwords.txt").expect("Failed to open file");

    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let parts: Vec<&str> = line.split(':').collect();
        let username = parts[0].to_string();
        let password = parts[1];
        password_map.insert(username, password.to_string());
    }

    let new_password = generate_complex_password(&password_map);

    password_map.insert("new_password".to_string(), new_password.clone());

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("new_passwords.txt")
        .expect("Failed to open file");

    write!(file, "{}:{}\n", "new_password", new_password).expect("Failed to write to file");
}

fn generate_complex_password(password_map: &HashMap<String, String>) -> String {
    let mut new_password = String::new();
    for (_, password) in password_map {
        new_password.push_str(password);
    }

    let symbols = "!#$%&*+-/:<=>?@[\\]^_`{|}~";


    let mut rng = thread_rng();
    for _ in 0..5 {
        let index = rng.gen_range(0, symbols.len());
        let symbol = symbols.chars().nth(index).unwrap();

        new_password.insert(rng.gen_range(0, new_password.len()), symbol);
    }

    new_password
}

