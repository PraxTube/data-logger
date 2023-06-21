use std::error::Error;
use std::fs::OpenOptions;
use std::io;
use std::path::PathBuf;

use csv;

fn data_dir() -> Result<PathBuf, Box<dyn Error>> {
    // Get the user's home directory
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
