use time::{Date, Month};

pub fn date_from_arg(arg: &str) -> Date {
    let split = arg
        .split('-')
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    generate_date(split[0], split[1], split[2])
}

pub fn generate_date(day: i32, month: i32, year: i32) -> Date {
    Date::from_calendar_date(year, Month::try_from(month as u8).unwrap(), day as u8).unwrap()
}

pub fn rrmmdd_from_date(birthday: Date) -> String {
    format!(
        "{:02}{:02}{:02}",
        birthday.year() % 100,
        get_pesel_birth_month(&birthday),
        birthday.day()
    )
}

fn get_pesel_birth_month(birthday: &Date) -> u8 {
    match birthday.year() {
        1800..=1899 => {
            let m: u8 = birthday.month().try_into().unwrap();
            m + 80
        }
        1900..=1999 => birthday.month().try_into().unwrap(),
        2000..=2099 => {
            let m: u8 = birthday.month().try_into().unwrap();
            m + 20
        }
        2100..=2199 => {
            let m: u8 = birthday.month().try_into().unwrap();
            m + 40
        }
        2200..=2299 => {
            let m: u8 = birthday.month().try_into().unwrap();
            m + 60
        }
        _ => panic!(
            "PESEL is only defined for years 1800-2299. Given year: {}",
            birthday.year()
        ),
    }
}
