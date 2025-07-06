use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;

pub fn init(name: &String) -> io::Result<()> {
    let root = Path::new(&name);
    let src = root.join("src");
    let target = root.join("target");
    let main_file = src.join("main.kurai");
    let config_file = root.join("kuros.toml");

    // Create all necessary dirs
    fs::create_dir_all(&src)?;    // This makes both `name/` and `name/src/`
    fs::create_dir_all(&target)?; // Will not fail if already exists

    // Write to main.kurai (create or overwrite)
    let mut file = File::create(&main_file)?;
    writeln!(
        file,
        r#"fn main() {{
    printf("Hello there!");
}}"#
    )?;

    let mut file2 = File::create(&config_file)?;

    // For the kuros.toml file
    let newline = |f: &mut File| writeln!(f, "\n").unwrap();
    writeln!(file2, "[package]")?;
    writeln!(file2, "name = \"{}\"", root.to_str().unwrap())?;
    writeln!(file2, "version = \"0.1.0\"")?;
    newline(&mut file2);

    writeln!(file2, "[profile]")?;
    writeln!(file2, "opt-level = 3")?;
    newline(&mut file2);

    writeln!(file2, "[build]")?;
    writeln!(file2, "external_libs = []")?;
    writeln!(file2, "external_libs_paths = []")?;
    newline(&mut file2);

    println!("Project '{}' initialized successfully!", name);
    Ok(())
}
