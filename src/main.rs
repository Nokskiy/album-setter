use std::{
    env::args,
    fs,
    path::{Path, PathBuf},
};

use lofty::{
    config::ParseOptions,
    file::TaggedFileExt,
    probe::Probe,
    tag::{Accessor, TagExt},
};

fn main() {
    let args: Vec<String> = args().collect();
    if let Some(dir_path) = args.get(1) {
        if let Ok(_) = fs::read_dir(&dir_path) {
            let name = Path::new(&dir_path).file_name().unwrap().to_str().unwrap();
            let paths = recursive(&dir_path);
            for path in paths {
                println!("{}", path.to_str().unwrap());
                change(path.to_str().unwrap(), &name).unwrap_or_else(|err| println!("{}", err));
            }
        } else {
            panic!("Invalid path");
        }
    } else {
        panic!("Please, enter path");
    }
}

fn recursive<P: AsRef<str>>(path: P) -> Vec<PathBuf> {
    let mut result = vec![];

    let f = fs::read_dir(path.as_ref()).unwrap();
    for entry in f {
        let entry = entry.unwrap();
        let p = entry.path();
        if p.is_dir() {
            result.append(&mut recursive(p.to_str().unwrap()));
        } else {
            result.push(p);
        }
    }

    result
}

fn change<P: AsRef<str>>(path: P, album_name: P) -> Result<(), String> {
    let options = ParseOptions::new().parsing_mode(lofty::config::ParsingMode::Relaxed);
    let mut tagged_file = Probe::open(path.as_ref())
        .map_err(|err| err.to_string())?
        .options(options)
        .read()
        .map_err(|err| err.to_string())?;
    let tag = tagged_file.primary_tag_mut().unwrap();
    tag.set_album(album_name.as_ref().to_string());

    let mut file = std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .open(&path.as_ref())
        .map_err(|err| err.to_string())?;

    tag.save_to(&mut file, lofty::config::WriteOptions::default())
        .map_err(|err| err.to_string())?;
    Ok(())
}
