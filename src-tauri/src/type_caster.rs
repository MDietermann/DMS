pub fn string_to_u8(convert_string: String) -> Result<u8, String> {
    match convert_string.parse::<u8>() {
        Ok(num) => Ok(num),
        Err(_) => Err(format!("Failed to convert '{}' to u8", convert_string))
    }
}
