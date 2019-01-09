use crate::run::run;
use crate::verify::verify;
use clap::{crate_version, App, Arg, SubCommand};
use notify::DebouncedEvent;
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use std::io::BufRead;
use std::sync::mpsc::channel;
use std::time::Duration;
use syntect::easy::HighlightFile;
use syntect::highlighting::{Style, ThemeSet};
use syntect::parsing::SyntaxSet;
use syntect::util::as_24_bit_terminal_escaped;

mod run;
mod util;
mod verify;

fn main() {
    let matches = App::new("rustlings")
        .version(crate_version!())
        .author("Olivia Hugger")
        .about("Test")
        .subcommand(SubCommand::with_name("verify").alias("v"))
        .subcommand(SubCommand::with_name("watch").alias("w"))
        .subcommand(
            SubCommand::with_name("run")
                .alias("r")
                .arg(Arg::with_name("file").required(true).index(1))
                .arg(Arg::with_name("test").short("t").long("test")),
        )
        .get_matches();

    let ss = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();

    if None == matches.subcommand_name() {
        println!("");
        println!(r#"       welcome to...                      "#);
        println!(r#"                 _   _ _                  "#);
        println!(r#"  _ __ _   _ ___| |_| (_)_ __   __ _ ___  "#);
        println!(r#" | '__| | | / __| __| | | '_ \ / _` / __| "#);
        println!(r#" | |  | |_| \__ \ |_| | | | | | (_| \__ \ "#);
        println!(r#" |_|   \__,_|___/\__|_|_|_| |_|\__, |___/ "#);
        println!(r#"                               |___/      "#);
        println!("");
    }

    if let Some(matches) = matches.subcommand_matches("run") {
        run(matches.clone());
    }

    if let Some(_) = matches.subcommand_matches("verify") {
        match verify() {
            Ok(_) => {}
            Err(_) => std::process::exit(1),
        }
    }

    if let Some(_) = matches.subcommand_matches("watch") {
        watch().unwrap();
    }

    if let None = matches.subcommand_name() {
        let mut highlighter =
            HighlightFile::new("default_out.md", &ss, &ts.themes["base16-eighties.dark"]).unwrap();
        for maybe_line in highlighter.reader.lines() {
            let line = maybe_line.unwrap();
            let regions: Vec<(Style, &str)> = highlighter.highlight_lines.highlight(&line, &ss);
            println!("{}", as_24_bit_terminal_escaped(&regions[..], true));
        }
    }

    println!("\x1b[0m");
}

fn watch() -> notify::Result<()> {
    let (tx, rx) = channel();

    let mut watcher: RecommendedWatcher = Watcher::new(tx, Duration::from_secs(2))?;
    watcher.watch("./exercises", RecursiveMode::Recursive)?;

    let _ignored = verify();

    loop {
        match rx.recv() {
            Ok(event) => match event {
                DebouncedEvent::Chmod(_) | DebouncedEvent::Write(_) => {
                    let _ignored = verify();
                }
                _ => {}
            },
            Err(e) => println!("watch error: {:?}", e),
        }
    }
}
