use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;

pub fn init(name: &String) -> io::Result<()> {
    let root = Path::new(&name);
    let src = root.join("src");
    let target = root.join("target");
    let main_file = src.join("main.kurai");

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

    println!("Project '{}' initialized successfully!", name);
    Ok(())
}
