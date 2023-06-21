mod categories;
mod input;

use std::error::Error;
use std::fs::OpenOptions;
use std::io;

use csv;

fn append_data(file: &str, data: Vec<String>) -> Result<(), Box<dyn Error>> {
    let file = OpenOptions::new().create(true).append(true).open(file)?;

    let mut writer = csv::WriterBuilder::new().from_writer(io::BufWriter::new(file));
    writer.write_record(data)?;
    writer.flush()?;
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let category = input::category_input()?;
    let category_data: Vec<String> = categories::get_data(&category)?;
    let date: String = input::time_input()?;

    let mut data: Vec<String> = vec![date];
    data.extend(category_data);

    append_data("data.csv", data)?;

    Ok(())
}
