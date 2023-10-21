use ureq::Error;
use serde::Deserialize;
use std::io;
use std::process::Command;

#[derive(Deserialize,Debug)]
struct TwitchChannel {
    channel: String,
    title: String,
}

fn main() -> Result<(), Error> {
    let subscriptions: Vec<TwitchChannel> = ureq::get("https://media.monkeyinsight.com/twitch")
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
