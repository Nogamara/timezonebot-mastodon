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

It will ask for permission on the first run and write to `mastodon-data.toml`

## todo

  * the 0.1.x version of the `time` crate is 4 years old and later ones are incompatible

## Changelog

  * 2022-06: Archiving this because mammut hasn't been updated in ages and even the fork elefrens is abandonded.
  * 2023-09: Unarchiving this because it works with mastodon-async. See TODO though

## stuff

  * uses [mastodon-async](https://crates.io/crate/mastodon-async) for the heavy lifting
