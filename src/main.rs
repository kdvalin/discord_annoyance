use std::fs::File;
use std::env;

use chrono::NaiveDate;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct DiscordNotifications {
    message: String,
    webhook_url: String,
    since: NaiveDate,
}

fn main() {
    let filename = match env::args().nth(1) {
        Some(t) => t,
        None => panic!("Usage: discord_notifications <JSON filename>")
    };
    let file = File::open(&filename);
    if file.is_err() {
        let error = file.unwrap_err();
        panic!("Error opening {}!  {}", filename, error);
    }
    let data: Vec<DiscordNotifications> = serde_json::from_reader(file.unwrap()).expect("Not valid json");
    let today = chrono::offset::Utc::now().date_naive();
    for item in data {
        let date_diff = today - item.since;
        let req = reqwest::blocking::Client::new()
            .post(item.webhook_url).body( 
                format!("{{\"content\": \"{}\"}}", item.message.replace("$DATE", date_diff.num_days().to_string().as_str()))
            ).header("Content-Type", "application/json").send().unwrap();
        println!("{}: {}", req.status(), req.text().unwrap());
    }
}
