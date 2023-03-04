use std::{fs, path::Path, process::exit, env, time, thread};
use serde::Serialize;
use serde_json::Value;

#[derive(Serialize)]
struct VanityBody {
    code: String
}

fn main() {
    dotenv::dotenv().ok();

    println!("{:?}", get_vanity_urls());

    loop {
        println!("Checking...");

        for url in get_vanity_urls() {
            if check_vanity_url(&url) {
                println!("✔ {} is a valid vanity url", url);
                // if set_vanity_url(&url).await {
                //     println!("Successfully set vanity url to {}", url);
                // } else {
                //     println!("Failed to set vanity url to {}", url);
                // }
            } else {
                println!("❌ {} is not a valid vanity url", url);
            }
        }

        println!("Done checking");
        println!("Sleeping for 1 second");
        println!();

        thread::sleep(time::Duration::from_secs(1));

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
    let mut res = surf::get(format!("https://discord.com/api/invites/{}", url)).await.unwrap();

    if res.status() == 429 {
        println!("Rate limited, retrying in {} seconds", res.header("Retry-After").unwrap());
        thread::sleep(time::Duration::from_secs(20));
        return false;
    }

    let json: Value = res.body_json().await.unwrap();

    //code coincides with error code (string is the invite, int is an error code)
    json["code"].is_string()
}

#[tokio::main]
async fn set_vanity_url(url: &str) -> bool {
    surf::post(format!("https://discord.com/api/v8/guilds/{}/vanity-url", env::var("GUILD_ID").unwrap()))
        .header("Authorization", format!("Bot {}", env::var("TOKEN").unwrap()))
        .body_json(&VanityBody {
            code: url.to_string()
        })
        .unwrap().recv_string().await.is_ok()
}