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
    let data_file = data_dir()?.join(format!("{}.csv", category));
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(data_file)?;

    let mut writer = csv::WriterBuilder::new().from_writer(io::BufWriter::new(file));
    writer.write_record(data)?;
    writer.flush()?;
    Ok(())
}

fn json_data() -> Result<serde_json::Value, Box<dyn Error>> {
    let path: PathBuf = data_dir()?.join("config.json");
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let json_data: serde_json::Value = serde_json::from_str(&contents)?;
    Ok(json_data)
}

pub fn category_json_data(category: &str) -> Result<serde_json::Value, Box<dyn Error>> {
    let json_data = json_data()?;
    let data = json_data[category].clone();
    Ok(data)
}

pub fn data_keys() -> Result<Vec<String>, Box<dyn Error>> {
    let json_data = json_data()?;
    let keys: Vec<String> = json_data
        .as_object()
        .ok_or("Not a valid dict.")?
        .keys()
        .cloned()
        .collect();
    Ok(keys)
}
