use std::{env, io, process::Command};
use clap::{Parser, Subcommand};

use colored::Colorize;
use kuros::start::init;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    New {
        name: String,
    },
    Run {
        #[arg()]
        args: Vec<String>
    }
}

fn main() {
    let cli = Cli::parse();
    let binding = env::current_dir().unwrap();
    let binding_file = binding.file_name().unwrap();
    let project_name = binding_file.to_str().unwrap();

    let mut kuraic_path = dirs::home_dir().expect("No home directory found");
    kuraic_path.push(".local/bin/kuraic");

    match &cli.command {
        Commands::New { name } => {
            println!("Creating a new project");
            init(&name.clone()).unwrap();
        }
        Commands::Run { args } => {
            let status = Command::new(kuraic_path)
                .arg("./src/main.kurai")
                .arg("-o")
                .arg(&project_name)
                .status()
                .unwrap();

            if !status.success() {
                eprintln!("{}: Dawg it failed", "error".red().bold());
            }
        }
        _ => println!("You entered nothing relevant..?\nTry `kuros new project_name`"),
    }
}
