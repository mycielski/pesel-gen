use crate::checksum::calculate_checksum;
use crate::date;
use time::Date;

pub fn generate_pesel_numbers(birthday: Date, gender: char) {
    let g = match gender {
        'f' => 0,
        'm' => 1,
        _ => panic!("Only 'f' and 'm' are allowed as gender. Given: {}", gender),
    };

    let rrmmdd = date::rrmmdd_from_date(birthday);

    for value in (g..10).step_by(2) {
        for ppp in 000..=999 {
            let last_digit = calculate_checksum(format!("{:06}{:03}{:01}", rrmmdd, ppp, value));
            println!("{:06}{:03}{:01}{:01}", rrmmdd, ppp, value, last_digit);
        }
    }
}
