use std::error::Error;

use crate::input::get_input;

pub fn category_food() -> Result<Vec<String>, Box<dyn Error>> {
    let mut data: Vec<String> = vec![];
    let amount: u32 = get_input("[1=Low, 2=Medium, 3=High]\nAmount eaten", None)?;
    data.push(amount.to_string());
    Ok(data)
}
