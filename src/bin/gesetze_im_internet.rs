extern crate clap;
extern crate gesetze_im_internet;
extern crate regex;
extern crate stderrlog;

use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};
use gesetze_im_internet::{Client, Toc};
use regex::Regex;
use futures::future::Future;
use tokio::runtime::Runtime;

fn main() {
    let matches = App::new("lawapi_de")
        .setting(AppSettings::ArgRequiredElseHelp)
        .arg(
            Arg::with_name("verbosity")
                .short("v")
                .long("verbose")
                .multiple(true)
                .help("Increase message verbosity"),
        )
        .arg(
            Arg::with_name("quiet")
                .short("q")
                .long("quiet")
                .help("Silence all output"),
        )
        .subcommand(
            SubCommand::with_name("list")
                .about("lists laws by fetching the table of contents")
                .arg(
                    Arg::with_name("search")
                        .short("s")
                        .long("search")
                        .help("searches for a string")
                        .value_name("REGEX")
                        .takes_value(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("get")
                .about("gets a law with the specified short id")
                .arg(
                    Arg::with_name("ID")
                        .help("which law to fetch")
                        .required(true)
                        .index(1),
                ),
        )
        .get_matches();

    // setup logging.
    let verbose = matches.occurrences_of("verbosity") as usize;
    let quiet = matches.is_present("quiet");

    stderrlog::new()
        .module(module_path!())
        .quiet(quiet)
        .verbosity(verbose)
        .init()
        .unwrap();

    // run subcommands.
    match matches.subcommand() {
        ("list", a) => list(a),
        ("get", a) => get(a),
        (a, b) => println!("{} {:?}", a, b),
    }
}

fn list(matches: Option<&ArgMatches>) {
    let mut rt = Runtime::new().unwrap();
    let client = Client::default();
    let task = client
        .get_toc()
        .map(|toc| {
            /*
            let regex = if let Some(search) = matches.unwrap().value_of("search") {
                Regex::new(search).ok()
            } else {
                None
            };
            */
            let regex: Option<Regex> = None;

            for item in toc.items {
                if regex
                    .as_ref()
                    .map(|r| r.is_match(&item.title))
                    .unwrap_or(true)
                {
                    println!("[{}] {}", item.short().unwrap_or("???"), item.title);
                }
            }
        })
        .map_err(|err| {});

    let res = rt.block_on(task);
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
        }
        Err(e) => println!("{:?}", e),
    }
}
