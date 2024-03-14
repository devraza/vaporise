use std::{fs, path, process};
use clap::Parser;
use colored::Colorize;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None, author)]
/// A simple, featureful and blazingly fast memory-safe alternative to 'rm' written in Rust.
struct Args {
    /// Don't try to preserve '~' or '/'
    #[arg(long)]
    no_preserve: bool,

    #[arg(trailing_var_arg = true, allow_hyphen_values = false)]
    targets: Vec<String>,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    if args.targets.len() == 0 {
        println!("{} {}", "error:".red().bold(), "no arguents passed");
        println!("{} {}", "note:".cyan().bold(), "try 'vpr -h' for more information");
        process::exit(0);
    }

    for target in args.targets.iter() {
        if args.no_preserve == false {
            if target == "/" || target == "~" {
                println!("{} you're trying to delete an important directory ({})! specify '{}' if you really want to do this", "error:".red().bold(), "--no-preserve".yellow(), target);
                process::exit(0);
            }
        }
        
        if path::Path::new(target).exists() {
            if fs::metadata(target).unwrap().is_dir() {
                let _ = fs::remove_dir_all(target);
            } else {
                let _ = fs::remove_file(target);
            }
        } else {
            println!("{} {} {}", "error:".red().bold(), "the specified target does not exist:", target.yellow());
        }
    }

    Ok(())
}
