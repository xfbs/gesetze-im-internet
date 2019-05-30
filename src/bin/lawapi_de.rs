extern crate clap;
extern crate lawapi_de;
extern crate regex;

use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};
use lawapi_de::gesetz::{Toc};
use regex::Regex;

fn main() {
    let matches = App::new("lawapi_de")
        .setting(AppSettings::ArgRequiredElseHelp)
        .subcommand(
            SubCommand::with_name("list")
                .about("lists laws by fetching the table of contents")
                .arg(Arg::with_name("search")
                     .short("s")
                     .long("search")
                     .help("searches for a string")
                     .value_name("REGEX")
                     .takes_value(true))
        )
        .subcommand(
            SubCommand::with_name("get")
                .about("gets a law with the specified short id")
                .arg(Arg::with_name("ID")
                     .help("which law to fetch")
                     .required(true)
                     .index(1)),
        )
        .get_matches();

    match matches.subcommand() {
        ("list", a) => list(a),
        ("get", a) => get(a),
        (a, b) => println!("{} {:?}", a, b),
    }
}

fn list(matches: Option<&ArgMatches>) {
    let toc = Toc::fetch();

    match toc {
        Ok(toc) => {
            let regex = if let Some(search) = matches.unwrap().value_of("search") {
                Regex::new(search).ok()
            } else {
                None
            };

            for item in toc.items {
                if regex.as_ref().map(|r| r.is_match(&item.title)).unwrap_or(true) {
                    println!("[{}] {}", item.short().unwrap_or("???"), item.title);
                }
            }
        },
        Err(e) => println!("{:?}", e),
    }
}

fn get(matches: Option<&ArgMatches>) {
    let toc = Toc::fetch();
    let name = matches.unwrap().value_of("ID").unwrap();

    match toc {
        Ok(toc) => {
            let law = toc.items.iter().find(|ref i| i.short().unwrap() == name);

            if let Some(law) = law {
                match law.fetch() {
                    Ok(law) => println!("{}", law),
                    Err(e) => println!("{:?}", e),
                }
            }
        },
        Err(e) => println!("{:?}", e)
    }
}
