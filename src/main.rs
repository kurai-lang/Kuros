use std::{env, path::PathBuf, process::Command};
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
        args: Vec<String>,

        #[arg(long, action)]
        show_output_files: bool,
    }
}

fn main() {
    let cli = Cli::parse();
    let binding = env::current_dir().unwrap();
    let binding_file = binding.file_name().unwrap();
    let project_name = binding_file.to_str().unwrap();

    let home = dirs::home_dir().expect("No home directory found");
    let kuraic_path: PathBuf = home.join(".local/bin/kuraic");

    let output_path = format!("./target/{}", project_name);
    // let kuraic_path_str = kuraic_path.to_str().unwrap();

    match &cli.command {
        Commands::New { name } => {
            println!("Creating a new project");
            init(&name.clone()).unwrap();
        }
        Commands::Run { show_output_files, ..} => {
            let mut command = Command::new(&kuraic_path);
            command
                .arg("./src/main.kurai")
                .arg("-o")
                .arg(&output_path);

            if *show_output_files {
                command.arg("--show-output-files");
            }

            let status = command
                .status()
                .unwrap();

            if !status.success() {
                eprintln!("{}: Dawg it failed", "error".red().bold());
            }
        }
        _ => eprintln!("You entered nothing relevant..?\nTry `kuros new project_name`"),
    }
}
