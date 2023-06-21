mod categories;
mod data;
mod input;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let category = input::category_input()?;
    let category_data: Vec<String> = categories::get_data(&category)?;
    let date: String = input::time_input()?;

    let mut data: Vec<String> = vec![date];
    data.extend(category_data);

    data::add_data("data.csv", data)?;
    Ok(())
}
