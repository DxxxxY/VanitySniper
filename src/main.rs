use std::{env, fs, io::{stdout, Write}, path::Path, process::exit, thread, time::Duration};

use clearscreen::clear;
use colored::Colorize;
use serde::Serialize;

#[derive(Serialize)]
struct VanityBody {
    code: String
}

fn main() {
    let mut stdout = stdout();
    let mut round = 1;
    dotenv::dotenv().ok();

    loop {
        for (index, url) in get_vanity_urls().iter().enumerate() {
            print!("\rChecking... ({}/{}) [round {}]", index + 1, get_vanity_urls().len(), round);
            stdout.flush().unwrap();

            if check_vanity_url(&url) {
                println!(" {}", url.green());

                if set_vanity_url(url) {
                    println!("{} Vanity URL set to {}", "Success".green(), url.green());
                    exit(0);
                } else {
                    println!("{} Vanity URL set to {}", "Failed".red(), url.red());
                }
            } else {
                println!(" {}", url.red());
            }
        }

        round += 1;
        println!("Waiting 1 second...");
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

    !text.contains("<meta name=\"twitter:creator\" content=\"@discord\" />")
}

#[tokio::main]
async fn set_vanity_url(url: &str) -> bool {
    surf::post(format!("https://discord.com/api/v10/guilds/{}/vanity-url", env::var("GUILD_ID").unwrap()))
        .header("Authorization", format!("Bot {}", env::var("TOKEN").unwrap()))
        .body_json(&VanityBody {
            code: url.to_string()
        })
        .unwrap().recv_string().await.is_ok()
}