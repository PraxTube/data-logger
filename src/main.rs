use std::error::Error;
use std::fs::OpenOptions;
use std::io;
use std::str::FromStr;

use chrono::prelude::*;
use chrono::{DateTime, Local};
use csv;

fn get_input<T: FromStr>(msg: &str, default_value: Option<T>) -> Result<T, Box<dyn Error>>
where
    T::Err: Error + 'static,
    T: std::fmt::Display,
{
    loop {
        match &default_value {
            Some(value) => {
                println!(
                    "{} ({}, empty = {}): ",
                    msg,
                    std::any::type_name::<T>(),
                    value
                );
            }
            None => {
                println!("{} ({}): ", msg, std::any::type_name::<T>(),);
            }
        }
        let mut line = String::new();
        io::stdin().read_line(&mut line)?;

        if line.trim().is_empty() {
            if let Some(value) = default_value {
                return Ok(value);
            }
        }

        match line.trim().parse::<T>() {
            Ok(value) => return Ok(value),
            Err(_) => println!("Invalid input. Please try again."),
        }
    }
}

fn append_data(file: &str, data: &[String]) -> Result<(), Box<dyn Error>> {
    let file = OpenOptions::new().create(true).append(true).open(file)?;

    let mut writer = csv::WriterBuilder::new().from_writer(io::BufWriter::new(file));
    writer.write_record(data)?;
    writer.flush()?;
    Ok(())
}

fn current_datetime() -> DateTime<Local> {
    let current_time: DateTime<Local> = Local::now()
        .with_nanosecond(0)
        .expect("Failed to set nano second to zero.")
        .with_second(0)
        .expect("Failed to set second to zero.");
    current_time
}

fn time_input() -> Result<String, Box<dyn Error>> {
    let date = current_datetime();
    let year: i32 = get_input("Year", Some(date.year()))?;
    let month: u32 = get_input("Month", Some(date.month()))?;
    let day: u32 = get_input("Day", Some(date.day()))?;
    let date = date
        .with_year(year)
        .expect("Invalid range for year.")
        .with_month(month)
        .expect("Invalid range for month.")
        .with_day(day)
        .expect("Invalid range for day.");
    Ok(date.to_string())
}

fn main() -> Result<(), Box<dyn Error>> {
    let date = time_input()?;
    println!("Date: {}", date);
    let time: f32 = get_input("Time of day", None)?;
    println!("Time: {}", time);
    let info: String = get_input("Info", None)?;
    let data = [date, time.to_string(), info];

    append_data("data.csv", &data)?;

    Ok(())
}
