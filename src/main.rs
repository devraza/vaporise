use std::{fs, path, process};
use clap::Parser;
use colored::Colorize;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    recursive: bool,

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
        if path::Path::new(target).exists() {
            if fs::metadata(target).unwrap().is_dir() {
                if args.recursive == false {
                    println!("{} could not remove directory {}, please specify '-r'", "error:".red().bold(), target);
                    process::exit(0);
                } else {
                    let _ = fs::remove_dir_all(target);
                }
            } else {
                let _ = fs::remove_file(target);
            }
        } else {
            println!("{} {} {}", "error:".red().bold(), "the specified target does not exist:", target);
        }
    }

    Ok(())
}
