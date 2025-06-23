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
    writeln!(file2, "[package]")?;
    writeln!(file2, "name = \"{}\"", root.to_str().unwrap())?;
    writeln!(file2, "version = \"0.1.0\"")?;

    writeln!(file2, "[language]")?;
    writeln!(file2, "for_style = \"range\" # range, classic, or both")?;
    writeln!(file2, "bm_mode = true")?;
    writeln!(file2, "experimental_features = [] # we can just omit this fr")?;

    println!("Project '{}' initialized successfully!", name);
    Ok(())
}
