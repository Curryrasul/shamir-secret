use clap::{Arg, App};
use std::collections::hash_map::Entry::Occupied;

mod split;
mod recover;

fn main() {
    let mut matches = App::new("Shamir's Secret Sharing")
        .arg(Arg::with_name("split")
             .long("split")
             .takes_value(false)
             .help("Start program in split mode"))
        .arg(Arg::with_name("recover")
             .long("recover")
             .takes_value(false)
             .help("Start program in recover mode"))
        .get_matches();

    if matches.args.len() > 1 {
        panic!("Too much args");
    }

    if let Occupied(_) = matches.args.entry("split") {
        split::split();
        return
    }

    if let Occupied(_) = matches.args.entry("recover") {
        recover::recover();
        return 
    }

    panic!("Not enough args!");
}
