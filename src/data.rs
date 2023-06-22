use std::error::Error;
use std::fs::File;
use std::fs::OpenOptions;
use std::io;
use std::io::Read;
use std::path::PathBuf;

use csv;
use serde_json;

fn data_dir() -> Result<PathBuf, Box<dyn Error>> {
    let home_dir = match std::env::var("HOME") {
        Ok(dir) => PathBuf::from(dir),
        Err(_) => {
            return Err("Unable to determine user's home directory".into());
        }
    };

    let data_dir = home_dir.join(".config").join("dogg").join("data");

    if !data_dir.exists() {
        std::fs::create_dir_all(&data_dir)?;
    }
    Ok(data_dir)
}

pub fn add_data(category: &str, data: Vec<String>) -> Result<(), Box<dyn Error>> {
    let data_file = data_dir()?.join(category);
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(data_file)?;

    let mut writer = csv::WriterBuilder::new().from_writer(io::BufWriter::new(file));
    writer.write_record(data)?;
    writer.flush()?;
    Ok(())
}

pub fn json_data(category: &str) -> Result<serde_json::Value, Box<dyn Error>> {
    let path: PathBuf = data_dir()?.join(category);
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let json_data: serde_json::Value = serde_json::from_str(&contents)?;
    Ok(json_data)
}

pub fn data_files() -> Result<Vec<String>, Box<dyn Error>> {
    let mut files = Vec::new();

    for entry in std::fs::read_dir(data_dir()?)? {
        let entry = entry?;
        let path = entry.path();

        if !path.is_file() {
            continue;
        }

        if let Some(file_name) = path.file_name() {
            if let Some(file_name_str_csv) = file_name.to_str() {
                if let Some(file_name_str) = file_name_str_csv.strip_suffix(".csv") {
                    files.push(file_name_str.to_string());
                }
            }
        }
    }
    Ok(files)
}
