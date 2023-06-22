use std::collections::HashMap;
use std::error::Error;
use std::str::FromStr;

use serde_json;
use strsim::levenshtein;

use crate::data;
use crate::input::get_input;

pub fn get_data(category: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let category: String = category.to_lowercase();
    let categories: HashMap<&str, fn() -> Result<Vec<String>, Box<dyn Error>>> = HashMap::from([(
        "food",
        log_food as fn() -> Result<Vec<String>, Box<dyn Error>>,
    )]);

    let mut closest_key: Option<&str> = None;
    let mut closest_distance = usize::max_value();

    for key in categories.keys() {
        let distance = levenshtein(key, &category);
        if distance < closest_distance {
            closest_key = Some(key);
            closest_distance = distance;
        }
    }

    if let Some(key) = closest_key {
        println!("FOUND: {}", key);
        categories[key]()
    } else {
        println!("The following categories are valid:\n");
        for key in categories.keys() {
            print!("{}, ", key);
        }
        panic!("Couldn't find category {}!\nAborting...", category);
    }
}

fn data_from_file(category: &str) -> Result<Vec<String>, Box<dyn Error>> {
    fn get_value<T: FromStr>(value_str: &str) -> Option<T>
    where
        T::Err: Error + 'static,
    {
        if value_str.to_lowercase() == "none" {
            return None;
        }

        match value_str.parse::<T>() {
            Ok(value) => Some(value),
            Err(_) => None,
        }
    }
    fn process_value(
        type_str: &str,
        help_msg: &str,
        default_value: &str,
    ) -> Result<String, Box<dyn Error>> {
        let value: String = match type_str {
            "u32" => get_input::<u32>(help_msg, get_value::<u32>(default_value))?.to_string(),
            "i32" => get_input::<i32>(help_msg, get_value::<i32>(default_value))?.to_string(),
            _ => String::from("Unknown type"),
        };
        Ok(value)
    }

    let json_data: serde_json::Value = data::json_data(category)?;

    let data_types: Vec<String> = serde_json::from_value(json_data["type"].clone())?;
    let help_msgs: Vec<String> = serde_json::from_value(json_data["help"].clone())?;
    let default_values: Vec<String> = serde_json::from_value(json_data["value"].clone())?;

    let mut data: Vec<String> = Vec::new();
    for i in 0..data_types.len() {
        let value = process_value(&data_types[i], &help_msgs[i], &default_values[i])?;
        data.push(value);
    }
    Ok(data)
}

pub fn log_food() -> Result<Vec<String>, Box<dyn Error>> {
    let mut data: Vec<String> = vec![];
    let amount: u32 = get_input("[1=Low, 2=Medium, 3=High]\nAmount eaten", None)?;
    let speed: u32 = get_input("[1=Slow, 2=Medium, 3=Fast]\nEating speed", None)?;
    let drink_afterwards: bool = get_input("Drink afterwards", Some(true))?;
    let info: String = get_input("Food detail info", None)?;

    data.extend(vec![
        amount.to_string(),
        speed.to_string(),
        drink_afterwards.to_string(),
        info,
    ]);
    Ok(data)
}
