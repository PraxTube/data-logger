mod category;
mod data;
mod input;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut sorted_keys = data::data_keys()?;
    sorted_keys.sort();
    let raw_category = input::category_input(sorted_keys)?;
    let category: String = category::category(&raw_category)?;
    let date: String = input::time_input()?;

    let mut data: Vec<String> = vec![date];
    data.extend(category::data_from_category(&category)?);
    data::add_data(&category, data)?;

    Ok(())
}
