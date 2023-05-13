use std::fs;
use std::io;

mod cli;

struct Settings {
    name: String,
}

fn main() {
    init(Settings {
        name: String::from("romeo"),
    });
}

fn init(settings: Settings) -> Result<(), io::Error> {
    fs::create_dir(settings.name.clone())?;
    let paths = vec!["objects", "refs"];

    for path in paths.iter() {
        fs::create_dir_all(format!("{}/.git/{}", settings.name, path))?;
    }

    Ok(())
}
