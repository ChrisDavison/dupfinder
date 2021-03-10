use std::collections::HashMap;
use std::path::{Path, PathBuf};

type Result<T> = std::result::Result<T, Box<dyn ::std::error::Error>>;

fn main() -> Result<()> {
    let files: Vec<PathBuf> = std::fs::read_dir(".")?
        .map(|x| x.unwrap().path())
        .filter(|x| x.is_file())
        .collect();
    for filenames in duplicate_files(files) {
        println!("Duplicates of {}", filenames[0]);
        for fname in filenames[1..].iter() {
            println!("    {}", fname);
        }
    }

    Ok(())
}

fn duplicate_files(paths: Vec<PathBuf>) -> Vec<Vec<String>> {
    let mut md5s: HashMap<String, Vec<String>> = HashMap::new();
    for filename in paths {
        if let Ok(hash) = md5_of_file(&filename) {
            if let Some(val) = md5s.get_mut(&hash) {
                val.push(filename.to_string_lossy().into());
            } else {
                md5s.insert(hash, vec![filename.to_string_lossy().into()]);
            }
        }
    }
    md5s.values()
        .filter(|x| x.len() > 1)
        .map(|x| x.to_owned())
        .collect()
}

fn md5_of_file(path: &Path) -> Result<String> {
    let contents = std::fs::read_to_string(path)?;
    Ok(format!("{:x}", md5::compute(contents)))
}
