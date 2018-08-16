extern crate mammut;
extern crate regex;
extern crate time;
extern crate toml;
extern crate tzbot;

use std::io;
use std::fs::File;
use std::io::prelude::*;

use mammut::{Data, Mastodon, Registration};
use mammut::apps::{AppBuilder, Scopes};
use mammut::status_builder::{StatusBuilder};
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

fn main() {
    let mastodon = match File::open("mastodon-data.toml") {
        Ok(mut file) => {
            let mut config = String::new();
            file.read_to_string(&mut config).unwrap();
            let data: Data = toml::from_str(&config).unwrap();
            Mastodon::from_data(data)
        },
        Err(_) => register(),
    };

    let you = mastodon.verify_credentials().unwrap();

    log!("{:#?}", you);

    let list = mastodon.notifications().unwrap();

    let re = Regex::new(r"(?i)<a href=.https://botsin.space/@timezonebot.([^>]+)>@<span>timezonebot</span></a></span>(.*)</p>").unwrap();

    for toot in list.initial_items {
        println!("({}) {} @{} {:?}", toot.id, toot.account.acct, toot.account.username, toot.created_at);
        println!(" ");
        match toot.status {
            Some(s) => {
                //println!("{}", s.content);
                for cap in re.captures_iter(&s.content) {
                    //println!("{}", &cap[0]);
                    //println!("{}", &cap[1]);
                    println!("{}", &cap[2]);
                    //let mut x = String::new(&cap[2]).trim();
                    let result = convert(&cap[2]);
                    match result {
                        None => {
                            log!("NOPE");
                        },
                        Some(val) => {
                            let mut rt = String::new().to_owned();
                            rt.push_str("@");
                            rt.push_str(&toot.account.acct);
                            rt.push_str(" \n");
                            rt.push_str(&val);
                            println!("{:?}", rt);
                            let mut reply = StatusBuilder::new(rt);
                            let id = u64::from_str_radix(&toot.id, 10).unwrap();
                            reply.in_reply_to_id = Some(id);
                            let r = mastodon.new_status(reply);
                            match r {
                                Err(f) => println!("Err: {:?}", f),
                                Ok(rr) => println!("OK : {:?}", rr),
                            }
                        },
                    }
                }
            }
            _ => (),
        }
        println!("============================================================");
    }
    let _ = mastodon.clear_notifications();
}

fn register() -> Mastodon {
    let app = AppBuilder {
        client_name: "tzbot",
        redirect_uris: "urn:ietf:wg:oauth:2.0:oob",
        scopes: Scopes::ReadWrite,
        website: Some("https://github.com/nogamara"),
    };

    let mut registration = Registration::new("https://botsin.space");
    registration.register(app).unwrap();;
    let url = registration.authorise().unwrap();

    println!("Click this link to authorize on Mastodon: {}", url);
    println!("Paste the returned authorization code: ");

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let code = input.trim();
    let mastodon = registration.create_access_token(code.to_string()).unwrap();

    // Save app data for using on the next run.
    let toml = toml::to_string(&*mastodon).unwrap();
    let mut file = File::create("mastodon-data.toml").unwrap();
    file.write_all(toml.as_bytes()).unwrap();

    mastodon
}
