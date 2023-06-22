mod categories;
mod data;
mod input;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let raw_category = input::category_input()?;
    let category: String = categories::category(&raw_category)?;
    let date: String = input::time_input()?;

    let mut data: Vec<String> = vec![date];
    data.extend(categories::data_from_category(&category)?);
    data::add_data(&category, data)?;

    Ok(())
}
