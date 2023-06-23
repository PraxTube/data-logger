use std::error::Error;
use std::str::FromStr;

use serde_json;
use strsim::levenshtein;

use crate::data;
use crate::input::get_input;

pub fn category(category: &str) -> Result<String, Box<dyn Error>> {
    let category: String = category.to_lowercase();
    let categories: Vec<String> = data::data_keys()?;

    let mut closest_key: Option<&str> = None;
    let mut closest_distance = usize::max_value();

    for key in &categories {
        let distance = levenshtein(&key, &category);
        if distance < closest_distance {
            closest_key = Some(&key);
            closest_distance = distance;
        }
    }

    if let Some(key) = closest_key {
        println!("FOUND: {}", key);
        Ok(key.to_string())
    } else {
        println!("The following categories are valid:\n");
        for key in categories {
            print!("{}, ", &key);
        }
        panic!("Couldn't find category {}!\nAborting...", category);
    }
}

fn stylize_string(input_str: &str) -> String {
    fn hex_to_asni_str(hex_str: &str) -> String {
        if let Ok(color) = u32::from_str_radix(&hex_str.replace("#", ""), 16) {
            let red = (color >> 16) & 0xFF;
            let green = (color >> 8) & 0xFF;
            let blue = color & 0xFF;

            return format!("{};{};{}", red, green, blue);
        }
        "FORMAT ERROR".to_string()
    }

    let mut output_str: String = input_str
        .replace("[bold]", "\x1b[1m")
        .replace("[/bold]", "\x1b[22m")
        .replace("[italic]", "\x1b[3m")
        .replace("[/italic]", "\x1b[23m")
        .replace("[underline]", "\x1b[4m")
        .replace("[/underline]", "\x1b[24m")
        .replace("[blink]", "\x1b[5m")
        .replace("[/blink]", "\x1b[25m")
        .replace("[invert]", "\x1b[7m")
        .replace("[/invert]", "\x1b[27m")
        .replace("[crossout]", "\x1b[9m")
        .replace("[crossout]", "\x1b[29m")
        .replace("[strike]", "\x1b[9m")
        .replace("[/strike]", "\x1b[29m")
        .replace("[/color]", "\x1b[39m");

    let start_tag = "[color=";
    let end_tag = "]";
    let mut current_index = 0;

    while let Some(start_index) = output_str[current_index..].find(start_tag) {
        let adjusted_start_index = current_index + start_index + start_tag.len();

        if let Some(end_index) = output_str[adjusted_start_index..].find(end_tag) {
            let hex_str = &output_str[adjusted_start_index..(adjusted_start_index + end_index)];
            output_str = output_str.replace(
                &format!("{}{}{}", start_tag, hex_str, end_tag),
                &format!("\x1b[38;2;{}m", hex_to_asni_str(hex_str)),
            );
            current_index = adjusted_start_index + end_index;
        } else {
            break;
        }
    }
    output_str
}

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
    let type_str: &str = &stylize_string(type_str);
    let help_msg: &str = &stylize_string(help_msg);
    let default_value: &str = &stylize_string(default_value);

    let value: String = match type_str {
        "u32" => get_input::<u32>(help_msg, get_value::<u32>(default_value))?.to_string(),
        "i32" => get_input::<i32>(help_msg, get_value::<i32>(default_value))?.to_string(),
        "f32" => get_input::<f32>(help_msg, get_value::<f32>(default_value))?.to_string(),
        "bool" => get_input::<bool>(help_msg, get_value::<bool>(default_value))?.to_string(),
        "String" => get_input::<String>(help_msg, get_value::<String>(default_value))?.to_string(),
        _ => "None".to_string(),
    };

    if value == "None" {
        return Err(format!("Not a valid type: {}", type_str).into());
    }
    Ok(value)
}

pub fn data_from_category(category: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let json_data: serde_json::Value = data::category_json_data(category)?;

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
