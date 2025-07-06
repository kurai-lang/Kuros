use std::{fs, io, process::Command};

use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct ProfileConfig {
    #[serde(rename = "opt-level")]
    opt_level: u8,
}

#[derive(Deserialize, Debug)]
struct BuildConfig {
    external_libs: Vec<String>,
    external_libs_paths: Vec<String>,
}

#[derive(Deserialize, Debug)]
struct Config {
    profile: ProfileConfig,
    build: BuildConfig,
}

pub fn manage_toml(command: &mut Command) -> io::Result<()> {
    let content = fs::read_to_string("kuros.toml")?;
    let config: Config = toml::from_str(&content).unwrap();

    println!("{:?}", config);

    // Optimization
    command.arg(format!("-O{}", config.profile.opt_level));

    // Environment for external libraries
    let ld_path = config.build.external_libs_paths.join(":");
    command.env("LD_LIBRARY_PATH", ld_path);

    // Search path for external libraries needed
    for dir in config.build.external_libs_paths {
        command.arg(format!("-L{}", dir));
    }

    // External library names
    for lib in config.build.external_libs {
        command.arg(format!("-l{}", lib));
    }
    Ok(())
}
