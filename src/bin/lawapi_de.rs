extern crate clap;
extern crate lawapi_de;

use clap::{App, AppSettings, ArgMatches, SubCommand};
use lawapi_de::gesetz::{Toc, TocItem};

fn main() {
    let matches = App::new("lawapi_de")
        .setting(AppSettings::ArgRequiredElseHelp)
        .subcommand(
            SubCommand::with_name("list")
                .about("lists laws by fetching the table of contents"),
        )
        .subcommand(
            SubCommand::with_name("get")
                .about("gets a law with the specified short id"),
        )
        .get_matches();

    match matches.subcommand() {
        ("list", a) => list(a),
        ("get", a) => get(a),
        (a, b) => println!("{} {:?}", a, b),
    }
}

fn list(matches: Option<&ArgMatches>) {
    Toc::fetch()
        .map(|toc|
             for item in toc.items {
                 println!("[{}] {}", item.short().unwrap_or("???"), item.title);
             })
        .map_err(|e| println!("{:?}", e));
}

fn get(matches: Option<&ArgMatches>) {
    let toc = Toc::fetch();

    match toc {
        Ok(toc) => {
            ()
        },
        Err(e) => println!("{:?}", e)
    }
}
