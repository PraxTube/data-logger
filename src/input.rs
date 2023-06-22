use std::any::type_name;
use std::error::Error;
use std::io;
use std::str::FromStr;

use chrono::prelude::*;
use chrono::{DateTime, Local};

pub fn get_input<T: FromStr>(msg: &str, default_value: Option<T>) -> Result<T, Box<dyn Error>>
where
    T::Err: Error + 'static,
    T: std::fmt::Display,
{
    loop {
        match &default_value {
            Some(value) => println!("{} ({}, empty = {}): ", msg, type_name::<T>(), value),
            None => println!("{} ({}): ", msg, std::any::type_name::<T>(),),
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

pub fn time_input() -> Result<String, Box<dyn Error>> {
    let date: DateTime<Local> = Local::now()
        .with_nanosecond(0)
        .expect("Failed to set nano second to zero.")
        .with_second(0)
        .expect("Failed to set second to zero.");

    let year: i32 = get_input("Year", Some(date.year()))?;
    let month: u32 = get_input("Month", Some(date.month()))?;
    let day: u32 = get_input("Day", Some(date.day()))?;
    let hour: u32 = get_input("Hour", Some(date.hour()))?;
    let minute: u32 = get_input("Minute", Some(date.minute()))?;
    let date = date
        .with_year(year)
        .expect("Invalid range for year.")
        .with_month(month)
        .expect("Invalid range for month.")
        .with_day(day)
        .expect("Invalid range for day.")
        .with_hour(hour)
        .expect("Invalid range for hour.")
        .with_minute(minute)
        .expect("Invalid range for minute.");
    Ok(date.to_string())
}

pub fn category_input(keys: Vec<String>) -> Result<String, Box<dyn Error>> {
    println!("Log Keys:");
    for key in keys {
        print!("{}, ", key);
    }
    println!("\n");
    let category: String = get_input("Which category to log?", None)?;
    Ok(category)
}
