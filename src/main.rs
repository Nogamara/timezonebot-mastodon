//use mastodon_async::prelude::*;
use mastodon_async::helpers::toml;
use mastodon_async::{helpers::cli, Language, Mastodon, Registration, Result, StatusBuilder, Visibility};
use regex::Regex;
use tzbot::convert;

macro_rules! log {
    () => (
        println!("{}", now());
    );
    ($x:expr) => (
        print!("{} ", now());
        println!($x);
    );
    ($x:expr, $($arg:tt)*) => (
        print!("{} ", now());
        println!($x, $($arg)*);
    );
}

fn now() -> String {
    let now = time::now();

    match time::strftime("[%Y-%m-%d %H:%M:%S]", &now) {
        Err(_) => "[XXXX-XX-XX xx:xx:xx]".to_string(),
        Ok(s) => s,
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let mastodon = if let Ok(data) = toml::from_file("mastodon-data.toml") {
        Mastodon::from(data)
    } else {
        register().await?
    };

    let you = mastodon.verify_credentials().await?;

    log!("{:#?}", you);

    let list = mastodon.notifications().await?;

    let re = Regex::new(r"(?i)<a href=.https://botsin.space/@timezonebot.([^>]+)>@<span>timezonebot</span></a></span>(.*)</p>").unwrap();

    for toot in list.initial_items {
        println!("--------");
        println!("({}) ({}) {} @{} {:?}", toot.id, toot.account.id, toot.account.acct, toot.account.username, toot.created_at);
        //println!("{:?}", toot);
        println!(" ");
        match toot.status  {
            Some(stat) => {
                for cap in re.captures_iter(&stat.content) {
                    println!("Toot:[{}]", &cap[2]);

                    let result = convert(&cap[2]);
                    match result {
                        None => {
                            log!("No match!");
                        },
                        Some(val) => {
                            let mut rt = String::new().to_owned();
                            rt.push_str("@");
                            rt.push_str(&toot.account.acct);
                            rt.push_str(" \n");
                            rt.push_str(&val);
                            println!("RT:{:?}", rt);
                            let reply = StatusBuilder::new()
                                .status(rt)
                                .visibility(Visibility::Unlisted)
                                .language(Language::Eng)
                                .in_reply_to(stat.id.to_string())
                                .build()?;
                            println!("Reply:{:?}", reply);
                            let status = mastodon.new_status(reply).await?;
                            if let Some(url) = status.url {
                                println!(", visible when logged in at: {}\n", url);
                            } else {
                                println!(". For some reason, the status URL was not returned from the server.");
                                println!("Maybe try here? {}/{}", status.account.url, status.id);
                            }

                        },
                    }
                }
            }
            _ => (),
        }
        println!("============================================================");

        let _ = mastodon.clear_notifications().await?;
    }

    Ok(())
}

async fn register() -> Result<Mastodon> {
    let registration = Registration::new("https://botsin.space")
        .client_name("timezonebot")
        .build()
        .await?;
    let mastodon = cli::authenticate(registration).await?;

    // Save app data for using on the next run.
    toml::to_file(&mastodon.data, "mastodon-data.toml")?;

    Ok(mastodon)
}
