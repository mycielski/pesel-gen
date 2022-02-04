use crate::checksum::calculate_checksum;

pub fn validate_pesel(pesel: String) -> bool {
    // saves the last digit of the pesel to be compared with the calculated checksum
    let expected_checksum =
        i32::try_from(pesel.chars().last().unwrap().to_digit(10).unwrap()).unwrap();

    let calculated_checksum = calculate_checksum(pesel);

    if expected_checksum != calculated_checksum {
        return false;
    }
    true
}

#[allow(dead_code)]
fn format_then_validate_pesel(pesel: String) -> bool {
    // Removes whitespaces, checks if pesel is correct length and if it contains only digits. If not, returns false.
    let pesel: String = pesel.chars().filter(|c| !c.is_whitespace()).collect();

    if pesel.len() != 11 || pesel.chars().map(|c| c.is_numeric()).any(|x| !x) {
        return false;
    }
    validate_pesel(pesel)
}
