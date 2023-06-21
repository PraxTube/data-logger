use std::collections::HashMap;
use std::error::Error;

use strsim::levenshtein;

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

pub fn log_food() -> Result<Vec<String>, Box<dyn Error>> {
    let mut data: Vec<String> = vec![];
    let amount: u32 = get_input("[1=Low, 2=Medium, 3=High]\nAmount eaten", None)?;
    data.push(amount.to_string());
    Ok(data)
}
