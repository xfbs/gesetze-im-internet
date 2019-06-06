use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};
use gesetze_im_internet::{Client, Toc};
use regex::Regex;
use futures::future::Future;
use tokio::runtime::Runtime;
use error_chain::error_chain;

error_chain! {
    foreign_links {
        LoggerError(log::SetLoggerError);
        IOError(std::io::Error);
    }

    errors {
        NoArgMatches(s: String) {}
    }
}

pub struct CLI<'a, 'b> where 'a: 'b {
    app: App<'a, 'b>
}

fn main() {
    CLI::new().run().unwrap();
}

impl<'a, 'b> CLI<'a, 'b> {
    pub fn new() -> CLI<'a, 'b> {
        let app = App::new("gesetze-im-internet")
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
            );

        CLI { app }
    }

    pub fn run(self) -> Result<()> {
        let matches = self.app.get_matches();

        // setup logging.
        let verbose = matches.occurrences_of("verbosity") as usize;
        let quiet = matches.is_present("quiet");

        stderrlog::new()
            .module(module_path!())
            .quiet(quiet)
            .verbosity(verbose)
            .init()?;

        // create runtime.
        let mut rt = Runtime::new()?;

        // run subcommands.
        match matches.subcommand() {
            ("list", a) => Self::list(&mut rt, a.ok_or(ErrorKind::NoArgMatches("list".into()))?),
            ("get", a) => Self::get(&mut rt, a.ok_or(ErrorKind::NoArgMatches("get".into()))?),
            (a, b) => Ok(()),
        }
    }

    fn list(rt: &mut Runtime, matches: &ArgMatches) -> Result<()> {
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

        Ok(())
    }

    fn get(_rt: &mut Runtime, matches: &ArgMatches) -> Result<()> {
        let toc = Toc::fetch();
        let name = matches.value_of("ID").ok_or(ErrorKind::NoArgMatches("ID".into()))?;

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

        Ok(())
    }
}
