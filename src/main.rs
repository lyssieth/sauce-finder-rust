#[macro_use]
extern crate smart_default;
use clap::{crate_description, App, Arg, ArgMatches};
use std::env;

mod iqdb;
mod util;

fn main() {
    let app = App::new("sauce-finder")
        .version("1.0")
        .author("Rax Ixor <raxixor@rax.ee>")
        .about(crate_description!())
        .arg(
            Arg::with_name("verbose")
                .short('v')
                .long("verbose")
                .takes_value(false)
                .about("Uses verbose mode"),
        )
        .arg(
            Arg::with_name("download")
                .short('d')
                .long("download")
                .takes_value(false)
                .about("Downloads results"),
        )
        .arg(
            Arg::with_name("open")
                .short('o')
                .long("open")
                .takes_value(false)
                .about("Open results in browser"),
        )
        .arg(
            Arg::with_name("json")
                .long("json")
                .takes_value(false)
                .about("Prints output as json; conflicts with --verbose, --download, --open, --no-print")
                .conflicts_with("verbose")
                .conflicts_with("download")
                .conflicts_with("open")
                .conflicts_with("no-print"),
        )
        .arg(
            Arg::with_name("no-print")
                .long("no-print")
                .takes_value(false)
                .about("Doesn't print the full list of matches; use with --download or --open"),
        )
        .arg(
            Arg::with_name("LINK")
                .about("Link to an image to find the sauce for")
                .required(true)
                .index(1),
        );
    let matches = app.get_matches();

    let verbose = matches.is_present("verbose");

    main_args(verbose, matches);
}

fn main_args(verbose: bool, arg: ArgMatches) {
    let url: &str = arg.value_of("LINK").unwrap();
    if verbose {
        println!("Searching for the sauce of {}", url);
    }
    let res = util::build_match(url);

    if let Ok(m) = res {
        if m.found.is_empty() && verbose {
            println!("Found zero results.");
        } else {
            if verbose {
                if m.found.len() == 1 {
                    println!("Found 1 result.");
                } else {
                    println!("Found {} results.", m.found.len());
                }
            }
            let download = arg.is_present("download");
            let open = arg.is_present("open");
            let no_print = arg.is_present("no-print");
            let json = arg.is_present("json");

            if !json {
                handle_no_json(verbose, download, open, no_print, m);
            } else {
                handle_json(m)
            }
        }
    } else if let Err(e) = res {
        println!("Error: {}", e)
    }
}

fn handle_no_json(verbose: bool, download: bool, open: bool, no_print: bool, m: iqdb::Matches) {
    if !no_print {
        println!("Matches: \n{}", m);
    }

    if download {
        util::download_matches(&m, verbose);
    }

    if open {
        util::open_matches(&m, verbose);
    }
}

fn handle_json(m: iqdb::Matches) {
    let res = serde_json::to_string_pretty(&m);

    if let Ok(pretty) = res {
        println!("{}", pretty);
    } else if let Err(e) = res {
        println!("{{\"error\": \"{}\"}}", e);
    }
}
