#![recursion_limit = "128"]

use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};
use error_chain::error_chain;
use error_chain::ChainedError;
use futures::future::Future;
use gesetze_im_internet::{Client, Toc, TocItem};
use tokio::runtime::Runtime;

error_chain! {
    links {
        ClientError(gesetze_im_internet::Error, gesetze_im_internet::ErrorKind);
    }

    foreign_links {
        LoggerError(log::SetLoggerError);
        IOError(std::io::Error);
    }

    errors {
        NoArgMatches(s: String) {}
        NoShortTitle(item: TocItem) {}
        ItemNotFound(name: String) {}
    }
}

pub struct CLI<'a, 'b>
where
    'a: 'b,
{
    app: App<'a, 'b>,
}

fn main() {
    let ret: i32 = CLI::new()
        .run()
        .map(|_| 0)
        .or_else(|err| {
            println!("{}", err.display_chain().to_string());

            // FIXME: return value depending on error type.
            Ok(1) as Result<i32>
        })
        .unwrap();

    std::process::exit(ret);
}

impl<'a, 'b> CLI<'a, 'b>
where
    'a: 'b,
{
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
            (_, _) => Ok(()),
        }
    }

    fn list(rt: &mut Runtime, _matches: &ArgMatches) -> Result<()> {
        let client = Client::default();
        let task = client
            .get_toc()
            .map_err(Error::from)
            .map(Self::toc_get_items)
            .and_then(Self::print_toc_items);

        let ret = rt.block_on(task);

        Ok(ret?)
    }

    fn get(rt: &mut Runtime, matches: &ArgMatches) -> Result<()> {
        let client = Client::default();
        let name: String = matches
            .value_of("ID")
            .ok_or(ErrorKind::NoArgMatches("ID".into()))?
            .into();

        let task = client
            .get_toc()
            .map_err(Error::from)
            .map(Self::toc_get_items)
            .and_then(Self::item_find(name))
            .and_then(move |item| client.clone().get_toc_item(&item).map_err(Error::from))
            .map(|item| ());

        let ret = rt.block_on(task);

        Ok(ret?)
    }

    fn item_find(name: String) -> impl FnOnce(Vec<TocItem>) -> Result<TocItem> {
        move |items| {
            items
                .into_iter()
                .find(|i| i.short() == Some(&name.clone()))
                .ok_or(ErrorKind::ItemNotFound(name.clone()).into())
        }
    }

    /*
    fn item_get(client: Client) -> impl FnOnce(TocItem) -> impl Future<Item = String, Error = Error> {
        move |item| client.clone().get_toc_item(&item)
    }
    */

    fn print_toc_items(items: Vec<TocItem>) -> Result<()> {
        // get the length of the longest short title
        let max_short = items
            .iter()
            .map(|item| item.short().map(|s| s.len()).unwrap_or(0))
            .max()
            .unwrap_or(0);

        // FIXME remove unwrap()
        items
            .iter()
            .map(|item| (item.short().unwrap(), &item.title))
            .map(|(short, title)| {
                format!(
                    "{short:>pad$}: {title}",
                    pad = max_short,
                    short = short,
                    title = title
                )
            })
            .for_each(|line| println!("{}", line));

        Ok(())
    }

    fn toc_get_items(toc: Toc) -> Vec<TocItem> {
        toc.items
    }
}
