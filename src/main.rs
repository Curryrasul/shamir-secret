mod utils;

use clap::{App, Arg};
use std::collections::hash_map::Entry::Occupied;

use shamir_secret::recover;
use shamir_secret::split;

use utils::parse_recover;
use utils::parse_split;

fn main() {
    let mut matches = App::new("Shamir's Secret Sharing")
        .arg(
            Arg::with_name("split")
                .long("split")
                .takes_value(false)
                .help("Start program in split mode"),
        )
        .arg(
            Arg::with_name("recover")
                .long("recover")
                .takes_value(false)
                .help("Start program in recover mode"),
        )
        .get_matches();

    if matches.args.len() > 1 {
        panic!("Too much args");
    }

    if let Occupied(_) = matches.args.entry("split") {
        let (key, n, t) = parse_split();
        let res = split(key, n, t);

        for (index, key_part) in res.into_iter().enumerate() {
            println!("{}  {:x}", index + 1, key_part);
        }
    } else if let Occupied(_) = matches.args.entry("recover") {
        let (t, v) = parse_recover();
        let res = recover(t, v);

        println!("Key = {:x}", res);
    } else {
        panic!("Not enough args!");
    }
}
