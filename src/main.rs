use std::fs;
use std::fs::File;
use std::io::prelude::*;

use walkdir::WalkDir;

fn main() {

    match traverse() {
        Ok(()) => std::process::exit(0),
        Err(err) => {
            eprintln!("{}", err);

            std::thread::sleep(std::time::Duration::from_millis(10000));
            std::process::exit(2);
        }
    }
}


pub fn traverse() -> Result<(),std::io::Error> {
    let directory: &'static str = "C:\\";

    let mut file = File::create("c.txt")?;

    for entry in WalkDir::new(directory).into_iter().filter_map(|e| e.ok()) {
        //println!("path: \"{}\", size: {}", entry.path().display(), fs::metadata(entry.path()).unwrap().len());
        let line = format!("path: \"{}\", size: {}\n", entry.path().display(), fs::metadata(entry.path())?.len());
        file.write_all(line.as_bytes())?;
    }
    Ok(())
} 

/*pub fn traverse_folder<P>(path: P) -> Result<Vec<PathBuf>, std::io::Error>
    where P: AsRef<Path> {
        fs::read_dir(path)?
        .into_iter()
        .map(|x| x.map(|entry| entry.path()))
        .collect()
    }

    */