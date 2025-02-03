use std::fs::File;

use chrono::NaiveDate;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct DiscordNotifications {
    message: String,
    webhook_url: String,
    since: NaiveDate,
}

fn main() {
    let mut file = File::open("example.json");
    if file.is_err() {
        panic!("Error opening example.json!");
    }
    let data: Vec<DiscordNotifications> = serde_json::from_reader(file.unwrap()).expect("Not valid json");
    for item in data {
        println!("{}", item.message);
    }
}
