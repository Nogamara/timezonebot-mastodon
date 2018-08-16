# TimezoneBot - because Time Zones are hard

## why?

This bot will parse some time formats if you @ mention it on Mastodon and reply with a few conversions.

Examples:

```
starting at 9:30 cet
9:30 CET is 00:09 PST / 02:09 CST / 03:09 EST

starting at 20:30 msk
20:30 MSK is 10:09 PST / 12:09 CST / 13:09 EST / 19:09 CET

starting at 9pm pst
9:00PM PST is 23:09 CST / 00:09 EST / 06:09 CET
```

## howto

```
cargo build --release
./target/release/timezonebot
```

It will ask for permission on the first run.

## stuff

  * uses [mammut](https://crates.io/crate/mammut) for the heavy lifting
