use crate::exercise::{Exercise, ExerciseList};
use crate::run::run;
use crate::verify::verify;
use clap::{crate_version, App, Arg, SubCommand};
use notify::DebouncedEvent;
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use std::ffi::OsStr;
use std::fs;
use std::path::Path;
use std::process::{Command, Stdio};
use std::sync::mpsc::channel;
use std::time::Duration;

mod exercise;
mod run;
mod verify;

fn main() {
    let matches = App::new("rustlings")
        .version(crate_version!())
        .author("Olivia Hugger, Carol Nichols")
        .about("Rustlings is a collection of small exercises to get you used to writing and reading Rust code")
        .subcommand(SubCommand::with_name("verify").alias("v").about("Verifies all exercises according to the recommended order"))
        .subcommand(SubCommand::with_name("watch").alias("w").about("Reruns `verify` when files were edited"))
        .subcommand(
            SubCommand::with_name("run")
                .alias("r")
                .about("Runs/Tests a single exercise")
                .arg(Arg::with_name("name").required(true).index(1)),
        )
        .subcommand(
            SubCommand::with_name("hint")
                .alias("h")
                .about("Returns a hint for the current exercise")
                .arg(Arg::with_name("name").required(true).index(1)),
        )
        .get_matches();

    if None == matches.subcommand_name() {
        println!();
        println!(r#"       welcome to...                      "#);
        println!(r#"                 _   _ _                  "#);
        println!(r#"  _ __ _   _ ___| |_| (_)_ __   __ _ ___  "#);
        println!(r#" | '__| | | / __| __| | | '_ \ / _` / __| "#);
        println!(r#" | |  | |_| \__ \ |_| | | | | | (_| \__ \ "#);
        println!(r#" |_|   \__,_|___/\__|_|_|_| |_|\__, |___/ "#);
        println!(r#"                               |___/      "#);
        println!();
    }

    if !Path::new("info.toml").exists() {
        println!(
            "{} must be run from the rustlings directory",
            std::env::current_exe().unwrap().to_str().unwrap()
        );
        println!("Try `cd rustlings/`!");
        std::process::exit(1);
    }

    if !rustc_exists() {
        println!("We cannot find `rustc`.");
        println!("Try running `rustc --version` to diagnose your problem.");
        println!("For instructions on how to install Rust, check the README.");
        std::process::exit(1);
    }

    let toml_str = &fs::read_to_string("info.toml").unwrap();
    let exercises = toml::from_str::<ExerciseList>(toml_str).unwrap().exercises;

    if let Some(ref matches) = matches.subcommand_matches("run") {
        let name = matches.value_of("name").unwrap();

        let matching_exercise = |e: &&Exercise| name == e.name;

        let exercise = exercises.iter().find(matching_exercise).unwrap_or_else(|| {
            println!("No exercise found for your given name!");
            std::process::exit(1)
        });

        run(&exercise).unwrap_or_else(|_| std::process::exit(1));
    }

    if let Some(ref matches) = matches.subcommand_matches("hint") {
        let name = matches.value_of("name").unwrap();

        let exercise = exercises
            .iter()
            .find(|e| name == e.name)
            .unwrap_or_else(|| {
                println!("No exercise found for your given name!");
                std::process::exit(1)
            });

        println!("{}", exercise.hint);
    }

    if matches.subcommand_matches("verify").is_some() {
        verify(&exercises).unwrap_or_else(|_| std::process::exit(1));
    }

    if matches.subcommand_matches("watch").is_some() {
        watch(&exercises).unwrap();
    }

    if matches.subcommand_name().is_none() {
        let text = fs::read_to_string("default_out.txt").unwrap();
        println!("{}", text);
    }
}

fn watch(exercises: &[Exercise]) -> notify::Result<()> {
    /* Clears the terminal with an ANSI escape code.
    Works in UNIX and newer Windows terminals. */
    fn clear_screen() {
        println!("\x1Bc");
    }

    let (tx, rx) = channel();

    let mut watcher: RecommendedWatcher = Watcher::new(tx, Duration::from_secs(2))?;
    watcher.watch(Path::new("./exercises"), RecursiveMode::Recursive)?;

    clear_screen();
    let _ignored = verify(exercises.iter());

    loop {
        match rx.recv() {
            Ok(event) => match event {
                DebouncedEvent::Create(b) | DebouncedEvent::Chmod(b) | DebouncedEvent::Write(b) => {
                    if b.extension() == Some(OsStr::new("rs")) && b.exists() {
                        let filepath = b.as_path().canonicalize().unwrap();
                        let pending_exercises = exercises
                            .iter()
                            .skip_while(|e| !filepath.ends_with(&e.path));
                        clear_screen();
                        let _ignored = verify(pending_exercises);
                    }
                }
                _ => {}
            },
            Err(e) => println!("watch error: {:?}", e),
        }
    }
}

fn rustc_exists() -> bool {
    Command::new("rustc")
        .args(&["--version"])
        .stdout(Stdio::null())
        .spawn()
        .and_then(|mut child| child.wait())
        .map(|status| status.success())
        .unwrap_or(false)
}
