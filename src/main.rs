mod categories;
mod input;

use std::error::Error;
use std::fs::OpenOptions;
use std::io;

use csv;

use input::time_input;

fn append_data(file: &str, data: &Vec<String>) -> Result<(), Box<dyn Error>> {
    let file = OpenOptions::new().create(true).append(true).open(file)?;

    let mut writer = csv::WriterBuilder::new().from_writer(io::BufWriter::new(file));
    writer.write_record(data)?;
    writer.flush()?;
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let date = time_input()?;
    println!("Date: {}", date);
    let food_data = categories::category_food()?;
    let mut data: Vec<String> = vec![date];
    data.extend(food_data);

    append_data("data.csv", &data)?;

    Ok(())
}
