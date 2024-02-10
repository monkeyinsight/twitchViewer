use ureq::Error;
use serde::Deserialize;
use std::io;
use std::process::Command;
use clap::Parser;
use dotenv::dotenv;

#[derive(Deserialize,Debug)]
struct TwitchChannel {
    channel: String,
    title: String,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    add: Option<String>,

    #[arg(short, long)]
    remove: Option<String>,
}

fn main() -> Result<(), Error> {
    dotenv().ok();

    let key = std::env::var("KEY").expect("didn't provide key");

    let cli = Args::parse();

    if let Some(add) = cli.add.as_deref() {
        let response = ureq::post("https://media.monkeyinsight.com/twitch")
            .set("Auth-token", &key)
            .send_json(ureq::json!({"channel":add}))?;
        println!("{:?}", response);
        return Ok(());
    }

    if let Some(remove) = cli.remove.as_deref() {
        let response = ureq::delete("https://media.monkeyinsight.com/twitch")
            .set("Auth-token", &key)
            .send_json(ureq::json!({"channel":remove}))?;
        println!("{:?}", response);
        return Ok(());
    }

    let subscriptions: Vec<TwitchChannel> = ureq::get("https://media.monkeyinsight.com/twitch")
        .set("Auth-token", &key)
        .call()?
        .into_json()?;

    if subscriptions.len() == 0 {
        panic!("No one is streaming");
    }
    let mut i = 0;
    for subscription in subscriptions.iter() {
        i += 1;
        println!("{}: {} is streaming {}", i, subscription.channel, subscription.title)
    }

    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .unwrap();

        let input: usize = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {0},
        };

        if subscriptions.len() > input - 1 && input > 0 {
            Command::new("streamlink")
                .arg("-Q")
                .arg(format!("https://twitch.tv/{}", subscriptions[input-1].channel))
                .arg("best")
                .spawn()
                .expect("Failed to open streamlink");

            break;
        }
    }
    Ok(())
}
