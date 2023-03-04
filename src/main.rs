use std::{env, fs::{self, OpenOptions}, io::{stdout, Write}, path::Path, process::exit, thread, time::Duration};

use chrono::Local;
use clearscreen::clear;
use colored::Colorize;
use serde_json::json;

fn main() {
    let mut stdout = stdout();
    let mut round = 1;
    dotenv::dotenv().ok();

    let vanity_urls = get_vanity_urls();
    if vanity_urls.is_empty() {
        log("No urls found!");
        exit(1);
    }

    loop {
        let mut results = String::new();

        for (index, url) in vanity_urls.iter().enumerate() {
            if check_vanity_url(&url) {
                results.push_str(&format!(" {}", url.green()));

                if set_vanity_url(&url) {
                    log(&format!("Succeeded setting Vanity URL to {}\n", url));

                    exit(0);
                } else {
                    log(&format!("Failed setting Vanity URL to {}\n", url));
                }
            } else {
                results.push_str(&format!(" {}", url.red()));
            }

            print!("\rChecking... ({}/{}) [round: {}] [mode: ∞]{}", index + 1, vanity_urls.len(), round, results);
            stdout.flush().unwrap();
        }
        round += 1;


        //wait 1 second then clear and start again
        println!("\nWaiting 1 second...");
        thread::sleep(Duration::from_secs(1));
        clear().unwrap();
    }
}

fn get_vanity_urls() -> Vec<String> {
    let mut urls = Vec::new();

    if !Path::new("urls.txt").exists() {
        println!("File does not exist");
        exit(1);
    }

    fs::read_to_string("urls.txt")
        .expect("Something went wrong reading the file")
        .lines()
        .for_each(|line| urls.push(line.to_string()));

    urls
}

#[tokio::main]
async fn check_vanity_url(url: &str) -> bool {
    let mut res = surf::get(format!("https://discord.com/invite/{}", url)).await.unwrap();
    let text = res.body_string().await.unwrap();

    //this text only exists on invalid discord invite pages, which means its not taken
    text.contains("<meta name=\"twitter:creator\" content=\"@discord\" />")
}

#[tokio::main]
async fn set_vanity_url(url: &str) -> bool {
    let res = surf::patch(format!("https://discord.com/api/v10/guilds/{}/vanity-url", env::var("GUILD_ID").unwrap()))
        .header("Authorization", env::var("TOKEN").unwrap())
        .body(json!({ "code": &url }))
        .send()
        .await.unwrap();

    res.status().is_success()
}

fn log(text: &str) {
    //log to file
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open("log.txt")
        .unwrap();

    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S%.3f");
    let log_message = format!("[{}] {}", timestamp, text);

    file.write_all(log_message.as_bytes()).unwrap();
}