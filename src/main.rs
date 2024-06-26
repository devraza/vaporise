use anyhow::{Context, Result};
use clap::Parser;
use owo_colors::OwoColorize;
use std::{fs, path, process};
use std::{io, io::Write};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None, author)]
/// A simple, featureful and blazingly fast memory-safe alternative to 'rm' written in Rust.
struct Args {
    /// Ask once before removing all
    #[arg(short, long)]
    ask_once: bool,

    /// Ask before removing each target
    #[arg(short = 'x', long, conflicts_with = "ask_once")]
    ask_each: bool,

    #[arg(trailing_var_arg = true, allow_hyphen_values = false)]
    targets: Vec<String>,
}

fn confirm_parse() {
    io::stdout().flush().unwrap();

    let mut confirm = String::new();
    io::stdin()
        .read_line(&mut confirm)
        .expect("failed to read input");

    if confirm != "y\n" {
        process::exit(0);
    }
}

fn confirm_once() {
    print!("Delete the specified targets? [y/N]: ");
    confirm_parse();
}

fn confirm_each(target: &String) {
    print!("Delete {} ? [y/N]: ", target.bold());
    confirm_parse();
}

fn vaporise() -> Result<()> {
    let args = Args::parse();

    if args.targets.is_empty() {
        eprintln!("{} no arguments passed", "error:".red().bold());
        eprintln!(
            "{} try 'vpr -h' for more information",
            "note:".cyan().bold()
        );
        process::exit(0);
    }

    if args.ask_once {
        println!();
        for target in args.targets.iter() {
            println!("  {}", target.bold());
        }
        println!();
        confirm_once();
    }

    for target in args.targets.iter() {
        if args.ask_each {
            confirm_each(target);
        }

        if path::Path::new(target).exists() {
            if fs::metadata(target).unwrap().is_dir() {
                fs::remove_dir_all(target)
                    .with_context(|| format!("could not remove directory: {}", target.bold()))?;
            } else {
                fs::remove_file(target)
                    .with_context(|| format!("could not remove file: {}", target.bold()))?;
            }
        } else {
            eprintln!(
                "{} the specified target does not exist: {}",
                "error:".red().bold(),
                target.yellow()
            );
        }
    }

    Ok(())
}

fn main() {
    if let Err(error) = vaporise() {
        eprintln!("{} {:?}", "error:".red().bold(), error);
    }
}
