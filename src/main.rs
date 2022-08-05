use rayon::prelude::*;
use std::collections::HashMap;

type Result<T> = std::result::Result<T, Box<dyn ::std::error::Error>>;

fn main() -> Result<()> {
    let files: Vec<_> = std::fs::read_dir(".")?
        .map(|x| x.unwrap().path())
        .filter(|x| x.is_file())
        .map(|x| x.to_string_lossy().to_string())
        .collect();
    for filenames in duplicate_files(files) {
        println!("Duplicates of {}", filenames[0]);
        for fname in filenames[1..].iter() {
            println!("    {}", fname);
        }
    }

    Ok(())
}

fn duplicate_files(paths: Vec<String>) -> Vec<Vec<String>> {
    let mut md5s: HashMap<String, Vec<String>> = HashMap::new();
    let md5d_paths: Vec<_> = paths
        .par_iter()
        .map(|filename| match md5_of_file(filename) {
            Ok(hash) => (hash, filename.clone()),
            _ => (String::new(), String::new()),
        })
        .filter(|(a, _)| *a != String::new())
        .collect();

    for (check, filename) in md5d_paths {
        if let Some(val) = md5s.get_mut(&check) {
            val.push(filename);
        } else {
            md5s.insert(check, vec![filename]);
        }
    }

    md5s.values()
        .filter(|x| x.len() > 1)
        .map(|x| x.to_owned())
        .collect()
}

fn md5_of_file(path: &String) -> Result<String> {
    let contents = std::fs::read(path)?;
    Ok(format!("{:x}", md5::compute(contents)))
}
