use std::{
    env,
    error::Error,
    fs::File,
    io::{BufWriter, Write},
    path::Path,
};
use rayon::prelude::*;
use walkdir::WalkDir;

fn main() -> Result<(), Box<dyn Error>> {
    let output_path = Path::new("./tree.csv");

    let current_exe = env::current_exe()?;

    let walker = WalkDir::new(".")
        .into_iter()
        .filter_entry(|e| {
            let path = e.path();
            path != output_path && path != current_exe
        });

    let mut file_data: Vec<(String, u64)> = walker
        .par_bridge()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
        .filter_map(|entry| {
            let metadata = entry.metadata().ok()?;
            let path_str = entry.path().to_string_lossy().replace('\\', "/");
            let normalized_path = if path_str.starts_with("./") {
                format!("/{}", &path_str[2..])
            } else if path_str == "." {
                "/".to_string()
            } else {
                format!("/{}", path_str)
            };
            Some((normalized_path, metadata.len()))
        })
        .collect();

    // Sort by file name
    file_data.sort_by(|a, b| a.0.cmp(&b.0));

    let file = File::create(output_path)?;
    let mut writer = BufWriter::new(file);
    for (path, size) in file_data {
        writer.write_all(format!("{},{}\n", path, size).as_bytes())?;
    }

    Ok(())
}